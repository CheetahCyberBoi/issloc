use std::io::{self, stdout, Stdout};

use crossterm::{execute, terminal::*};
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

