# oxitime

Single-crate Rust CLI Pomodoro timer. No workspace, no tests, no CI, no rustfmt config.

## Build & run

```sh
cargo build
cargo check               # verify compilation
cargo run -- start        # run a pomodoro
cargo run -- short-break  # short break
cargo run -- long-break   # long break
```

## Keybindings (terminal app in raw mode)

`s` — start / stop, `p` — pause / resume, `q` — quit.

## System dependencies

On Linux, building requires the `alsa` development library (rodio dependency):

```sh
sudo apt install libasound2-dev   # Debian/Ubuntu
sudo dnf install alsa-lib-devel   # Fedora
```

## Config file

`~/.config/oxitime/config.toml` — if the file does not exist the program **panics** at startup. Create the file (even empty) to avoid this.

Duration fields use `humantime` format (e.g., `"30m"`, `"5m"`):

```toml
pomodoro_duration = "25m"
short_break_duration = "5m"
long_break_duration = "15m"
```

CLI flags override config file values:

```
cargo run -- start --pomodoro-duration 45m
```

## Architecture

Entrypoint: `src/main.rs` → `config::Config::load()` → `oxitime::run()`.

- `src/config/` — CLI parsing (`clap` derive) + TOML file loading
- `src/timer/` — timer loop, terminal display (`crossterm`), alarm sound (`rodio`), desktop notification (`notify-rust`)
- `assets/alarm.mp3` — embedded at compile time via `include_bytes!`
- `src/lib.rs` — orchestrates command dispatch and event loop

The `log` subcommand is a `todo!()` stub.
