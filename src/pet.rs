use chrono::Timelike;
use std::fs;

pub struct PetState {
    pub icon: &'static str,
    pub name: &'static str,
    pub mood: &'static str,
    pub activity: &'static str,
    pub message: &'static str,
}

pub fn detect(xp: i32) -> PetState {
    let (icon, name) = active_pet(xp);
    let hour = chrono::Local::now().hour();
    let temp = cpu_temp();
    let load = load_average();
    let memory = memory_used_percent();

    if temp.unwrap_or(0) >= 80 {
        return PetState {
            icon,
            name,
            mood: "overheated",
            activity: "watching CPU thermals",
            message: "System is cooking. Your pet recommends less fire, more airflow.",
        };
    }

    if load.unwrap_or(0.0) >= 4.0 {
        return PetState {
            icon,
            name,
            mood: "busy",
            activity: "counting runaway processes",
            message: "Load is spicy. Pet is holding the terminal together with vibes.",
        };
    }

    if memory.unwrap_or(0) >= 85 {
        return PetState {
            icon,
            name,
            mood: "concerned",
            activity: "guarding RAM",
            message: "Memory pressure is high. Something is eating the desktop budget.",
        };
    }

    if hour < 5 {
        return PetState {
            icon,
            name,
            mood: "sleepy",
            activity: "night shift",
            message: "It is late. The pet is awake, but judging your sleep schedule.",
        };
    }

    if is_kde_session() {
        return PetState {
            icon,
            name,
            mood: "plasma-charged",
            activity: "floating through KDE",
            message: "KDE detected. Pet has claimed the desktop as a tiny kingdom.",
        };
    }

    if uses_cachy_kernel() {
        return PetState {
            icon,
            name,
            mood: "fast",
            activity: "surfing the kernel scheduler",
            message: "Cachy kernel energy detected. Pet is feeling optimized.",
        };
    }

    PetState {
        icon,
        name,
        mood: "calm",
        activity: "watching for achievements",
        message: "All systems look normal. Pet is ready for suspicious terminal choices.",
    }
}

fn active_pet(xp: i32) -> (&'static str, &'static str) {
    match xp {
        x if x >= 2000 => ("☢️", "Arch Spirit"),
        x if x >= 1000 => ("👾", "Kernel Beast"),
        x if x >= 600 => ("🐉", "Garuda Dragon"),
        x if x >= 300 => ("🦊", "Terminal Fox"),
        x if x >= 100 => ("🐧", "Baby Penguin"),
        _ => ("·", "No Pet"),
    }
}

fn cpu_temp() -> Option<i32> {
    fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .ok()
        .and_then(|value| value.trim().parse::<i32>().ok())
        .map(|value| value / 1000)
}

fn load_average() -> Option<f32> {
    fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|value| value.split_whitespace().next()?.parse::<f32>().ok())
}

fn memory_used_percent() -> Option<i32> {
    let meminfo = fs::read_to_string("/proc/meminfo").ok()?;
    let mut total = None;
    let mut available = None;

    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1)?.parse::<i32>().ok();
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1)?.parse::<i32>().ok();
        }
    }

    let total = total?;
    let available = available?;
    Some(((total - available) * 100) / total)
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
