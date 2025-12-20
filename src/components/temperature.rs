use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};
use sysinfo::Components;

#[derive(Debug)]
pub struct Temperature {
    pub value: String,
    components: Components,
}

impl Temperature {
    pub fn new() -> Self {
        let components = Components::new();

        Self {
            value: "0".to_string(),
            components,
        }
    }

    pub fn update(&mut self) {
        self.components.refresh(true);

        if let Some(component) = self.components.iter().find(|c| {
            c.label().to_lowercase().contains("cpu")
                || c.label().to_lowercase().contains("core")
                || c.label().to_lowercase().contains("package")
        }) && let Some(temp) = component.temperature()
        {
            self.value = format!("{:.0}", temp);
        }
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let temp_icon = Span::raw(" ".to_owned());
        let temp_span = Span::raw(self.value.clone() + "°C");
        let space_span = Span::raw(" ");

        let temp_line = Line::from(vec![temp_icon, temp_span, space_span]);

        frame.render_widget(
            Paragraph::new(temp_line).right_aligned().fg(Color::White),
            area,
        );
    }
}

