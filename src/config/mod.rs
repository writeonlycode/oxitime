use clap::{Parser, Subcommand};
use config_args::ConfigArgs;
use config_file::ConfigFile;
use serde::Deserialize;
use std::time::Duration;

mod config_args;
mod config_file;

pub struct Config {
    pub command: TimerCommand,
    pub pomodoro_duration: Duration,
    pub short_break_duration: Duration,
    pub long_break_duration: Duration,
}

impl Config {
    pub fn load() -> Config {
        let args = ConfigArgs::parse();
        let config = ConfigFile::load();

        Config {
            command: args.command,
            pomodoro_duration: args.pomodoro_duration.unwrap_or(
                config
                    .pomodoro_duration
                    .unwrap_or(Duration::from_secs(25 * 60)),
            ),
            short_break_duration: args.short_break_duration.unwrap_or(
                config
                    .short_break_duration
                    .unwrap_or(Duration::from_secs(5 * 60)),
            ),
            long_break_duration: args.long_break_duration.unwrap_or(
                config
                    .long_break_duration
                    .unwrap_or(Duration::from_secs(15 * 60)),
            ),
        }
    }
}

#[derive(Subcommand, Deserialize, Debug)]
pub enum TimerCommand {
    /// Start a new Pomodoro work session
    Start,

    /// Start a short break
    ShortBreak,

    /// Start a long break after several sessions
    LongBreak,

    /// View the history of completed sessions
    Log,
}
