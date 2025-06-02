use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
};
use serde::Deserialize;
use std::{
    io::Read,
    sync::mpsc,
    thread::{self},
    time::Duration,
};
use timer::{Timer, TimerKind, TimerMessage};

mod sync;
mod timer;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: TimerCommand,

    #[arg(long, value_parser = Args::parse_duration)]
    pub pomodoro_duration: Option<Duration>,

    #[arg(long, value_parser = Args::parse_duration)]
    pub short_break_duration: Option<Duration>,

    #[arg(long, value_parser = Args::parse_duration)]
    pub long_break_duration: Option<Duration>,
}

impl Args {
    fn parse_duration(s: &str) -> Result<Duration, String> {
        let minutes: u64 = s.parse().map_err(|_| "Invalid duration".to_string())?;
        Ok(Duration::from_secs(minutes * 60))
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub pomodoro_duration: Option<u64>,
    pub short_break_duration: Option<u64>,
    pub long_break_duration: Option<u64>,
}

impl Config {
    pub fn load() -> Config {
        let path = dirs::config_dir().unwrap().join("oxitime/config.toml");
        let mut file = std::fs::File::open(path).unwrap();

        let mut input = String::new();
        file.read_to_string(&mut input).unwrap();

        let config: Config = toml::from_str(input.as_str()).unwrap();

        Config {
            pomodoro_duration: config.pomodoro_duration,
            short_break_duration: config.short_break_duration,
            long_break_duration: config.long_break_duration,
        }
    }
}

pub struct Opts {
    pub command: TimerCommand,
    pub pomodoro_duration: Duration,
    pub short_break_duration: Duration,
    pub long_break_duration: Duration,
}

impl Opts {
    pub fn build(args: Args, config: Config) -> Opts {
        Opts {
            command: args.command,
            pomodoro_duration: args.pomodoro_duration.unwrap_or(Duration::from_secs(
                config.pomodoro_duration.unwrap_or(25 * 60),
            )),
            short_break_duration: args.short_break_duration.unwrap_or(Duration::from_secs(
                config.short_break_duration.unwrap_or(5 * 60),
            )),
            long_break_duration: args.long_break_duration.unwrap_or(Duration::from_secs(
                config.long_break_duration.unwrap_or(15 * 60),
            )),
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

pub fn run(opts: Opts) -> Result<()> {
    let (tx, rx) = mpsc::channel::<TimerMessage>();

    match opts.command {
        TimerCommand::Start => {
            let mut timer = Timer::new(TimerKind::Pomodoro, opts.pomodoro_duration, rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        TimerCommand::ShortBreak => {
            let mut timer = Timer::new(TimerKind::ShortBreak, opts.short_break_duration, rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        TimerCommand::LongBreak => {
            let mut timer = Timer::new(TimerKind::LongBreak, opts.long_break_duration, rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        TimerCommand::Log => {
            todo!("Log!")
        }
    }

    let _ = enable_raw_mode();

    loop {
        match read() {
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('s'),
                ..
            })) => {
                let _ = tx.send(TimerMessage::ToggleStop);
            }
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('p'),
                ..
            })) => {
                let _ = tx.send(TimerMessage::TogglePause);
            }
            Ok(Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            })) => {
                break;
            }
            Err(error) => println!("{:?}", error),
            _ => (),
        }
    }

    let _ = execute!(
        std::io::stdout(),
        Clear(crossterm::terminal::ClearType::All),
        MoveTo(0, 0),
    );

    let _ = disable_raw_mode();

    Ok(())
}
