use serde::Deserialize;
use std::{io::Read, time::Duration};

#[derive(Deserialize)]
pub struct ConfigFile {
    #[serde(default, with = "humantime_serde::option")]
    pub pomodoro_duration: Option<Duration>,

    #[serde(default, with = "humantime_serde::option")]
    pub short_break_duration: Option<Duration>,

    #[serde(default, with = "humantime_serde::option")]
    pub long_break_duration: Option<Duration>,
}

impl ConfigFile {
    pub fn load() -> ConfigFile {
        let path = dirs::config_dir().unwrap().join("oxitime/config.toml");
        let mut file = std::fs::File::open(path).unwrap();

        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        let config: ConfigFile = toml::from_str(input.as_str()).unwrap();

        ConfigFile {
            pomodoro_duration: config.pomodoro_duration,
            short_break_duration: config.short_break_duration,
            long_break_duration: config.long_break_duration,
        }
    }
}
