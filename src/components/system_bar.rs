use crate::components::{Battery, Brightness, Cpu, Ram, Temperature, Volume, Wifi};
use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Debug)]
pub struct SystemBar {
    temperature: Temperature,
    cpu: Cpu,
    ram: Ram,
    wifi: Wifi,
    brightness: Brightness,
    volume: Volume,
    battery: Battery,
}

impl SystemBar {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            temperature: Temperature::new(),
            cpu: Cpu::new(),
            ram: Ram::new(),
            wifi: Wifi::new(),
            brightness: Brightness::new(),
            volume: Volume::new(),
            battery: Battery::new()?,
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        self.temperature.update();
        self.cpu.update();
        self.ram.update();
        self.wifi.update();
        self.brightness.update();
        self.volume.update();
        self.battery.update()?;
        Ok(())
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let spans = vec![
            Span::raw(" "),
            Span::raw(self.temperature.value.clone() + "°C"),
            Span::raw(" "),
            Span::raw("󰻠 "),
            Span::raw(self.cpu.usage.clone() + "%"),
            Span::raw(" "),
            Span::raw("󰍛 "),
            Span::raw(self.ram.usage.clone() + "%"),
            Span::raw(" | "),
            Span::raw(if self.wifi.status == "connected" {
                "󰤨 "
            } else {
                "󰤮 "
            }),
            Span::raw(
                if self.wifi.status == "connected" && !self.wifi.network.is_empty() {
                    &self.wifi.network
                } else {
                    "Off"
                }
                .to_string(),
            ),
            Span::raw(" | "),
            Span::raw("󰃠 "),
            Span::raw(self.brightness.level.clone()),
            Span::raw(" "),
            Span::raw("󰕾 "),
            Span::raw(self.volume.level.clone() + "%"),
            Span::raw(" | "),
            Span::raw("󰁹 "),
            Span::raw(self.battery.percentage.clone() + "%"),
        ];

        let system_line = Line::from(spans);

        frame.render_widget(
            Paragraph::new(system_line).right_aligned().fg(Color::White),
            area,
        );
    }
}
