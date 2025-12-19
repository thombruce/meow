use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

#[derive(Debug)]
pub struct Cpu {
    pub usage: String,
    system: System,
}

impl Cpu {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        
        Self {
            usage: "0".to_string(),
            system,
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_cpu_all();

        let iter = self.system.cpus().iter();
        let count = iter.len() as f32;
        let sum = iter.fold(0.0, |acc, x| acc + x.cpu_usage());
        let avg: u32 = (sum / count) as u32;
        self.usage = avg.to_string();
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let cpu_icon = Span::raw("󰻠 ".to_owned());
        let cpu_span = Span::raw(self.usage.clone() + "%");

        let cpu_line = Line::from(vec![cpu_icon, cpu_span]);

        frame.render_widget(
            Paragraph::new(cpu_line)
                .right_aligned()
                .fg(Color::White),
            area,
        );
    }
}