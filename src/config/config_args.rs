use super::TimerCommand;
use anyhow::Result;
use clap::Parser;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: TimerCommand,

    #[arg(long, value_parser = ConfigArgs::parse_duration)]
    pub pomodoro_duration: Option<Duration>,

    #[arg(long, value_parser = ConfigArgs::parse_duration)]
    pub short_break_duration: Option<Duration>,

    #[arg(long, value_parser = ConfigArgs::parse_duration)]
    pub long_break_duration: Option<Duration>,

    #[arg(long)]
    pub toggl_api_token: Option<String>,

    #[arg(long)]
    pub toggl_workspace_id: Option<String>,

    #[arg(long)]
    pub description: Option<String>,
}

impl ConfigArgs {
    fn parse_duration(s: &str) -> Result<Duration, String> {
        let duration = humantime::parse_duration(s).map_err(|_| "Invalid duration".to_string())?;
        Ok(duration)
    }
}
