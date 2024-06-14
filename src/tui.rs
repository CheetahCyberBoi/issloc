use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::*};
use crossterm::event::*;

use ratatui::prelude::*;

//A gerneral TUI type that is used througought.
pub type Tui = Terminal<CrosstermBackend<Stdout>>;


// Initialize said terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

// Restore the terminal back to the host OS
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

//Stores the actions that the program can take
pub enum Action {
    PingRESTAPI,
    IncreaseDelayTime,
    DecreaseDelayTime,
    Exit,
    Error, //DO NOT USE
}

impl Action {
    pub fn from_keypress(key_event: crossterm::event::KeyEvent) -> Action {
        match key_event.code {
            KeyCode::Char('q') => Action::Exit,
            KeyCode::Left => Action::DecreaseDelayTime,
            KeyCode::Right => Action::IncreaseDelayTime,
            KeyCode::Enter => Action::PingRESTAPI,
            _ => Action::Error
        }
    }
}

