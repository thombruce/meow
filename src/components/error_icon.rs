#[derive(Debug, Default, Clone)]
pub struct ErrorIcon;

impl ErrorIcon {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self) -> String {
        "  ".to_string()
    }

    pub fn render_as_spans(&self) -> Vec<ratatui::text::Span<'_>> {
        vec![ratatui::text::Span::styled(
            self.render(),
            ratatui::style::Style::default(), // .fg(ratatui::style::Color::Yellow),
        )]
    }
}
