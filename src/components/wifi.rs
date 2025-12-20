use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};
use std::process::Command;

#[derive(Debug)]
pub struct Wifi {
    pub status: String,
    pub network: String,
}

impl Wifi {
    pub fn new() -> Self {
        let (status, network) =
            get_wifi_status().unwrap_or(("disconnected".to_string(), "".to_string()));

        Self { status, network }
    }

    pub fn update(&mut self) {
        if let Some((status, network)) = get_wifi_status() {
            self.status = status;
            self.network = network;
        }
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let wifi_icon = if self.status == "connected" {
            "󰤨 "
        } else {
            "󰤮 "
        };

        let wifi_text = if self.status == "connected" && !self.network.is_empty() {
            &self.network
        } else {
            "Off"
        };

        let wifi_span = Span::raw(wifi_icon.to_owned());
        let network_span = Span::raw(wifi_text.to_string());

        let wifi_line = Line::from(vec![wifi_span, network_span]);

        frame.render_widget(
            Paragraph::new(wifi_line).right_aligned().fg(Color::White),
            area,
        );
    }
}

fn get_wifi_status() -> Option<(String, String)> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "TYPE,STATE,CONNECTION", "device"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = str::from_utf8(&output.stdout).ok()?;

    for line in stdout.lines() {
        if line.starts_with("wifi:") {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                let state = parts[1].to_lowercase();
                let connection = parts[2].to_string();

                if state == "connected" {
                    return Some(("connected".to_string(), connection));
                } else {
                    return Some(("disconnected".to_string(), "".to_string()));
                }
            }
        }
    }

    Some(("disconnected".to_string(), "".to_string()))
}

