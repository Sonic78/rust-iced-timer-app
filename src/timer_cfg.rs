use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub timer: TimerConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerConfig {
    pub red_text_limit_seconds: u64,
}

pub fn load_config() -> Config {
    let config_str = std::fs::read_to_string("config.toml")
        .unwrap_or_else(|_| String::from("[timer]\nred_text_limit_seconds = 120"));
    toml::from_str(&config_str).unwrap_or_else(|_| Config {
        timer: TimerConfig {
            red_text_limit_seconds: 120,
        },
    })
}
