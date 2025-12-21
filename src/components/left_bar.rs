use crate::components::Workspaces;
use ratatui::{Frame, prelude::Stylize, style::Color, text::Line, widgets::Paragraph};

#[derive(Debug)]
pub struct LeftBar {
    workspaces: Workspaces,
}

impl LeftBar {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            workspaces: Workspaces::new(),
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        self.workspaces.update();
        Ok(())
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let spans = self.workspaces.workspace_spans();

        let left_line = Line::from(spans);

        frame.render_widget(
            Paragraph::new(left_line).left_aligned().fg(Color::White),
            area,
        );
    }
}
