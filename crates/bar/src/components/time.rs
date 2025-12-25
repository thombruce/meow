use chrono::{Local, Timelike};
use ratatui::{prelude::Stylize, style::Color, text::Span};

#[derive(Debug, Default, Clone)]
pub struct Time {
    pub time_string: String,
    pub cached_span_content: String,
}

impl Time {
    pub fn new() -> Self {
        let time_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            time_string: time_string.clone(),
            cached_span_content: time_string,
        }
    }

    pub fn update(&mut self) {
        self.time_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.cached_span_content = self.time_string.clone();
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let span = Span::raw(&self.cached_span_content);
        if colorize {
            let hour = Local::now().hour();
            let color = if (6..18).contains(&hour) {
                Color::Yellow // Daytime (6:00 - 17:59): Yellow
            } else {
                Color::Magenta // Nighttime (18:00 - 5:59): Purple
            };
            vec![span.fg(color)]
        } else {
            vec![span]
        }
    }
}
