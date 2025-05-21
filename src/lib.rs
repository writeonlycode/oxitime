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

    /// View the history of completed sessions
    Log,
}
