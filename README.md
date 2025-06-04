# Oxitime 🦀⏱️

**Oxitime** is a fast, minimal, and terminal-native Pomodoro time tracker built
with the Rust programming language. Stay focused, beat procrastination, and
reclaim your time — one oxidized session at a time.

## Features

- 🍅 Pomodoro technique (25/5 intervals)
- ⏱️ Command-line interface (lightning fast)
- 🔔 Notifications (desktop alerts)
- ⚙️ Configurable session durations
- 🦀 Blazingly fast and portable (Rust-powered)

## Installation

Coming soon: Precompiled binaries and `cargo install`.

For now, clone and build manually:

```bash
git clone https://github.com/yourusername/oxitime.git
cd oxitime
cargo build --release
```

Then run it from target/release/oxitime.

## Usage

```bash
oxitime start        # Start a Pomodoro session
oxitime short-break  # Start a short break
oxitime long-break   # Start a long break
```

## Configuration

Customize durations and preferences with a simple config file at
~/.config/oxitime/config.toml:

```toml
pomodoro_duration = "30m" 
short_break_duration = "5m" 
long_break_duration = "30m" 
```

## Roadmap

- ✅ Basic Pomodoro timer
- ✅ Desktop notifications
- 🔜 Session persistence & logs
- 🔜 Stats and productivity graphs

## Contributing

Pull requests are welcome! If you find a bug or want a new feature, open an issue.

## License

MIT License © Iago Bozza

## Thanks

- Alarm Sound Effect by <a href="https://pixabay.com/users/u_inx5oo5fv3-49729779/?utm_source=link-attribution&utm_medium=referral&utm_campaign=music&utm_content=327234">u_inx5oo5fv3</a> from <a href="https://pixabay.com/sound-effects//?utm_source=link-attribution&utm_medium=referral&utm_campaign=music&utm_content=327234">Pixabay</a>.
