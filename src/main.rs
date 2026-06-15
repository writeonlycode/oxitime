use oxitime::run;
use anyhow::Result;

fn main() -> Result<()> {
    // Build configuration from command-line arguments and configuration file. Command-line
    // arguments take precedence over the configuration file.
    let config = oxitime::config::Config::load();

    if let Err(error) = run(config) {
        eprintln!("{}", error);
        std::process::exit(1);
    }

    Ok(())
}
