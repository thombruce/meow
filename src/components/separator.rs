#[derive(Debug, Default, Clone)]
pub struct Separator {
    pub separator: String,
}

impl Separator {
    pub fn new() -> Self {
        Self::with_separator(" | ")
    }

    pub fn with_separator(separator: &str) -> Self {
        Self {
            separator: separator.to_string(),
        }
    }

    pub fn render(&self) -> String {
        self.separator.clone()
    }
}
