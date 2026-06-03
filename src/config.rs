use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub tts_enabled: bool,
    pub tts_model_path: String,
    pub tts_player: String,
}

impl Config {
    pub fn load() -> Self {
        let mut config = Self::default();

        if let Some(path) = config_path() {
            if let Ok(contents) = fs::read_to_string(path) {
                config.apply_config_file(&contents);
            }
        }

        if let Ok(value) = std::env::var("ACHIEVIX_TTS_ENABLED") {
            config.tts_enabled = parse_bool(&value).unwrap_or(config.tts_enabled);
        }

        if let Ok(value) = std::env::var("ACHIEVIX_TTS_MODEL") {
            config.tts_model_path = expand_home(&value);
        }

        if let Ok(value) = std::env::var("ACHIEVIX_TTS_PLAYER") {
            config.tts_player = value;
        }

        config
    }

    fn apply_config_file(&mut self, contents: &str) {
        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let Some((key, value)) = line.split_once('=') else {
                continue;
            };

            let key = key.trim();
            let value = value.trim().trim_matches('"');

            match key {
                "tts_enabled" => {
                    self.tts_enabled = parse_bool(value).unwrap_or(self.tts_enabled);
                }
                "tts_model_path" => {
                    self.tts_model_path = expand_home(value);
                }
                "tts_player" => {
                    self.tts_player = value.to_string();
                }
                _ => {}
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            tts_enabled: true,
            tts_model_path: expand_home("~/piper/en_US-ryan-high.onnx"),
            tts_player: "mpv".into(),
        }
    }
}

fn config_path() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join(".config/achievix/config"))
}

pub fn expand_home(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return format!("{}/{}", home, rest);
        }
    }

    path.to_string()
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}
