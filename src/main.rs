use std::io;
use std::time::Duration;
use std::time::SystemTime;

use serde::Deserialize;
use crossterm::event::{self, Event, KeyEventKind};

use log::{debug, info, warn};

use issloc::App;
use issloc::tui;









fn main() -> Result<(), Box<dyn std::error::Error>> {

    info!("Initialized logging!");
    //Initialize TUI
    let mut terminal = tui::init()?;
    let app_result = App::new()?.run(&mut terminal);
    info!("App initialized!");
    
    Ok(app_result?)
}


