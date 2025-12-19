use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};
use regex::Regex;
use std::process::Command;

static BRIGHTNESS_REGEX: std::sync::LazyLock<Regex> =
    std::sync::LazyLock::new(|| Regex::new(r"\d+%").unwrap());

#[derive(Debug)]
pub struct Brightness {
    pub level: String,
}

impl Brightness {
    pub fn new() -> Self {
        Self {
            level: get_system_brightness().unwrap_or_default(),
        }
    }

    pub fn update(&mut self) {
        self.level = get_system_brightness().unwrap_or_default();
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let brightness_icon = Span::raw("󰃠 ".to_owned());
        let brightness_span = Span::raw(self.level.clone());
        let space_span = Span::raw(" ");

        let brightness_line = Line::from(vec![brightness_icon, brightness_span, space_span]);

        frame.render_widget(
            Paragraph::new(brightness_line)
                .right_aligned()
                .fg(Color::White),
            area,
        );
    }
}

fn get_system_brightness() -> Option<String> {
    let output = Command::new("brightnessctl")
        .output()
        .expect("failed to get brightness");

    if output.status.success() {
        let brightness_str = str::from_utf8(&output.stdout).unwrap();

        let re = &BRIGHTNESS_REGEX;

        if let Some(brightness) = re.find(brightness_str).map(|m| m.as_str()) {
            return Some(brightness.to_string());
        }

        eprintln!("Failed to parse brightness from output: {}", brightness_str);
    } else {
        eprintln!(
            "Error: {}",
            str::from_utf8(&output.stderr).unwrap_or("unknown error")
        );
    }

    None
}