use log::info;

use issloc::app::App;
use issloc::tui;









fn main() -> Result<(), Box<dyn std::error::Error>> {

    info!("Initialized logging!");
    //Initialize TUI
    let mut terminal = tui::init()?;
    let app_result = App::new()?.run(&mut terminal);
    info!("App initialized!");
    
    Ok(app_result?)
}


