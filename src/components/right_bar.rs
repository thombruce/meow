use crate::component_manager::ComponentManager;
use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Debug)]
pub struct RightBar {
    component_manager: ComponentManager,
}

impl RightBar {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            component_manager: ComponentManager::new()?,
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        self.component_manager.update()
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let components = self.component_manager.get_bar_components("right");

        if components.is_empty() {
            return;
        }

        let spans: Vec<Span> = components
            .iter()
            .map(|component| {
                let content = component.render();
                if component.is_muted() {
                    Span::raw(content).fg(Color::DarkGray)
                } else {
                    Span::raw(content)
                }
            })
            .collect();

        let right_line = Line::from(spans);

        frame.render_widget(
            Paragraph::new(right_line).right_aligned().fg(Color::White),
            area,
        );
    }
}
