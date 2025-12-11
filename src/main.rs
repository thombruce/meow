// NOTE: On how to think about the structure of a ratatui application...
// We have got:
// 1. State (see pub struct App and initial run setup)
// 2. Controller/mutations (see the while loop which also calls the draw function)
// 3. Renderer (see fn render)
// When we think about separating this out into modules... a naive approach might be to
// take a sort of single file component approach, with each component defining its own
// needs and therefore having many separate loops. Better would be to handle state more
// effectively (think like Pinia in Nuxt). Ratatui in fact has a suggested means for
// appropriately handling state that I should look at.

use chrono::Local;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Direction, Layout},
    prelude::Constraint,
    style::{Color, Stylize},
    widgets::Paragraph,
};
use std::{io, time::Duration};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default, Clone)]
pub struct App {
    /// Is the application running?
    running: bool,
    time: String,
    bat_percent: String,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;

        // TODO: Move me! Ideally each widget should be moved into its own file and assembled here.
        // TODO: Document me! We need better comments to describe what's going on. This doesn't get
        // the battery state at all, for instance, it just gets the first found instance of a
        // battery.
        let manager = battery::Manager::new()?;
        let mut battery = match manager.batteries()?.next() {
            Some(Ok(battery)) => battery,
            Some(Err(e)) => {
                eprintln!("Unable to access battery information");
                return Err(e.into());
            }
            None => {
                eprintln!("Unable to find any batteries");
                return Err(io::Error::from(io::ErrorKind::NotFound).into());
            }
        };

        while self.running {
            self.time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            self.bat_percent = ((battery.state_of_charge().value * 100.0) as i32).to_string();
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
            manager.refresh(&mut battery)?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(frame.area());

        let text = "=^,^=";

        frame.render_widget(
            Paragraph::new("󰁹 ".to_owned() + &self.bat_percent + "%")
                .left_aligned()
                .fg(Color::White),
            layout[0],
        );

        frame.render_widget(Paragraph::new(text).centered().fg(Color::White), layout[1]);

        frame.render_widget(
            Paragraph::new(self.time.clone())
                .right_aligned()
                // .fg(Color::Rgb(189, 43, 174)),
                .fg(Color::White),
            layout[2],
        );
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(500))? {
            match event::read()? {
                // it's important to check KeyEventKind::Press to avoid handling key release events
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
