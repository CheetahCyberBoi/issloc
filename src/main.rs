use log::info;

<<<<<<< HEAD
use serde::Deserialize;
use crossterm::event::{self, Event, KeyEventKind};
use crate::tui::Action;
use crate::timer::Timer;
use log::{debug, info, warn};
=======
use issloc::app::App;
use issloc::tui;
>>>>>>> 3e8c105cf8c8889c34ce20b57d5b9739f27a5959









fn main() -> Result<(), Box<dyn std::error::Error>> {

    info!("Initialized logging!");
    //Initialize TUI
    let mut terminal = tui::init()?;
    let app_result = App::new()?.run(&mut terminal);
    info!("App initialized!");
    
    Ok(app_result?)
}


