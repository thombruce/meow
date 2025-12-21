use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Direction, Layout},
    prelude::Constraint,
};
use std::time::Duration;
use tokio::runtime::Runtime;

mod components;
use components::{LeftBar, MiddleBar, RightBar};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Initialize Tokio runtime
    let rt = Runtime::new()?;

    rt.block_on(async {
        let terminal = ratatui::init();
        let result = App::new()?.run_async(terminal).await;
        ratatui::restore();
        result
    })
}

/// The main application which holds the state and logic of the application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    running: bool,
    left_bar: LeftBar,
    middle_bar: MiddleBar,
    right_bar: RightBar,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            running: true,
            left_bar: LeftBar::new()?,
            middle_bar: MiddleBar::new()?,
            right_bar: RightBar::new()?,
        })
    }

    /// Run the application's main loop.
    pub async fn run_async(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            self.update_components();
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn update_components(&mut self) {
        if let Err(e) = self.left_bar.update() {
            eprintln!("Error updating middle bar: {}", e);
        }
        if let Err(e) = self.middle_bar.update() {
            eprintln!("Error updating middle bar: {}", e);
        }
        if let Err(e) = self.right_bar.update() {
            eprintln!("Error updating right bar: {}", e);
        }
    }

    /// Renders the user interface.
    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(frame.area());

        self.left_bar.render(frame, layout[0]);
        self.middle_bar.render(frame, layout[1]);
        self.right_bar.render(frame, layout[2]);
    }

    /// Reads the crossterm events and updates the state of [`App`].
    fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        if event::poll(Duration::from_millis(333))? {
            match event::read()? {
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
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
