use std::io;

use serde::Deserialize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crate::tui::Action;


pub mod ui;
pub mod tui;
#[derive(Deserialize, Debug, Default)]
pub struct IssData {
    pub name: String,
    pub id: u32,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub velocity: f64,
    pub visibility: String,
    pub footprint: f64,
    pub timestamp: u64,
    pub daynum: f64,
    pub solar_lat: f64,
    pub solar_lon: f64,
    pub units: String,
}

#[derive(Debug, Default)]
struct App {
    should_exit: bool,
    current_data: IssData,
    delay: u64, //in ms
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.should_exit {
            self.handle_events()?;
            terminal.draw(|frame| ui::ui(&self.current_data, frame))?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        //Read the events
        match event::read()? {
            //Important - check if it's a key press since crossterm also emits keyreleases
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let action = tui::Action::from_keypress(key_event);
                match action {
                    Action::Exit => self.should_exit = true,
                    Action::DecreaseDelayTime => self.delay -= 100,
                    Action::IncreaseDelayTime => self.delay += 100,
                    Action::PingRESTAPI => todo!(),
                    Action::Error => {},

                }

            }
            _ => {}
        };
        Ok(())
    }
}


fn main() -> Result<(), std::io::Error> {
    //Initialize TUI
    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}


//Run the application loop, checking for events, polling the REST API and drawing to the screen

// println!("Hello, world! (main)");
//     tokio::spawn(async move {
//         let resp = reqwest::get("https://api.wheretheiss.at/v1/satellites/25544")
//             .await.expect("Failed to acquire response from wheretheiss.at!")
//             .json::<IssData>()
//             .await.expect("Failed to convert response to JSON!");

//         println!("Hi from worker! Here's the data: {:#?}", resp);
//     }).await.expect("Failed to retrieve ISS data on alternate Tokio thread!"); 
//     println!("back in main bois");
//     Ok(())
