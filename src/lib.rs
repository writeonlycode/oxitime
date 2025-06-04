use anyhow::Result;
use config::{Config, TimerCommand};
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
};
use std::{
    sync::mpsc::{self, Sender},
    thread::{self},
};
use timer::{Timer, TimerKind, TimerMessage};

pub mod config;
mod timer;

pub fn run(config: Config) -> Result<()> {
    let (tx, rx) = mpsc::channel::<TimerMessage>();

    process_command(&config, &tx, rx);

    let _ = enable_raw_mode();

    process_events(tx);

    let _ = execute!(
        std::io::stdout(),
        Clear(crossterm::terminal::ClearType::All),
        MoveTo(0, 0),
    );

    let _ = disable_raw_mode();

    Ok(())
}

fn process_command(config: &Config, tx: &Sender<TimerMessage>, rx: mpsc::Receiver<TimerMessage>) {
    match config.command {
        TimerCommand::Start => {
            let mut timer = Timer::new(TimerKind::Pomodoro, config.pomodoro_duration, rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        TimerCommand::ShortBreak => {
            let mut timer = Timer::new(TimerKind::ShortBreak, config.short_break_duration, rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        TimerCommand::LongBreak => {
            let mut timer = Timer::new(TimerKind::LongBreak, config.long_break_duration, rx);

            thread::spawn(move || {
                timer.run();
            });

            let _ = tx.send(TimerMessage::Start);
        }
        TimerCommand::Log => {
            todo!("Log!")
        }
    }
}

fn process_events(tx: Sender<TimerMessage>) {
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
}
