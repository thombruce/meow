#[derive(Debug)]
pub struct Sparkline {
    pub enabled: bool,
    pub length: usize,
    pub data: Vec<u64>,
}

impl Sparkline {
    pub fn new(enabled: bool, length: usize) -> Self {
        Self {
            enabled,
            length,
            data: vec![0; length],
        }
    }

    pub fn update(&mut self, value: u64) {
        if self.enabled {
            self.data.remove(0);
            self.data.push(value);
        }
    }

    pub fn render(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        let max_value = self.data.iter().max().unwrap_or(&1);
        if *max_value == 0 {
            return " ".repeat(self.length);
        }

        let bars = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
        let mut result = String::new();

        for &value in &self.data {
            if value == 0 {
                result.push(' ');
            } else {
                let index = ((value as f64 / *max_value as f64) * (bars.len() - 1) as f64) as usize;
                result.push(bars[index.min(bars.len() - 1)].chars().next().unwrap());
            }
        }

        result
    }

    pub fn render_with_spaces(&self) -> String {
        if self.enabled {
            self.render()
        } else {
            " ".repeat(self.length)
        }
    }
}
