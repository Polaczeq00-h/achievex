# 🏆 Arch-Achievements RPG

**Gamify your Arch Linux terminal experience!**

Arch-Achievements is a lightweight RPG engine built in **Rust**. It tracks your terminal commands in real-time and unlocks achievements with XP rewards and system notifications.

---

## ✨ Features

- 🦀 **High-Performance Rust Engine**: Blazing fast command analysis with zero shell lag.
- 🔔 **Live Notifications**: Get desktop alerts the moment you level up.
- 📈 **XP & Leveling System**: Earn XP for your daily tasks and aim for the Platinum trophy.
- 🌙 **Context-Aware Trophies**: Unlock "Night Owl" by working late or "Overclocked Heart" when your CPU hits high temperatures.
- 🛠️ **Modular Design**: Easily add your own custom achievements in a dedicated file.

---

## Quick Start

### Prerequisites

- **Rust & Cargo**: `sudo pacman -S rust`
- **Libdbus**: Required for notifications (usually pre-installed).
- **Zsh a default shell**: A bash and fish version is planned, but unfortunately only zsh is supported at the moment.

### Installation

Clone the repository and run the automated setup script:

```bash
git clone https://github.com/Kanjurito/arch-achievements.git
cd arch-achievements
chmod +x setup.sh
./setup.sh
```

After installation, restart your terminal or run

```bash
source ~/.zshrc
```

---

## Commands

```bash
achievements
```

Display your profile, total XP, and unlocked trophies.

```bash
achievements -reset
```

Wipe your progress and start a fresh journey.

## Exemple of Achivements to discover

| achievement     | Description                            |
| --------------- | -------------------------------------- |
| I use Arch BTW  | Run your first fastfetch or neofetch   |
| PKGBUILD Hacker | Edit an AUR package build file         |
| Cargo Master    | Compile a Rust project in release mode |
| The Architect   | Successfully install this projet       |
| you're stupid   | Run the forbidden rm -rf / command     |
| arch user BTW   | [PLATINUM] Unlock every single trophy  |

add many other,catch them all !!!

## Built with love for the Arch Linux community.
