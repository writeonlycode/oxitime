use std::{
    sync::mpsc::Receiver,
    thread::sleep,
    time::{Duration, Instant},
};

use notify_rust::{Notification, Timeout};
use timer_alarm::TimerAlarm;
use timer_display::{DisplayPosition, TimerDisplay};

mod timer_alarm;
mod timer_display;

pub struct Timer {
    kind: TimerKind,
    duration: Duration,
    start_time: Option<Instant>,
    elapsed_time: Duration,
    receiver: Receiver<TimerMessage>,
    state: TimerState,
}

pub enum TimerKind {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

pub enum TimerMessage {
    Start,
    ToggleStop,
    TogglePause,
}

enum TimerState {
    Stopped,
    Running,
    Paused,
    Finished,
}

impl Timer {
    pub fn new(kind: TimerKind, duration: Duration, receiver: Receiver<TimerMessage>) -> Timer {
        Timer {
            kind,
            duration,
            start_time: None,
            elapsed_time: Duration::ZERO,
            receiver,
            state: TimerState::Stopped,
        }
    }

    pub fn start(&mut self) {
        if let TimerState::Stopped = self.state {
            self.start_time = Some(Instant::now());
            self.state = TimerState::Running;
        }
    }

    pub fn pause(&mut self) {
        if let TimerState::Running = self.state {
            self.state = TimerState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if let TimerState::Paused = self.state {
            self.state = TimerState::Running;
        }
    }

    pub fn stop(&mut self) {
        if let TimerState::Running | TimerState::Paused = self.state {
            self.start_time = None;
            self.elapsed_time = Duration::ZERO;
            self.state = TimerState::Stopped;
        }
    }

    pub fn finish(&mut self) {
        // Sync with external services
        //sync::sync().unwrap();

        //Notification
        let (title, body) = match self.kind {
            TimerKind::Pomodoro => (
                "Pomodoro Session Finished",
                "You've completed your focus session. Take a short break!",
            ),
            TimerKind::ShortBreak => (
                "Short Break  Finished",
                "You've completed your short break. Start a new pomodoro!",
            ),
            TimerKind::LongBreak => (
                "Pomodoro Session Finished",
                "You've completed your long break. Start a new pomodoro!",
            ),
        };

        Notification::new()
            .summary(title)
            .body(body)
            .timeout(Timeout::Milliseconds(10000))
            .show()
            .unwrap();

        if let TimerState::Running = self.state {
            self.state = TimerState::Finished;
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(timer_message) = self.receiver.try_recv() {
                match timer_message {
                    TimerMessage::Start => self.start(),
                    TimerMessage::ToggleStop => match self.state {
                        TimerState::Running => self.stop(),
                        TimerState::Stopped => self.start(),
                        _ => (),
                    },
                    TimerMessage::TogglePause => match self.state {
                        TimerState::Running => self.pause(),
                        TimerState::Paused => self.resume(),
                        _ => (),
                    },
                }
            }

            let remaining_time =
                humantime::format_duration(Duration::from_secs(self.remaining_time().as_secs()));

            match self.state {
                TimerState::Running => {
                    if !self.remaining_time().is_zero() {
                        TimerDisplay::print(DisplayPosition {
                            center: Some(remaining_time.to_string()),
                            ..DisplayPosition::default()
                        });

                        self.elapsed_time += Duration::from_millis(250);
                        sleep(Duration::from_millis(250));
                    } else {
                        TimerDisplay::print(DisplayPosition {
                            center: Some(remaining_time.to_string()),
                            ..DisplayPosition::default()
                        });
                        self.finish();
                    }
                }
                TimerState::Paused => {
                    TimerDisplay::print(DisplayPosition {
                        center: Some(remaining_time.to_string()),
                        bottom_middle: Some(
                            "The pomodoro is paused. Press P to resume.".to_string(),
                        ),
                        ..DisplayPosition::default()
                    });
                    sleep(Duration::from_millis(250));
                }
                TimerState::Stopped => {
                    TimerDisplay::print(DisplayPosition {
                        center: Some(remaining_time.to_string()),
                        bottom_middle: Some(
                            "The pomodoro has been stopped and reset. Press S to start again."
                                .to_string(),
                        ),
                        ..DisplayPosition::default()
                    });
                    sleep(Duration::from_millis(250));
                }
                TimerState::Finished => {
                    TimerDisplay::print(DisplayPosition {
                        center: Some(remaining_time.to_string()),
                        bottom_middle: Some(
                            "The pomodoro has finished! Press Q to quit.".to_string(),
                        ),
                        ..DisplayPosition::default()
                    });

                    let _ = TimerAlarm::play();

                    break;
                }
            }
        }
    }

    fn remaining_time(&self) -> Duration {
        self.duration - self.elapsed_time
    }
}
