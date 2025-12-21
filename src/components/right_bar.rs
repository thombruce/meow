use crate::components::{Battery, Brightness, Cpu, Ram, Temperature, Volume, Vpn, Wifi};
use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Debug)]
pub struct RightBar {
    temperature: Temperature,
    cpu: Cpu,
    ram: Ram,
    wifi: Wifi,
    vpn: Vpn,
    brightness: Brightness,
    volume: Volume,
    battery: Battery,
}

impl RightBar {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            temperature: Temperature::new(),
            cpu: Cpu::new(),
            ram: Ram::new(),
            wifi: Wifi::new(),
            vpn: Vpn::new(),
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
        self.vpn.update();
        self.brightness.update();
        self.volume.update();
        self.battery.update()?;
        Ok(())
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let spans = vec![
            Span::raw(self.temperature.render()),
            Span::raw(" "),
            Span::raw(self.cpu.render()),
            Span::raw(" "),
            Span::raw(self.ram.render()),
            Span::raw(" | "),
            Span::raw(self.wifi.render()),
            Span::raw(" "),
            Span::raw(self.vpn.render()),
            Span::raw(" | "),
            Span::raw(self.brightness.render()),
            Span::raw(" "),
            if self.volume.is_muted {
                Span::raw(self.volume.render()).fg(Color::DarkGray)
            } else {
                Span::raw(self.volume.render())
            },
            Span::raw(" | "),
            Span::raw(self.battery.render()),
        ];

        let right_line = Line::from(spans);

        frame.render_widget(
            Paragraph::new(right_line).right_aligned().fg(Color::White),
            area,
        );
    }
}
