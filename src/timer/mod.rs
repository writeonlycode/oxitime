use std::{
    sync::mpsc::Receiver,
    thread::sleep,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{window_size, Clear},
};

pub struct Timer {
    duration: Duration,
    start_time: Option<Instant>,
    elapsed_time: Duration,
    receiver: Receiver<TimerMessage>,
    state: TimerState,
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

struct PrintLocation {
    pub center: Option<String>,
    pub bottom_middle: Option<String>,
    //pub top: Option<String>,
    //pub bottom: Option<String>,
    //pub left: Option<String>,
    //pub right: Option<String>,
    //pub top_left: Option<String>,
    //pub top_right: Option<String>,
    //pub bottom_left: Option<String>,
    //pub bottom_right: Option<String>,
}

impl Default for PrintLocation {
    fn default() -> Self {
        PrintLocation {
            center: None,
            bottom_middle: None,
        }
    }
}

impl Timer {
    pub fn new(duration: Duration, receiver: Receiver<TimerMessage>) -> Timer {
        Timer {
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

            match self.state {
                TimerState::Running => {
                    if !self.remaining_time().is_zero() {
                        Timer::print(PrintLocation {
                            center: Some(self.remaining_time().as_secs_f32().ceil().to_string()),
                            ..PrintLocation::default()
                        });

                        self.elapsed_time += Duration::from_millis(250);
                        sleep(Duration::from_millis(250));
                    } else {
                        Timer::print(PrintLocation {
                            center: Some(self.remaining_time().as_secs_f32().ceil().to_string()),
                            ..PrintLocation::default()
                        });
                        self.finish();
                    }
                }
                TimerState::Paused => {
                    Timer::print(PrintLocation {
                        center: Some(self.remaining_time().as_secs_f32().ceil().to_string()),
                        bottom_middle: Some(
                            "The pomodoro is paused. Press P to resume.".to_string(),
                        ),
                        ..PrintLocation::default()
                    });
                    sleep(Duration::from_millis(250));
                }
                TimerState::Stopped => {
                    Timer::print(PrintLocation {
                        center: Some(self.remaining_time().as_secs_f64().ceil().to_string()),
                        bottom_middle: Some(
                            "The pomodoro has been stopped and reset. Press S to start again."
                                .to_string(),
                        ),
                        ..PrintLocation::default()
                    });
                    sleep(Duration::from_millis(250));
                }
                TimerState::Finished => {
                    Timer::print(PrintLocation {
                        center: Some(self.remaining_time().as_secs_f64().ceil().to_string()),
                        bottom_middle: Some(
                            "The pomodoro has finished! Press Q to quit.".to_string(),
                        ),
                        ..PrintLocation::default()
                    });
                    let _ = execute!(std::io::stdout(), Print("\x07"));
                    break;
                }
            }
        }
    }

    pub fn remaining_time(&self) -> Duration {
        self.duration - self.elapsed_time
    }

    fn print(print_location: PrintLocation) {
        let _ = execute!(
            std::io::stdout(),
            Clear(crossterm::terminal::ClearType::All),
        );

        if let Some(s) = print_location.center {
            let x_center = (window_size().unwrap().columns / 2) - s.len() as u16 / 2;
            let y_center = window_size().unwrap().rows / 2;

            let _ = execute!(std::io::stdout(), MoveTo(x_center, y_center), Print(s));
        }

        if let Some(s) = print_location.bottom_middle {
            let x_center = (window_size().unwrap().columns / 2) - s.len() as u16 / 2;
            let y_center = window_size().unwrap().rows / 2;
            let y_center = y_center + (y_center / 2);

            let _ = execute!(std::io::stdout(), MoveTo(x_center, y_center), Print(s));
        }
    }
}
