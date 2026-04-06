// src/achievements.rs

pub fn get_list() -> Vec<&'static str> {
    vec![
        "A great power...", "PKGBUILD Hacker", "Cargo Master", "Network Ninja",
        "I use Arch BTW", "Fetch Enjoyer", "you're stupid", "Git Init", "The Cleaner",
        "System Refreshed", "Mirror Master", "Log Whisperer", "Disk Surgeon",
        "The Architect", "Manual Reader", "Package Hoarder","Media moments",

        "Shell Jumper", "Alias Wizard", "Disk Whisperer", "The Observer",
        "Wallpaper Hunter", "Font Explorer", "Kernel Peek", "Time Keeper",
        "Process Watcher",

        "Mirror Surgeon", "Regex Summoner", "Pipe Lord", "Dotfile Keeper",
        "Compile Time Champion", "AUR Addict", "PKGBUILD Surgeon",
        "Time Traveler", "Systemd Sorcerer", "Kernel Juggler",

        "System Breaker", "Kernel Gambler", "Boot Surgeon", "Chroot Traveler",
        "The Summoning", "Docker Tamer", "Wayland Explorer",
        "Filesystem Whisperer", "Network Necromancer", "System Resurrection"
    ]
}


pub fn check(cmd: &str, full: &str, storage: &str) -> Option<(&'static str, i32)> {
    match cmd {

        "alias" => Some(("Alias Wizard", 50)),
        "man" => Some(("Manual Reader", 50)),
        "df" if full.contains("-h") => Some(("Disk Whisperer", 50)),
        "ls" if full.contains("-la") => Some(("The Observer", 30)),
        "feh" | "nitrogen" | "swww" => Some(("Wallpaper Hunter", 50)),
        "fc-list" => Some(("Font Explorer", 40)),
        "uname" if full.contains("-a") => Some(("Kernel Peek", 50)),
        "timedatectl" => Some(("Time Keeper", 50)),
        "ps" if full.contains("aux") => Some(("Process Watcher", 40)),

        "reflector" if full.contains("--sort") => Some(("Mirror Surgeon", 100)),
        "grep" if full.contains("-E") || full.contains("-P") => Some(("Regex Summoner", 120)),
        _ if full.matches('|').count() >= 2 => Some(("Pipe Lord", 150)),
        "git" if full.contains("add") && full.contains(".config") => Some(("Dotfile Keeper", 150)),
        "make" => Some(("Compile Time Champion", 150)),
        "yay" if full.contains("-S") => Some(("AUR Addict", 150)),
        "nano" | "vim" | "nvim" if full.contains("PKGBUILD") => Some(("PKGBUILD Surgeon", 200)),
        "git" if full.contains("reset --hard") => Some(("Time Traveler", 150)),
        "systemctl" if full.contains("enable") || full.contains("disable") => {
            Some(("Systemd Sorcerer", 150))
        },
        "pacman" if full.contains("linux-lts") => Some(("Kernel Juggler", 150)),

        "pacman" if full.contains("-Rdd") => Some(("System Breaker", 300)),
        "mkinitcpio" if full.contains("-P") => Some(("Kernel Gambler", 300)),
        "grub-mkconfig" => Some(("Boot Surgeon", 300)),
        "arch-chroot" => Some(("Chroot Traveler", 300)),
        "dd" if full.contains("if=") => Some(("The Summoning", 400)),
        "docker" if full.contains("run") => Some(("Docker Tamer", 250)),
        "sway" | "hyprland" | "niri" => Some(("WM Explorer", 200)),
        "btrfs" | "zfs" => Some(("Filesystem Whisperer", 300)),
        "iptables" | "nft" => Some(("Network Necromancer", 300)),
        "pacman" if full.contains("-Qk") => Some(("System Resurrection", 350)),

        "nmtui" => Some(("Network Nomad", 150)),
        "btop" | "htop" | "top" => Some(("System Overseer", 50)),
        "pacman" if full.contains("-Rns") => Some(("Deep Clean", 200)),
        "sh" | "bash" | "zsh" if full.contains(".sh") => Some(("Script Kiddy", 100)),
        "sudo" | "doas" | "su" => Some(("A great power...", 100)),
        "pacman" if full.contains("-Syu") => Some(("System Refreshed", 150)),
        "reflector" => Some(("Mirror Master", 100)),
        "journalctl" => Some(("Log Whisperer", 50)),
        "git" if full.contains("init") => Some(("Git Init", 100)),
        "cargo" if full.contains("--release") => Some(("Cargo Master", 100)),
        "ip" if full.contains("a") => Some(("Network Ninja", 100)),
        "clear" => Some(("The Cleaner", 50)),
        "mpv" => Some(("Media moments", 50)),
        "rm" if full.contains("-rf") && full.contains("/") => Some(("you're stupid", 1000)),

        "fastfetch" | "neofetch" => {
            let count = crate::increment_and_get(storage, "fetch_count");
            if count == 1 { Some(("I use Arch BTW", 100)) }
            else if count == 50 { Some(("Fetch Enjoyer", 200)) }
            else { None }
        },

        _ => None,
    }
}
