// src/main.rs
mod achievements; // On importe notre nouveau fichier

use notify_rust::Notification;
use std::fs;
use std::path::Path;
use chrono::Timelike;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { return; }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let storage_path = format!("{}/.arch_achievements", home);
    let xp_file = format!("{}/total_xp", storage_path);

    // --- LOGIQUE DE RESET ---
    if args[1] == "-reset" {
        if let Ok(_) = fs::remove_dir_all(&storage_path) {
            let _ = fs::create_dir_all(&storage_path);
            let _ = fs::write(&xp_file, "0");
            println!("All succes are reset");
        }
        return;
    }

    if args.len() < 3 { return; }
    let _ = fs::create_dir_all(&storage_path);

    check_passive_achievements(&storage_path, &xp_file);

    if let Some((name, xp)) = achievements::check(&args[2], args.get(3).unwrap_or(&String::new()), &storage_path) {
        unlock_achievement(&storage_path, &xp_file, name, xp);

        check_platinum(&storage_path, &xp_file);
    }
}

pub fn increment_and_get(path: &str, key: &str) -> i32 {
    let p = format!("{}/{}.count", path, key);
    let c = fs::read_to_string(&p).unwrap_or("0".into()).trim().parse::<i32>().unwrap_or(0) + 1;
    let _ = fs::write(p, c.to_string());
    c
}

fn check_passive_achievements(path: &str, xp_p: &str) {
    let hour = chrono::Local::now().hour();
    if hour < 5 { unlock_achievement(path, xp_p, "Night Owl", 200); }

    if let Ok(temp) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        if temp.trim().parse::<i32>().unwrap_or(0) / 1000 > 80 {
            unlock_achievement(path, xp_p, "Overclocked Heart", 300);
        }
    }
}

fn check_platinum(path: &str, xp_p: &str) {
    let list = achievements::get_list();
    let mut unlocked = 0;
    for ach in &list {
        if Path::new(&format!("{}/{}.lock", path, ach.replace(" ", "_"))).exists() {
            unlocked += 1;
        }
    }
    if unlocked == list.len() {
        unlock_achievement(path, xp_p, "arch user BTW", 5000);
    }
}

fn unlock_achievement(path: &str, xp_p: &str, name: &str, gain: i32) {
    let lock = format!("{}/{}.lock", path, name.replace(" ", "_"));
    if !Path::new(&lock).exists() {
        let _ = fs::File::create(&lock);
        let cur = fs::read_to_string(xp_p).unwrap_or("0".into()).trim().parse::<i32>().unwrap_or(0);
        let _ = fs::write(xp_p, (cur + gain).to_string());
        Notification::new().summary("🏆 New Achievement Unlocked").body(&format!("{}\n+{} XP", name, gain)).show().ok();
    }
}
