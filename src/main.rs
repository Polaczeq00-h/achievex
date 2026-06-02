// src/main.rs
mod achievements;
mod pet;

use chrono::Timelike;
use notify_rust::Notification;
use std::fs::{self, OpenOptions};
use std::io::ErrorKind;
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const SCAN_THROTTLE_SECONDS: u64 = 300;
const EVENT_THROTTLE_SECONDS: u64 = 30;

enum UnlockResult {
    Unlocked,
    AlreadyUnlocked,
    Failed,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let storage_path = format!("{}/.arch_achievements", home);
    let xp_file = format!("{}/total_xp", storage_path);
    let _ = fs::create_dir_all(&storage_path);
    ensure_xp_file(&xp_file);

    if args.len() < 2 {
        print_profile(&storage_path, &xp_file);
        return;
    }

    if args[1] == "-reset" {
        reset_progress(&storage_path, &xp_file);
        return;
    }

    match args[1].as_str() {
        "profile" => {
            print_profile(&storage_path, &xp_file);
            return;
        }
        "stats" => {
            print_stats(&storage_path, &xp_file);
            return;
        }
        "list" => {
            print_achievement_list(&storage_path);
            return;
        }
        "pets" => {
            print_pets(read_xp(&xp_file));
            return;
        }
        "pet" => {
            handle_pet_command(&storage_path, &xp_file, args.get(2));
            return;
        }
        "scan" => {
            if !should_run(&storage_path, "scan", SCAN_THROTTLE_SECONDS) {
                println!("Scan throttled. Try again in a moment.");
                return;
            }
            run_system_scan(&storage_path, &xp_file);
            check_platinum(&storage_path, &xp_file);
            print_profile(&storage_path, &xp_file);
            return;
        }
        "event" => {
            handle_event(&storage_path, &xp_file, &args);
            return;
        }
        "xp" => {
            println!("{}", read_xp(&xp_file));
            return;
        }
        "reset" => {
            reset_progress(&storage_path, &xp_file);
            return;
        }
        "debug" => {
            print_debug(&storage_path, &xp_file);
            return;
        }
        "export" => {
            print_export(&storage_path, &xp_file);
            return;
        }
        "daemon" => {
            run_daemon(&storage_path, &xp_file, args.get(2));
            return;
        }
        "overlay" => {
            launch_overlay();
            return;
        }
        "help" | "-h" | "--help" => {
            print_help();
            return;
        }
        "trigger" => {}
        _ => {
            print_help();
            return;
        }
    }

    if args.len() < 3 {
        return;
    }

    if should_run(&storage_path, "passive", SCAN_THROTTLE_SECONDS) {
        check_passive_achievements(&storage_path, &xp_file);
    }

    if let Some((name, xp)) = achievements::check(
        &args[2],
        args.get(3).unwrap_or(&String::new()),
        &storage_path,
    ) {
        unlock_achievement(&storage_path, &xp_file, name, xp);

        check_platinum(&storage_path, &xp_file);
    }
}

fn run_daemon(storage_path: &str, xp_file: &str, interval_arg: Option<&String>) {
    let Some(_guard) = DaemonGuard::acquire(storage_path) else {
        println!("Achievix daemon is already running.");
        return;
    };

    let interval = interval_arg
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(300)
        .max(30);

    println!("Achievix daemon started. Scan interval: {}s", interval);
    loop {
        if should_run(storage_path, "daemon_scan", SCAN_THROTTLE_SECONDS) {
            run_system_scan(storage_path, xp_file);
            check_platinum(storage_path, xp_file);
        }
        thread::sleep(Duration::from_secs(interval));
    }
}

fn now_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn should_run(storage_path: &str, key: &str, cooldown_seconds: u64) -> bool {
    let path = format!("{}/throttle_{}.stamp", storage_path, safe_key(key));
    let now = now_seconds();
    let last = fs::read_to_string(&path)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .unwrap_or(0);

    if now.saturating_sub(last) < cooldown_seconds {
        return false;
    }

    let _ = fs::write(path, now.to_string());
    true
}

struct DaemonGuard {
    path: String,
}

impl DaemonGuard {
    fn acquire(storage_path: &str) -> Option<Self> {
        let path = format!("{}/daemon.lock", storage_path);
        if let Ok(pid) = fs::read_to_string(&path) {
            if process_exists(pid.trim()) {
                return None;
            }
            let _ = fs::remove_file(&path);
        }

        let result = OpenOptions::new().write(true).create_new(true).open(&path);
        let Ok(mut file) = result else {
            return None;
        };

        use std::io::Write;
        let _ = writeln!(file, "{}", std::process::id());
        Some(Self { path })
    }
}

impl Drop for DaemonGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn process_exists(pid: &str) -> bool {
    !pid.is_empty()
        && pid.chars().all(|ch| ch.is_ascii_digit())
        && Path::new(&format!("/proc/{}", pid)).exists()
}

fn safe_key(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars().take(96) {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }

    if out.is_empty() { "event".into() } else { out }
}

fn ensure_xp_file(path: &str) {
    if !Path::new(path).exists() {
        let _ = fs::write(path, "0");
    }
}

fn read_xp(path: &str) -> i32 {
    fs::read_to_string(path)
        .unwrap_or_else(|_| "0".into())
        .trim()
        .parse::<i32>()
        .unwrap_or(0)
        .max(0)
}

pub fn increment_and_get(path: &str, key: &str) -> i32 {
    let p = format!("{}/{}.count", path, key);
    let c = fs::read_to_string(&p)
        .unwrap_or("0".into())
        .trim()
        .parse::<i32>()
        .unwrap_or(0)
        + 1;
    let _ = fs::write(p, c.to_string());
    c
}

fn increment_stat(path: &str, key: &str) {
    let _ = increment_and_get(path, key);
}

fn handle_event(storage_path: &str, xp_file: &str, args: &[String]) {
    if args.len() < 4 {
        println!("Usage: achievix event <type> <data> [detail]");
        return;
    }

    let event_type = args[2].as_str();
    let data = args[3].as_str();
    let detail = args.get(4).map(|value| value.as_str()).unwrap_or("");
    let throttle_key = format!("event_{}_{}", event_type, data);

    increment_stat(storage_path, "events_seen");

    if !should_run(storage_path, &throttle_key, EVENT_THROTTLE_SECONDS) {
        increment_stat(storage_path, "events_throttled");
        println!(
            "Event throttled: {} {}",
            short_display(event_type),
            short_display(data)
        );
        return;
    }

    if let Some((name, xp)) = achievements::check_event(event_type, data, detail) {
        let result = unlock_achievement(storage_path, xp_file, name, xp);
        check_platinum(storage_path, xp_file);
        if matches!(result, UnlockResult::Unlocked) {
            println!(
                "Event accepted: {} {} -> {}",
                short_display(event_type),
                short_display(data),
                name
            );
        } else if matches!(result, UnlockResult::AlreadyUnlocked) {
            println!(
                "Event accepted: {} {} -> already unlocked ({})",
                short_display(event_type),
                short_display(data),
                name
            );
        } else {
            println!(
                "Event accepted but unlock failed: {} {} -> {}",
                short_display(event_type),
                short_display(data),
                name
            );
        }
    } else {
        println!(
            "Event accepted: {} {}",
            short_display(event_type),
            short_display(data)
        );
    }
}

fn reset_progress(storage_path: &str, xp_file: &str) {
    let sound_path = format!("{}/achieve.mp3", storage_path);
    let custom_sound = fs::read(&sound_path).ok();

    let _ = fs::remove_dir_all(storage_path);
    let _ = fs::create_dir_all(storage_path);
    let _ = fs::write(xp_file, "0");

    if let Some(sound) = custom_sound {
        let _ = fs::write(sound_path, sound);
    }

    println!("All achievements are reset");
}

fn short_display(value: &str) -> String {
    const MAX_CHARS: usize = 80;
    let mut output: String = value.chars().take(MAX_CHARS).collect();
    if value.chars().count() > MAX_CHARS {
        output.push_str("...");
    }
    output
}

fn check_passive_achievements(path: &str, xp_p: &str) {
    let hour = chrono::Local::now().hour();
    if hour < 5 {
        unlock_achievement(path, xp_p, "Night Owl", 200);
    }

    if let Ok(temp) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        if temp.trim().parse::<i32>().unwrap_or(0) / 1000 > 80 {
            unlock_achievement(path, xp_p, "Overclocked Heart", 300);
        }
    }
}

fn run_system_scan(path: &str, xp_p: &str) {
    check_passive_achievements(path, xp_p);
    check_distro_achievements(path, xp_p);

    if Path::new("/etc/arch-release").exists() {
        unlock_achievement(path, xp_p, "I use Arch BTW", 100);
    }

    if command_exists("fish") {
        unlock_achievement(path, xp_p, "Fish Heretic", 100);
    }

    if command_exists("nvidia-smi") {
        unlock_achievement(path, xp_p, "NVIDIA Survivor", 150);
    }

    if command_exists("blender") {
        unlock_achievement(path, xp_p, "Blender Victim", 150);
    }

    if command_exists("steam") {
        unlock_achievement(path, xp_p, "Steam Enjoyer", 100);
    }

    if command_exists("flatpak") {
        unlock_achievement(path, xp_p, "Flatpak Citizen", 75);
    }

    if command_exists("asciiquarium") {
        unlock_achievement(path, xp_p, "Asciiquarium Marine Biologist", 150);
    }

    if command_exists("sl") {
        unlock_achievement(path, xp_p, "Train Passenger", 100);
    }

    if is_kde_session() {
        unlock_achievement(path, xp_p, "KDE Goblin", 100);
    }

    if uses_cachy_kernel() {
        unlock_achievement(path, xp_p, "Cachy Kernel Rider", 150);
    }
}

fn check_distro_achievements(path: &str, xp_p: &str) {
    let distro = DistroInfo::detect();

    if distro.is_arch_family() {
        unlock_achievement(path, xp_p, "Arch Family Resident", 100);
    }

    if distro.is_debian_family() {
        unlock_achievement(path, xp_p, "Debian Family Resident", 100);
    }

    if distro.is_fedora_family() {
        unlock_achievement(path, xp_p, "Fedora Family Resident", 100);
    }

    if distro.is_opensuse_family() {
        unlock_achievement(path, xp_p, "SUSE Family Resident", 100);
    }

    match distro.id.as_str() {
        "arch" => {
            unlock_achievement(path, xp_p, "Pure Arch Pilgrim", 150);
        }
        "garuda" => {
            unlock_achievement(path, xp_p, "Garuda Native", 150);
        }
        "cachyos" => {
            unlock_achievement(path, xp_p, "CachyOS Sprinter", 150);
        }
        "manjaro" => {
            unlock_achievement(path, xp_p, "Manjaro Mountaineer", 150);
        }
        "endeavouros" => {
            unlock_achievement(path, xp_p, "Endeavour Voyager", 150);
        }
        "ubuntu" => {
            unlock_achievement(path, xp_p, "Ubuntu Tourist", 150);
        }
        "debian" => {
            unlock_achievement(path, xp_p, "Debian Elder", 150);
        }
        "fedora" => {
            unlock_achievement(path, xp_p, "Fedora Hat Wearer", 150);
        }
        "opensuse-tumbleweed" | "opensuse-leap" | "opensuse" => {
            unlock_achievement(path, xp_p, "openSUSE Chameleon", 150);
        }
        "nixos" => {
            unlock_achievement(path, xp_p, "NixOS Time Wizard", 200);
        }
        _ => {}
    }
}

fn command_exists(command: &str) -> bool {
    std::env::var_os("PATH")
        .unwrap_or_default()
        .to_string_lossy()
        .split(':')
        .any(|dir| Path::new(dir).join(command).exists())
}

fn is_kde_session() -> bool {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
    let session = std::env::var("DESKTOP_SESSION").unwrap_or_default();
    desktop.to_lowercase().contains("kde") || session.to_lowercase().contains("plasma")
}

fn uses_cachy_kernel() -> bool {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or_default()
        .to_lowercase()
        .contains("cachy")
}

struct DistroInfo {
    id: String,
    id_like: Vec<String>,
}

impl DistroInfo {
    fn detect() -> Self {
        let os_release = fs::read_to_string("/etc/os-release").unwrap_or_default();
        let id = os_release_value(&os_release, "ID").unwrap_or_default();
        let id_like = os_release_value(&os_release, "ID_LIKE")
            .unwrap_or_default()
            .split_whitespace()
            .map(|value| value.to_lowercase())
            .collect();

        Self {
            id: id.to_lowercase(),
            id_like,
        }
    }

    fn is_arch_family(&self) -> bool {
        self.id == "arch" || self.id_like.iter().any(|value| value == "arch")
    }

    fn is_debian_family(&self) -> bool {
        self.id == "debian"
            || self.id == "ubuntu"
            || self.id_like.iter().any(|value| value == "debian")
    }

    fn is_fedora_family(&self) -> bool {
        self.id == "fedora" || self.id_like.iter().any(|value| value == "fedora")
    }

    fn is_opensuse_family(&self) -> bool {
        self.id.contains("opensuse")
            || self
                .id_like
                .iter()
                .any(|value| value == "suse" || value == "opensuse")
    }
}

fn os_release_value(contents: &str, key: &str) -> Option<String> {
    let prefix = format!("{}=", key);
    contents.lines().find_map(|line| {
        let value = line.strip_prefix(&prefix)?;
        Some(value.trim_matches('"').to_string())
    })
}

fn check_platinum(path: &str, xp_p: &str) {
    let list = achievements::get_list();
    let required: Vec<&str> = list
        .into_iter()
        .filter(|ach| *ach != "Platinum Goblin")
        .collect();

    let unlocked = required.iter().filter(|ach| is_unlocked(path, ach)).count();

    if unlocked == required.len() {
        unlock_achievement(path, xp_p, "Platinum Goblin", 5000);
    }
}

fn unlock_achievement(path: &str, xp_p: &str, name: &str, gain: i32) -> UnlockResult {
    let lock = format!("{}/{}.lock", path, name.replace(" ", "_"));
    let lock_result = OpenOptions::new().write(true).create_new(true).open(&lock);

    if let Err(error) = lock_result {
        return if error.kind() == ErrorKind::AlreadyExists || Path::new(&lock).exists() {
            UnlockResult::AlreadyUnlocked
        } else {
            UnlockResult::Failed
        };
    }

    let cur = read_xp(xp_p);
    let new_xp = cur.saturating_add(gain.max(0));
    if fs::write(xp_p, new_xp.to_string()).is_err() {
        let _ = fs::remove_file(lock);
        return UnlockResult::Failed;
    }

    Notification::new()
        .summary("🏆 New Achievement Unlocked")
        .body(&format!("{}\n+{} XP", name, gain))
        .show()
        .ok();
    play_achievement_sound(path);
    UnlockResult::Unlocked
}

fn play_achievement_sound(storage_path: &str) {
    let sound_path = format!("{}/achieve.mp3", storage_path);
    if !Path::new(&sound_path).exists() {
        return;
    }

    let players = [
        ("paplay", vec![sound_path.as_str()]),
        ("pw-play", vec![sound_path.as_str()]),
        (
            "mpv",
            vec!["--no-video", "--really-quiet", sound_path.as_str()],
        ),
        (
            "ffplay",
            vec![
                "-nodisp",
                "-autoexit",
                "-loglevel",
                "quiet",
                sound_path.as_str(),
            ],
        ),
    ];

    for (player, args) in players {
        if std::process::Command::new(player)
            .args(args)
            .spawn()
            .is_ok()
        {
            break;
        }
    }
}

fn is_unlocked(path: &str, name: &str) -> bool {
    Path::new(&format!("{}/{}.lock", path, name.replace(" ", "_"))).exists()
}

fn unlocked_count(path: &str) -> usize {
    achievements::get_list()
        .iter()
        .filter(|ach| is_unlocked(path, ach))
        .count()
}

fn level_for_xp(xp: i32) -> i32 {
    (xp / 250) + 1
}

fn next_level_xp(xp: i32) -> i32 {
    level_for_xp(xp) * 250
}

fn active_pet(xp: i32) -> &'static str {
    match xp {
        x if x >= 2000 => "☢️ Arch Spirit",
        x if x >= 1000 => "👾 Kernel Beast",
        x if x >= 600 => "🐉 Garuda Dragon",
        x if x >= 300 => "🦊 Terminal Fox",
        x if x >= 100 => "🐧 Baby Penguin",
        _ => "none",
    }
}

fn print_pet_ai(storage_path: &str, xp_file: &str) {
    let xp = read_xp(xp_file);
    let state = pet::detect(xp);
    let state_path = format!("{}/pet_state", storage_path);
    let _ = fs::write(
        state_path,
        format!(
            "{}|{}|{}|{}",
            state.name, state.mood, state.message, state.activity
        ),
    );

    println!("{} {}", state.icon, state.name);
    println!("Mood: {}", state.mood);
    println!("Activity: {}", state.activity);
    println!("{}", state.message);
}

fn handle_pet_command(storage_path: &str, xp_file: &str, subcommand: Option<&String>) {
    match subcommand.map(|value| value.as_str()).unwrap_or("status") {
        "status" => print_pet_ai(storage_path, xp_file),
        "feed" => feed_pet(storage_path, xp_file),
        "mood" => print_pet_mood(storage_path, xp_file),
        "evolve" => print_pet_evolution(xp_file),
        _ => {
            println!("Usage:");
            println!("  achievix pet status");
            println!("  achievix pet feed");
            println!("  achievix pet mood");
            println!("  achievix pet evolve");
        }
    }
}

fn feed_pet(storage_path: &str, xp_file: &str) {
    let meals = increment_and_get(storage_path, "pet_feed");
    let bonus_due = meals % 10 == 0;

    if bonus_due {
        unlock_achievement(storage_path, xp_file, "Touch Grass", 500);
    }

    println!("Pet fed. Meals: {}", meals);
    if bonus_due {
        println!("The pet looks unusually powerful today.");
    }
}

fn print_pet_mood(storage_path: &str, xp_file: &str) {
    let xp = read_xp(xp_file);
    let state = pet::detect(xp);
    let meals = read_count(storage_path, "pet_feed");

    println!("Mood: {}", state.mood);
    println!("Fed: {} times", meals);
    println!("{}", state.message);
}

fn print_pet_evolution(xp_file: &str) {
    let xp = read_xp(xp_file);
    let next = match xp {
        x if x < 100 => Some((100, "Baby Penguin")),
        x if x < 300 => Some((300, "Terminal Fox")),
        x if x < 600 => Some((600, "Garuda Dragon")),
        x if x < 1000 => Some((1000, "Kernel Beast")),
        x if x < 2000 => Some((2000, "Arch Spirit")),
        _ => None,
    };

    println!("Current pet: {}", active_pet(xp));
    if let Some((required_xp, name)) = next {
        println!("Next evolution: {} at {} XP", name, required_xp);
        println!("Remaining: {} XP", required_xp - xp);
    } else {
        println!("Max evolution reached.");
    }
}

fn print_profile(storage_path: &str, xp_file: &str) {
    let xp = read_xp(xp_file);
    let list = achievements::get_list();
    let unlocked = unlocked_count(storage_path);
    let level = level_for_xp(xp);
    let next = next_level_xp(xp);

    println!("==== Achievix Profile ====");
    println!("XP: {}", xp);
    println!("Level: {}", level);
    println!("Next level: {} XP", next);
    println!("Achievements: {}/{}", unlocked, list.len());
    println!("Active pet: {}", active_pet(xp));
    println!("Events seen: {}", read_count(storage_path, "events_seen"));
    println!(
        "Events throttled: {}",
        read_count(storage_path, "events_throttled")
    );
}

fn print_stats(storage_path: &str, xp_file: &str) {
    let xp = read_xp(xp_file);
    let list = achievements::get_list();
    let unlocked = unlocked_count(storage_path);
    let mut common = 0;
    let mut rare = 0;
    let mut epic = 0;
    let mut legendary = 0;

    for achievement in &list {
        if !is_unlocked(storage_path, achievement) {
            continue;
        }

        match achievements::get_meta(achievement).rarity {
            achievements::Rarity::Common => common += 1,
            achievements::Rarity::Rare => rare += 1,
            achievements::Rarity::Epic => epic += 1,
            achievements::Rarity::Legendary => legendary += 1,
        }
    }

    println!("==== Achievix Stats ====");
    println!("XP: {}", xp);
    println!("Level: {}", level_for_xp(xp));
    println!("Achievements: {}/{}", unlocked, list.len());
    println!("Common: {}", common);
    println!("Rare: {}", rare);
    println!("Epic: {}", epic);
    println!("Legendary: {}", legendary);
    println!("Events seen: {}", read_count(storage_path, "events_seen"));
    println!(
        "Events throttled: {}",
        read_count(storage_path, "events_throttled")
    );
    println!("Pet feeds: {}", read_count(storage_path, "pet_feed"));
}

fn print_achievement_list(storage_path: &str) {
    for achievement in achievements::get_list() {
        let mark = if is_unlocked(storage_path, achievement) {
            "x"
        } else {
            " "
        };
        let meta = achievements::get_meta(achievement);
        println!("[{}] {:<9} {}", mark, meta.rarity.label(), achievement);
    }
}

fn read_count(path: &str, key: &str) -> i32 {
    fs::read_to_string(format!("{}/{}.count", path, key))
        .unwrap_or_else(|_| "0".into())
        .trim()
        .parse::<i32>()
        .unwrap_or(0)
}

fn print_debug(storage_path: &str, xp_file: &str) {
    println!("==== Achievix Debug ====");
    println!("Storage: {}", storage_path);
    println!("XP file: {}", xp_file);
    println!("XP: {}", read_xp(xp_file));
    println!("Known achievements: {}", achievements::get_list().len());
    println!("Unlocked achievements: {}", unlocked_count(storage_path));
    println!("Scan throttle: {}s", SCAN_THROTTLE_SECONDS);
    println!("Event throttle: {}s", EVENT_THROTTLE_SECONDS);
    println!("Events seen: {}", read_count(storage_path, "events_seen"));
    println!(
        "Events throttled: {}",
        read_count(storage_path, "events_throttled")
    );
}

fn print_export(storage_path: &str, xp_file: &str) {
    let xp = read_xp(xp_file);
    let unlocked: Vec<&str> = achievements::get_list()
        .into_iter()
        .filter(|achievement| is_unlocked(storage_path, achievement))
        .collect();

    println!("{{");
    println!("  \"xp\": {},", xp);
    println!("  \"level\": {},", level_for_xp(xp));
    println!("  \"active_pet\": \"{}\",", json_escape(active_pet(xp)));
    println!(
        "  \"events_seen\": {},",
        read_count(storage_path, "events_seen")
    );
    println!(
        "  \"events_throttled\": {},",
        read_count(storage_path, "events_throttled")
    );
    println!("  \"achievements\": [");
    for (index, achievement) in unlocked.iter().enumerate() {
        let comma = if index + 1 == unlocked.len() { "" } else { "," };
        let meta = achievements::get_meta(achievement);
        println!(
            "    {{ \"name\": \"{}\", \"rarity\": \"{}\" }}{}",
            json_escape(achievement),
            meta.rarity.label(),
            comma
        );
    }
    println!("  ]");
    println!("}}");
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn print_pets(xp: i32) {
    let pets = [
        (100, "🐧 Baby Penguin"),
        (300, "🦊 Terminal Fox"),
        (600, "🐉 Garuda Dragon"),
        (1000, "👾 Kernel Beast"),
        (2000, "☢️ Arch Spirit"),
    ];

    println!("Active pet: {}", active_pet(xp));
    for (required_xp, name) in pets {
        let mark = if xp >= required_xp { "x" } else { " " };
        println!("[{}] {:>4} XP - {}", mark, required_xp, name);
    }
}

fn print_help() {
    println!("Achievix - Linux RPG runtime layer");
    println!();
    println!("Core CLI:");
    println!("  achievix help");
    println!("  achievix profile");
    println!("  achievix stats");
    println!("  achievix list");
    println!("  achievix pets");
    println!("  achievix scan");
    println!();
    println!("Gameplay/system:");
    println!("  achievix event <type> <data> [detail]");
    println!("  achievix daemon <interval_seconds>");
    println!("  achievix overlay");
    println!("  achievix pet status");
    println!();
    println!("Pet control:");
    println!("  achievix pet status");
    println!("  achievix pet feed");
    println!("  achievix pet mood");
    println!("  achievix pet evolve");
    println!();
    println!("Debug/dev:");
    println!("  achievix xp");
    println!("  achievix reset");
    println!("  achievix debug");
    println!("  achievix export");
    println!();
    println!("Shell hook:");
    println!("  arch-achieve trigger <command> <full command>");
}

fn launch_overlay() {
    let candidates = ["achievix-overlay", "./achievix-overlay.sh"];
    for candidate in candidates {
        if std::process::Command::new(candidate).spawn().is_ok() {
            return;
        }
    }

    eprintln!("Overlay launcher not found. Try ./achievix-overlay.sh from the project directory.");
}
