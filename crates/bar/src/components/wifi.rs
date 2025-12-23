use crate::components::ConfigurableComponent;
use ratatui::{prelude::Stylize, style::Color, text::Span};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiConfig {
    pub component: String,
    #[serde(default)]
    pub sparkline: bool,
    #[serde(default = "default_sparkline_length")]
    pub sparkline_length: usize,
    #[serde(default = "default_update_interval")]
    pub update_interval: u64,
}

fn default_sparkline_length() -> usize {
    20
}

fn default_update_interval() -> u64 {
    2
}

impl ConfigurableComponent for Wifi {
    type Config = WifiConfig;

    fn from_config(config: Self::Config) -> color_eyre::Result<Self> {
        let mut wifi = Self::new();
        if config.sparkline {
            wifi.enable_sparkline(config.sparkline_length, config.update_interval);
        }
        Ok(wifi)
    }

    fn component_name() -> &'static str {
        "wifi"
    }
}

#[derive(Debug)]
pub struct Wifi {
    pub status: String,
    pub network: String,
    last_update: Instant,
    update_interval: Duration,
    sparkline_mode: bool,
    sparkline_length: usize,
    network_usage: Vec<u64>,
    last_total_bytes: Option<u64>,
}

impl Wifi {
    pub fn new() -> Self {
        let (status, network) =
            get_wifi_status().unwrap_or(("disconnected".to_string(), "".to_string()));

        Self {
            status,
            network,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(2),
            sparkline_mode: false,
            sparkline_length: 20,
            network_usage: Vec::new(),
            last_total_bytes: None,
        }
    }

    pub fn enable_sparkline(&mut self, length: usize, update_interval_sec: u64) {
        self.sparkline_mode = true;
        self.sparkline_length = length;
        self.update_interval = Duration::from_secs(update_interval_sec);
        self.network_usage = vec![0; length];
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            if let Some((status, network)) = get_wifi_status() {
                self.status = status;
                self.network = network;
            }

            if self.sparkline_mode && self.status == "connected" {
                self.update_network_usage();
            }

            self.last_update = now;
        }
    }

    pub fn render(&self) -> String {
        let icon = if self.status == "connected" {
            "󰤨"
        } else {
            "󰤮"
        };

        let network_text = if self.status == "connected" && !self.network.is_empty() {
            &self.network
        } else {
            "Off"
        };

        format!("{} {}", icon, network_text)
    }

    pub fn render_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        if self.sparkline_mode {
            return self.render_sparkline_as_spans(colorize);
        }

        let span = Span::raw(self.render());
        if colorize {
            let color = if self.status == "disconnected" {
                Color::Red // Disconnected: Red
            } else {
                Color::Blue // Connected: Blue
            };
            vec![span.fg(color)]
        } else {
            vec![span]
        }
    }

    fn update_network_usage(&mut self) {
        if let Some((bytes_rx, bytes_tx)) = get_network_stats() {
            let current_total = bytes_rx + bytes_tx;

            if let Some(last_total) = self.last_total_bytes {
                let usage = current_total.saturating_sub(last_total);
                self.network_usage.push(usage);
                if self.network_usage.len() > self.sparkline_length {
                    self.network_usage.remove(0);
                }
            }

            self.last_total_bytes = Some(current_total);
        }
    }

    fn render_sparkline_as_spans(&self, colorize: bool) -> Vec<Span<'_>> {
        let max_usage = self.network_usage.iter().max().copied().unwrap_or(1);
        let normalized_usage: Vec<u64> = self
            .network_usage
            .iter()
            .map(|&usage| {
                if max_usage > 0 {
                    usage * 8 / max_usage.max(1)
                } else {
                    0
                }
            })
            .collect();

        let sparkline_chars: String = normalized_usage
            .iter()
            .map(|&usage| match usage {
                0 => ' ',
                1 => '▁',
                2 => '▂',
                3 => '▃',
                4 => '▄',
                5 => '▅',
                6 => '▆',
                7 => '▇',
                _ => '█',
            })
            .collect();

        let icon = if self.status == "connected" {
            "󰤨"
        } else {
            "󰤮"
        };

        let text = format!("{} {}", icon, sparkline_chars);
        let span = Span::raw(text);

        if colorize {
            let color = if self.status == "disconnected" {
                Color::Red
            } else {
                Color::Blue
            };
            vec![span.fg(color)]
        } else {
            vec![span]
        }
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

fn get_network_stats() -> Option<(u64, u64)> {
    let content = std::fs::read_to_string("/proc/net/dev").ok()?;

    for line in content.lines() {
        if line.contains("wlan") || line.contains("wl") || line.contains("wlp") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 {
                let bytes_rx = parts[1].parse::<u64>().ok()?;
                let bytes_tx = parts[9].parse::<u64>().ok()?;
                return Some((bytes_rx, bytes_tx));
            }
        }
    }

    None
}
