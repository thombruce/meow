#[derive(Debug, Default, Clone)]
pub struct Space;

impl Space {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self) -> String {
        " ".to_string()
    }
}
