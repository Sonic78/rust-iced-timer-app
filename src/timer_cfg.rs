use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub timer: TimerConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerConfig {
    pub red_text_limit_seconds: u64,
}

pub(crate) fn load_config_from_path(path: &Path) -> Config {
    let config_str = std::fs::read_to_string(path)
        .unwrap_or_else(|_| String::from("[timer]\nred_text_limit_seconds = 120"));
    toml::from_str(&config_str).unwrap_or_else(|_| Config {
        timer: TimerConfig {
            red_text_limit_seconds: 120,
        },
    })
}

pub fn load_config() -> Config {
    load_config_from_path(Path::new("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_config_dir() -> std::path::PathBuf {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("timer_cfg_test_{}_{}", std::process::id(), now));
        fs::create_dir_all(&dir).expect("failed to create temp dir");
        dir
    }

    #[test]
    fn deserialize_timer_config_from_toml() {
        let config_toml = r#"
            [timer]
            red_text_limit_seconds = 200
        "#;
        let config: Config = toml::from_str(config_toml).expect("failed to parse TOML");

        assert_eq!(config.timer.red_text_limit_seconds, 200);
    }

    #[test]
    fn load_config_from_path_reads_existing_file() {
        let dir = temp_config_dir();
        let path = dir.join("config.toml");
        fs::write(&path, "[timer]\nred_text_limit_seconds = 200").expect("failed to write config");

        let config = load_config_from_path(&path);
        assert_eq!(config.timer.red_text_limit_seconds, 200);

        fs::remove_dir_all(dir).expect("failed to remove temp dir");
    }

    #[test]
    fn load_config_from_path_uses_default_when_missing_file() {
        let dir = temp_config_dir();
        let path = dir.join("config.toml");

        let config = load_config_from_path(&path);
        assert_eq!(config.timer.red_text_limit_seconds, 120);

        fs::remove_dir_all(dir).expect("failed to remove temp dir");
    }
}
