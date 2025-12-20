use chrono::Local;

#[derive(Debug, Default, Clone)]
pub struct Time {
    pub time_string: String,
}

impl Time {
    pub fn new() -> Self {
        Self {
            time_string: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }

    pub fn update(&mut self) {
        self.time_string = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }
}
