use std::io;

use serde::Deserialize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};


pub mod ui;
pub mod tui;
#[derive(Deserialize, Debug)]
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

struct App {
    should_exit: bool,
    current_data: IssData,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| ui::ui(&self.current_data, frame))?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Initialize TUI


    Ok(())

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
