use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{window_size, Clear},
};

pub struct TimerDisplay {}

impl TimerDisplay {
    pub fn print(print_location: DisplayPosition) {
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

pub struct DisplayPosition {
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

impl Default for DisplayPosition {
    fn default() -> Self {
        DisplayPosition {
            center: None,
            bottom_middle: None,
        }
    }
}
