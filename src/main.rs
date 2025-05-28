use clap::Parser;
use oxitime::{run, Config};

fn main() {
    let config = Config::parse();

    if let Err(error) = run(config) {
        eprintln!("{}", error);
        std::process::exit(0);
    }
}
