# Achievix

Achievix is a fork and experimental continuation of the AUR project:

https://aur.archlinux.org/arch-achievements-rpg-git.git

The original idea is simple and excellent: turn daily Linux terminal usage into an achievement system. This fork pushes that idea further into a small Linux RPG runtime layer:

- CLI as the user interface
- systemd daemon as the world engine
- KDE overlay as a desktop HUD
- pets as a companion AI layer
- achievements as the quest system

## What It Does

Achievix watches terminal commands and system state, then unlocks achievements with XP rewards, desktop notifications, and an optional sound effect.

It keeps compatibility with the classic lock-file progress model:

- XP: `~/.arch_achievements/total_xp`
- unlocked achievements: `~/.arch_achievements/*.lock`
- achievement sound: `~/.arch_achievements/achieve.mp3`

## Features

- Rust CLI runtime
- XP and player levels
- achievement rarity: `common`, `rare`, `epic`, `legendary`
- pet unlocks by XP
- pet AI reacting to system load, RAM, temperature, KDE, and kernel state
- distro detection through `/etc/os-release`
- passive scan and systemd daemon
- Fish and Zsh command hooks
- KDE pet overlay prototype
- event API for external programs
- telemetry throttling to avoid spammy scans/events
- export/debug commands for development

## Installation

Requirements:

- Rust and Cargo
- `notify-rust` dependencies, usually DBus/libdbus on desktop Linux
- Fish or Zsh if you want command hooks
- optional: `paplay`, `pw-play`, `mpv`, or `ffplay` for achievement sound
- optional: Qt QML runner for overlay, such as `qml6`, `qmlscene`, or `qml`

Install:

```bash
chmod +x setup.sh
./setup.sh
```

The installer builds the Rust binary and installs:

- `/usr/local/bin/arch-achieve`
- `/usr/local/bin/achievix`
- `/usr/local/bin/achievix-overlay`
- Fish/Zsh hooks when available
- user systemd service: `achievix.service`

The project includes a default achievement sound:

```bash
assets/achieve.mp3
```

Setup copies it on first install to:

```bash
~/.arch_achievements/achieve.mp3
```

If that file already exists, setup leaves it alone so users can keep their own custom sound.

## Core CLI

```bash
achievix help
achievix profile
achievix stats
achievix list
achievix pets
achievix scan
```

## Gameplay/System

```bash
achievix event <type> <data> [detail]
achievix daemon <interval>
achievix overlay
achievix pet
```

Example external event:

```bash
achievix event blender render_complete
```

## Pet Control

```bash
achievix pet status
achievix pet feed
achievix pet mood
achievix pet evolve
```

Pet unlock thresholds:

- 100 XP: Baby Penguin
- 300 XP: Terminal Fox
- 600 XP: Garuda Dragon
- 1000 XP: Kernel Beast
- 2000 XP: Arch Spirit

## Debug/Development

```bash
achievix xp
achievix reset
achievix debug
achievix export
```

## Distro Achievements

Achievix reads `/etc/os-release` and unlocks achievements for distro families and specific systems, including:

- Arch family
- Garuda Linux
- CachyOS
- Manjaro
- EndeavourOS
- Debian family
- Ubuntu
- Fedora
- openSUSE
- NixOS

## Sound

Achievement unlocks play `~/.arch_achievements/achieve.mp3` when the file exists.

The included default sound lives at:

```bash
assets/achieve.mp3
```

To customize it, replace:

```bash
~/.arch_achievements/achieve.mp3
```

Playback tries these commands in order:

1. `paplay`
2. `pw-play`
3. `mpv`
4. `ffplay`

The sound only plays when a new achievement is actually unlocked. Existing `.lock` files do not retrigger sound or XP.

## Notes

This fork is not a clean upstream release. It is a playful, rapidly evolving Achievix branch built on top of the AUR project listed above. The goal is to keep the original spirit while adding more RPG systems around Linux desktop usage.
