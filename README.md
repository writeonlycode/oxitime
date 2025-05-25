# Oxitime 🦀⏱️

**Oxitime** is a fast, minimal, and terminal-native Pomodoro time tracker built
with the Rust programming language. Stay focused, beat procrastination, and
reclaim your time — one oxidized session at a time.

## Features

- 🍅 Pomodoro technique (25/5 intervals)
- ⏱️ Command-line interface (lightning fast)
- 🔔 Notifications (optional desktop alerts)
- 📊 Time summaries (see where your focus went)
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
oxitime short-break        # Start a short break
oxitime long-break   # Start a long break
oxitime log          # View session history
```

## Configuration

Customize durations and preferences with a simple config file at
~/.config/oxitime/config.toml:

```toml
[pomodoro]
work_minutes = 25
short_break = 5
long_break = 15
sessions_before_long_break = 4
```

## Roadmap

- ✅ Basic Pomodoro timer
- 🔜 Desktop notifications
- 🔜 Session persistence & logs
- 🔜 Stats and productivity graphs
- 🔜 TUI mode (text UI)

## Contributing

Pull requests are welcome! If you find a bug or want a new feature, open an issue.

## License

MIT License © Iago Bozza

## Thanks

- Alarm Sound Effect by <a href="https://pixabay.com/users/u_inx5oo5fv3-49729779/?utm_source=link-attribution&utm_medium=referral&utm_campaign=music&utm_content=327234">u_inx5oo5fv3</a> from <a href="https://pixabay.com/sound-effects//?utm_source=link-attribution&utm_medium=referral&utm_campaign=music&utm_content=327234">Pixabay</a>.
