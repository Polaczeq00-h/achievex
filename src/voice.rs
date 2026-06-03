use crate::achievements::Achievement;
use crate::config::Config;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn play_achievement_voice(
    achievement: &Achievement,
    config: &Config,
) -> Result<(), Box<dyn Error>> {
    if !config.tts_enabled {
        return Ok(());
    }

    let cache_path = cache_path(&achievement.id)?;

    if !cache_path.exists() {
        generate_voice(achievement, config, &cache_path)?;
    }

    if cache_path.exists() {
        play_wav(&cache_path, &config.tts_player);
    }

    Ok(())
}

fn generate_voice(
    achievement: &Achievement,
    config: &Config,
    output_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let model_path = Path::new(&config.tts_model_path);
    if !model_path.exists() {
        return Ok(());
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let line = narrator_line(achievement);
    let mut child = match Command::new("piper-tts")
        .arg("--model")
        .arg(&config.tts_model_path)
        .arg("--output_file")
        .arg(output_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return Ok(()),
    };

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(line.as_bytes())?;
        stdin.write_all(b"\n")?;
    }

    let status = child.wait()?;
    if !status.success() {
        let _ = fs::remove_file(output_path);
    }

    Ok(())
}

fn narrator_line(achievement: &Achievement) -> String {
    let context = format!(
        "{} {}",
        achievement.name.to_ascii_lowercase(),
        achievement.description.to_ascii_lowercase()
    );

    let comment = if context.contains("render") {
        "The machine screamed, but yes, that worked."
    } else if context.contains("detect") || context.contains("distro") {
        "Rare signal detected. The machine reluctantly approves."
    } else if context.contains("pet") {
        "Your companion noticed. This may become a problem."
    } else {
        achievement.rarity.voice_line()
    };

    format!("Achievement unlocked: {}. {}", achievement.name, comment)
}

fn play_wav(path: &Path, preferred_player: &str) {
    let path_string = path.to_string_lossy().to_string();
    let mut players = Vec::new();

    if !preferred_player.trim().is_empty() {
        players.push(preferred_player.to_string());
    }

    for fallback in ["mpv", "paplay"] {
        if !players.iter().any(|player| player == fallback) {
            players.push(fallback.to_string());
        }
    }

    for player in players {
        if spawn_player(&player, &path_string) {
            break;
        }
    }
}

fn spawn_player(player: &str, path: &str) -> bool {
    let mut command = Command::new(player);

    match player {
        "mpv" => {
            command.args(["--no-video", "--really-quiet", path]);
        }
        "paplay" => {
            command.arg(path);
        }
        other => {
            command.arg(path);
            if other.trim().is_empty() {
                return false;
            }
        }
    }

    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok()
}

fn cache_path(achievement_id: &str) -> Result<PathBuf, Box<dyn Error>> {
    let home = std::env::var("HOME")?;
    Ok(PathBuf::from(home)
        .join(".local/share/achievix/voice_cache/v2")
        .join(format!("{}.wav", achievement_id)))
}
