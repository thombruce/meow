use ratatui::{
    Frame,
    prelude::Stylize,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};

#[derive(Debug)]
pub struct Battery {
    pub percentage: String,
    battery_manager: battery::Manager,
    battery: battery::Battery,
}

impl Battery {
    pub fn new() -> color_eyre::Result<Self> {
        let manager = battery::Manager::new()?;
        let battery = match manager.batteries()?.next() {
            Some(Ok(battery)) => battery,
            Some(Err(e)) => {
                eprintln!("Unable to access battery information");
                return Err(e.into());
            }
            None => {
                eprintln!("Unable to find any batteries");
                return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
            }
        };

        Ok(Self {
            percentage: ((battery.state_of_charge().value * 100.0) as i32).to_string(),
            battery_manager: manager,
            battery,
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        self.battery_manager.refresh(&mut self.battery)?;
        self.percentage = ((self.battery.state_of_charge().value * 100.0) as i32).to_string();
        Ok(())
    }

    pub fn render(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        let bat_icon = Span::raw("󰁹 ".to_owned());
        let bat_span = Span::raw(self.percentage.clone() + "%");

        let battery_line = Line::from(vec![bat_icon, bat_span]);

        frame.render_widget(
            Paragraph::new(battery_line)
                .right_aligned()
                .fg(Color::White),
            area,
        );
    }
}