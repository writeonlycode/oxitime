use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
};
use std::{
    sync::mpsc,
    thread::{self},
    time::Duration,
};
use timer::{Timer, TimerMessage};

mod timer;

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

pub fn run(config: Config) -> Result<()> {
    let (tx, rx) = mpsc::channel::<TimerMessage>();

    match config.command {
        Commands::Start => {
            let mut timer = Timer::new(Duration::from_secs(3), rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        Commands::Break => {
            let mut timer = Timer::new(Duration::from_secs(300), rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        Commands::LongBreak => {
            let mut timer = Timer::new(Duration::from_secs(900), rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        Commands::Log => {
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
