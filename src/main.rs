use std::{
    sync::mpsc,
    thread::{self},
    time::Duration,
};

use clap::Parser;
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
};
use oxitime::{Commands::*, Config};
use timer::{Timer, TimerMessage};

mod timer;

fn main() {
    let config = Config::parse();
    let (tx, rx) = mpsc::channel::<TimerMessage>();

    match config.command {
        Start => {
            let mut timer = Timer::new(Duration::from_secs(3), rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        Break => {
            let mut timer = Timer::new(Duration::from_secs(300), rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        LongBreak => {
            let mut timer = Timer::new(Duration::from_secs(900), rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        Log => {
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
}
