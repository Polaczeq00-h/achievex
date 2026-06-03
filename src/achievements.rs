// src/achievements.rs

#[derive(Clone, Copy)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub fn label(self) -> &'static str {
        match self {
            Self::Common => "common",
            Self::Rare => "rare",
            Self::Epic => "epic",
            Self::Legendary => "legendary",
        }
    }
}

impl Rarity {
    pub fn voice_line(self) -> &'static str {
        match self {
            Self::Common => "Progress detected. Humanity remains under review.",
            Self::Rare => "Not bad. The machine noticed.",
            Self::Epic => "That was almost impressive.",
            Self::Legendary => "Legendary achievement unlocked. Civilization may recover.",
        }
    }
}

pub struct AchievementMeta {
    pub rarity: Rarity,
}

pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: Rarity,
}

impl Achievement {
    pub fn from_name(name: &str) -> Self {
        let meta = get_meta(name);
        Self {
            id: achievement_id(name),
            name: name.to_string(),
            description: get_description(name).to_string(),
            rarity: meta.rarity,
        }
    }
}

pub fn achievement_id(name: &str) -> String {
    let mut out = String::new();
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }

    out.trim_matches('_').to_string()
}

pub fn get_meta(name: &str) -> AchievementMeta {
    let rarity = match name {
        "you're stupid" | "The Summoning" | "Platinum Goblin" => Rarity::Legendary,
        "System Breaker"
        | "Kernel Gambler"
        | "Boot Surgeon"
        | "Chroot Traveler"
        | "Network Necromancer"
        | "System Resurrection"
        | "Render Victim"
        | "Cycles Abuser"
        | "Touch Grass"
        | "Overclocked Heart" => Rarity::Epic,
        "Fetch Enjoyer"
        | "PKGBUILD Hacker"
        | "Cargo Master"
        | "AUR Addict"
        | "PKGBUILD Surgeon"
        | "Time Traveler"
        | "Systemd Sorcerer"
        | "Kernel Juggler"
        | "Docker Tamer"
        | "Wayland Explorer"
        | "WM Explorer"
        | "Filesystem Whisperer"
        | "Kernel Collector"
        | "NVIDIA Survivor"
        | "GPU Priest"
        | "Proton Wizard"
        | "Minecraft Addict"
        | "Mod Addict"
        | "Shader Victim"
        | "Asciiquarium Marine Biologist"
        | "Arch Family Resident"
        | "Pure Arch Pilgrim"
        | "Garuda Native"
        | "CachyOS Sprinter"
        | "Cachy Kernel Rider"
        | "Manjaro Mountaineer"
        | "Endeavour Voyager"
        | "Ubuntu Tourist"
        | "Debian Elder"
        | "Fedora Hat Wearer"
        | "openSUSE Chameleon"
        | "NixOS Time Wizard"
        | "Night Owl" => Rarity::Rare,
        _ => Rarity::Common,
    };

    AchievementMeta { rarity }
}

pub fn get_description(name: &str) -> &'static str {
    match name {
        "I use Arch BTW" => "Run fastfetch or neofetch for the first time.",
        "Fetch Enjoyer" => "Use fetch tools enough times to become a mirror hazard.",
        "NVIDIA Survivor" => "Detect NVIDIA tooling and live to tell the tale.",
        "Blender Victim" => "Launch Blender and accept the default cube problem.",
        "Render Victim" => "Report a completed Blender render event.",
        "Cycles Abuser" => "Render with Cycles and warm the room a little.",
        "Steam Enjoyer" => "Detect Steam on the system.",
        "Proton Wizard" => "Run Proton or Wine helper tooling.",
        "Fish Heretic" => "Use the fish shell.",
        "KDE Goblin" => "Detect a KDE Plasma desktop session.",
        "Garuda Native" => "Detect Garuda Linux.",
        "Cachy Kernel Rider" => "Detect a Cachy kernel.",
        "Arch Family Resident" => "Detect an Arch-family distribution.",
        "Ubuntu Tourist" => "Detect Ubuntu.",
        "Debian Elder" => "Detect Debian.",
        "Fedora Hat Wearer" => "Detect Fedora.",
        "openSUSE Chameleon" => "Detect openSUSE.",
        "NixOS Time Wizard" => "Detect NixOS.",
        "Platinum Goblin" => "Unlock every known achievement.",
        _ => "Unlock a Linux RPG achievement.",
    }
}

pub fn get_list() -> Vec<&'static str> {
    vec![
        "A great power...",
        "PKGBUILD Hacker",
        "Cargo Master",
        "Network Ninja",
        "I use Arch BTW",
        "Fetch Enjoyer",
        "you're stupid",
        "Git Init",
        "The Cleaner",
        "System Refreshed",
        "Mirror Master",
        "Log Whisperer",
        "Disk Surgeon",
        "The Architect",
        "Manual Reader",
        "Package Hoarder",
        "Media moments",
        "Little Hacker",
        "Shell Jumper",
        "Alias Wizard",
        "Disk Whisperer",
        "The Observer",
        "Wallpaper Hunter",
        "Font Explorer",
        "Kernel Peek",
        "Time Keeper",
        "Process Watcher",
        "Mirror Surgeon",
        "Regex Summoner",
        "Pipe Lord",
        "Dotfile Keeper",
        "Compile Time Champion",
        "AUR Addict",
        "PKGBUILD Surgeon",
        "Time Traveler",
        "Systemd Sorcerer",
        "Kernel Juggler",
        "System Breaker",
        "Kernel Gambler",
        "Boot Surgeon",
        "Chroot Traveler",
        "The Summoning",
        "Docker Tamer",
        "Wayland Explorer",
        "WM Explorer",
        "Filesystem Whisperer",
        "Network Necromancer",
        "System Resurrection",
        "Network Nomad",
        "System Overseer",
        "Deep Clean",
        "Script Kiddy",
        "Fish Heretic",
        "Bash Enjoyer",
        "ZSH Wizard",
        "Editor Goblin",
        "Vim Prisoner",
        "Nano Civilian",
        "Rust Crab",
        "Python Snake",
        "Node Goblin",
        "Git Historian",
        "Branch Manager",
        "Merge Conflict Survivor",
        "Flatpak Citizen",
        "Snap Survivor",
        "Snap Exorcist",
        "Pacman Archaeologist",
        "Orphan Cleaner",
        "Cache Goblin",
        "Kernel Collector",
        "NVIDIA Survivor",
        "GPU Priest",
        "Steam Enjoyer",
        "Lutris Summoner",
        "Proton Wizard",
        "Minecraft Addict",
        "Block Goblin",
        "Mod Addict",
        "Shader Victim",
        "Blender Victim",
        "Default Cube Murderer",
        "Render Victim",
        "Cycles Abuser",
        "Asciiquarium Marine Biologist",
        "Train Passenger",
        "Matrix Cosplayer",
        "Rice Master",
        "Blur Addict",
        "KDE Goblin",
        "Arch Family Resident",
        "Debian Family Resident",
        "Fedora Family Resident",
        "SUSE Family Resident",
        "Pure Arch Pilgrim",
        "Garuda Native",
        "CachyOS Sprinter",
        "Cachy Kernel Rider",
        "Manjaro Mountaineer",
        "Endeavour Voyager",
        "Ubuntu Tourist",
        "Debian Elder",
        "Fedora Hat Wearer",
        "openSUSE Chameleon",
        "NixOS Time Wizard",
        "Theme Collector",
        "Uptime Maniac",
        "Reboot Fixes Everything",
        "Touch Grass",
        "Night Owl",
        "Overclocked Heart",
        "Platinum Goblin",
    ]
}

pub fn check_event(domain: &str, event: &str, detail: &str) -> Option<(&'static str, i32)> {
    match (domain, event) {
        ("blender", "render_complete") => Some(("Render Victim", 250)),
        ("blender", "cycles_render") => Some(("Cycles Abuser", 300)),
        ("blender", "default_cube_deleted") => Some(("Default Cube Murderer", 100)),
        ("minecraft", "launch") => Some(("Minecraft Addict", 150)),
        ("minecraft", "mod_loaded") => Some(("Mod Addict", 200)),
        ("minecraft", "shader_loaded") => Some(("Shader Victim", 150)),
        ("steam", "launch") => Some(("Steam Enjoyer", 100)),
        ("steam", "proton_run") => Some(("Proton Wizard", 150)),
        ("kde", "theme_changed") => Some(("Theme Collector", 120)),
        ("kde", "blur_enabled") => Some(("Blur Addict", 150)),
        ("system", "touch_grass") => Some(("Touch Grass", 500)),
        _ if detail.contains("render_complete") => Some(("Render Victim", 250)),
        _ => None,
    }
}

pub fn check(cmd: &str, full: &str, storage: &str) -> Option<(&'static str, i32)> {
    match cmd {
        "alias" => Some(("Alias Wizard", 50)),
        "man" => Some(("Manual Reader", 50)),
        "df" if full.contains("-h") => Some(("Disk Whisperer", 50)),
        "ls" if full.contains("-la") => Some(("The Observer", 30)),
        "feh" | "nitrogen" | "swww" => Some(("Wallpaper Hunter", 50)),
        "fc-list" => Some(("Font Explorer", 40)),
        "uname" if full.contains("-a") || full.contains("-r") => Some(("Kernel Peek", 50)),
        "timedatectl" => Some(("Time Keeper", 50)),
        "ps" if full.contains("aux") => Some(("Process Watcher", 40)),
        "btop" | "htop" | "top" => Some(("System Overseer", 50)),

        "reflector" if full.contains("--sort") => Some(("Mirror Surgeon", 100)),
        "reflector" => Some(("Mirror Master", 100)),
        "grep" if full.contains("-E") || full.contains("-P") => Some(("Regex Summoner", 120)),
        _ if full.matches('|').count() >= 2 => Some(("Pipe Lord", 150)),

        "git" if full.contains("init") => Some(("Git Init", 100)),
        "git" if full.contains("branch") => Some(("Branch Manager", 80)),
        "git" if full.contains("merge") => Some(("Merge Conflict Survivor", 150)),
        "git" if full.contains("log") => Some(("Git Historian", 75)),
        "git" if full.contains("reset --hard") => Some(("Time Traveler", 150)),
        "git" if full.contains("add") && full.contains(".config") => Some(("Dotfile Keeper", 150)),

        "cargo" if full.contains("--release") => Some(("Cargo Master", 100)),
        "cargo" => Some(("Rust Crab", 75)),
        "python" | "python3" => Some(("Python Snake", 75)),
        "node" | "npm" => Some(("Node Goblin", 75)),
        "make" => Some(("Compile Time Champion", 150)),

        "yay" if full.contains("-S") || full.contains("-Syu") => Some(("AUR Addict", 150)),
        "paru" if full.contains("-S") || full.contains("-Syu") => Some(("AUR Addict", 150)),
        "nano" | "vim" | "nvim" if full.contains("PKGBUILD") => Some(("PKGBUILD Surgeon", 200)),
        "pacman" if full.contains("-Syu") => Some(("System Refreshed", 150)),
        "pacman" if full.contains("-Rns") => Some(("Deep Clean", 200)),
        "pacman" if full.contains("-Qdt") => Some(("Orphan Cleaner", 150)),
        "pacman" if full.contains("-Sc") || full.contains("-Scc") => Some(("Cache Goblin", 150)),
        "pacman" if full.contains("-Q") => Some(("Pacman Archaeologist", 70)),
        "pacman"
            if full.contains("linux-lts")
                || full.contains("linux-zen")
                || full.contains("linux-cachyos") =>
        {
            Some(("Kernel Collector", 150))
        }
        "pacman" if full.contains("-Rdd") => Some(("System Breaker", 300)),

        "flatpak" => Some(("Flatpak Citizen", 75)),
        "snap" if full.contains("install") => Some(("Snap Survivor", 200)),
        "snap" if full.contains("remove") => Some(("Snap Exorcist", 250)),

        "systemctl" if full.contains("enable") || full.contains("disable") => {
            Some(("Systemd Sorcerer", 150))
        }
        "journalctl" => Some(("Log Whisperer", 50)),
        "mkinitcpio" if full.contains("-P") => Some(("Kernel Gambler", 300)),
        "grub-mkconfig" => Some(("Boot Surgeon", 300)),
        "arch-chroot" => Some(("Chroot Traveler", 300)),
        "pacman" if full.contains("-Qk") => Some(("System Resurrection", 350)),

        "ip" if full.contains("a") => Some(("Network Ninja", 100)),
        "nmtui" => Some(("Network Nomad", 150)),
        "iptables" | "nft" => Some(("Network Necromancer", 300)),

        "docker" if full.contains("run") => Some(("Docker Tamer", 250)),
        "sway" | "hyprland" | "niri" => Some(("WM Explorer", 200)),
        "btrfs" | "zfs" => Some(("Filesystem Whisperer", 300)),

        "sh" | "bash" | "zsh" if full.contains(".sh") => Some(("Script Kiddy", 100)),
        "fish" => Some(("Fish Heretic", 100)),
        "bash" => Some(("Bash Enjoyer", 70)),
        "zsh" => Some(("ZSH Wizard", 70)),
        "nano" => Some(("Nano Civilian", 40)),
        "vim" | "nvim" => Some(("Vim Prisoner", 100)),
        "code" | "kate" | "micro" => Some(("Editor Goblin", 60)),

        "nvidia-smi" => Some(("NVIDIA Survivor", 150)),
        "nvtop" => Some(("GPU Priest", 150)),
        "steam" => Some(("Steam Enjoyer", 100)),
        "lutris" => Some(("Lutris Summoner", 100)),
        "protontricks" | "winetricks" => Some(("Proton Wizard", 150)),

        "minecraft-launcher" | "prismlauncher" | "modrinth-app" => Some(("Minecraft Addict", 150)),
        "java" if full.contains("minecraft") => Some(("Block Goblin", 100)),
        _ if full.contains(".jar") && full.contains("mods") => Some(("Mod Addict", 200)),
        _ if full.contains("shader") || full.contains("iris") => Some(("Shader Victim", 150)),

        "blender" => Some(("Blender Victim", 150)),
        _ if full.contains("default cube") || full.contains("cube") => {
            Some(("Default Cube Murderer", 100))
        }
        _ if full.contains("cycles") || full.contains("render") => Some(("Render Victim", 250)),

        "asciiquarium" => Some(("Asciiquarium Marine Biologist", 150)),
        "sl" => Some(("Train Passenger", 100)),
        "cmatrix" => Some(("Matrix Cosplayer", 100)),

        "lookandfeeltool" => Some(("KDE Goblin", 100)),
        _ if full.contains("kvantum") => Some(("Theme Collector", 120)),
        _ if full.contains("blur") => Some(("Blur Addict", 150)),
        _ if full.contains("rice") => Some(("Rice Master", 200)),

        "uptime" => {
            let count = crate::increment_and_get(storage, "uptime_count");
            if count == 10 {
                Some(("Uptime Maniac", 200))
            } else {
                None
            }
        }

        "reboot" | "systemctl" if full.contains("reboot") => Some(("Reboot Fixes Everything", 100)),

        "sudo" | "doas" | "su" => Some(("A great power...", 100)),
        "clear" => Some(("The Cleaner", 50)),
        "mpv" | "vlc" => Some(("Media moments", 50)),

        "rm" if full.contains("-rf") && full.contains("/") => Some(("you're stupid", 1000)),
        "dd" if full.contains("if=") => Some(("The Summoning", 400)),

        "fastfetch" | "neofetch" => {
            let count = crate::increment_and_get(storage, "fetch_count");
            if count == 1 {
                Some(("I use Arch BTW", 100))
            } else if count == 50 {
                Some(("Fetch Enjoyer", 200))
            } else {
                None
            }
        }

        _ => None,
    }
}
