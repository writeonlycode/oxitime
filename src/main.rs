use clap::Parser;
use oxitime::{Commands::*, Config};

fn main() {
    let config = Config::parse();

    match config.command {
        Start => {
            todo!("Start!")
        }
        Break => {
            todo!("Break!")
        }
        LongBreak => {
            todo!("LongBreak!")
        }
        Pause => {
            todo!("Pause!")
        }
        Resume => {
            todo!("Pause!")
        }
        Stop => {
            todo!("Stop!")
        }
        Status => {
            todo!("Status!")
        }
        Log => {
            todo!("Log!")
        }
    }
}
