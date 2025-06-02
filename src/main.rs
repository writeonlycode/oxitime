use clap::Parser;
use oxitime::{run, Args, Config, Opts};

fn main() {
    let args = Args::parse();
    let config = Config::load();
    let opts = Opts::build(args, config);

    if let Err(error) = run(opts) {
        eprintln!("{}", error);
        std::process::exit(0);
    }
}
