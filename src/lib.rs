use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Config {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start a new Pomodoro work session
    Start,

    /// Start a short break
    Break,

    /// Start a long break after several sessions
    LongBreak,

    /// Pause the current timer
    Pause,

    /// Resume the current timer
    Resume,

    /// Stop and reset the current session
    Stop,

    /// Show the current timer status
    Status,

    /// View the history of completed sessions
    Log,
}
