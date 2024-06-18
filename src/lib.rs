//! # ISSloc-rs
//! 
//! `issloc_rs` is a binary crate that, when run, displays the current position of the International
//! Space Station on a map, alongside data on the right.


use std::io;
use std::time::Duration;
use std::time::SystemTime;

use crossterm::event::{Event, KeyEventKind};
use crossterm::event;

use serde::Deserialize;

use log::{info, warn, debug};

use crate::timer::Timer;
use crate::tui::Action;

pub mod ui;
pub mod tui;
pub mod timer;

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

#[derive(Debug)]
pub struct App {
    should_exit: bool,
    current_data: IssData,
    delay: u64, //in ms
    timer: Timer,
}

impl App {
    
    /// Creates a new instance of the `App` struct.
    /// Alongside this, it also calls `init_logging` to initalize logging.
    /// Will return an `Err` variant of any error if `init_logging()` returns `Err`.
    /// # Defaults
    /// The default delay for the app is 500 ms.
    /// The associated timer also follows this default delay.
    pub fn new() -> Result<App, Box<dyn std::error::Error>> {
        Self::init_logging()?;
        tui::init()?;
        Ok(App {
            should_exit: false,
            current_data: IssData::default(),
            delay: 500,
            timer: Timer::new(Duration::from_millis(500)),
        })
    }
    /// Runs the associated `App`, handling user input, pinging the REST API, and displaying to the screen.
    /// Will return an `Err` variant if either handling the `App`'s events returns `Err` or if drawing to the terminal returns `Err`.
    /// # Panics
    /// This function will panic if the `Result` from `ping_api` has variant `Err`.
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.should_exit {
            if self.timer.tick() {
                info!("Timer ticked! Pinging API!");
                self.current_data = self.ping_api("https://api.wheretheiss.at/v1/satellites/25544".to_string() /*The ID for the ISS*/).expect("Failed to automatically update ISS data in main thread!");
            }
            self.handle_events()?;
            debug!("Data from the API: {:#?}", self.current_data);
            terminal.draw(|frame| ui::ui(self, frame))?;
        }
        Ok(())
    }

    /// Handles the events that are sent to the `App` by the terminal backend.
    /// This function is called every iteration inside of `run()`.
    /// Will return an `Err` variant if reading the events from the terminal backend returns `Err`.
    /// # Panics
    /// This function will panic if the `Result` returned from `ping_api()` is of variant `Err`.
    fn handle_events(&mut self) -> io::Result<()> {
        //Read the events
        match event::read()? {
            //Important - check if it's a key press since crossterm also emits keyreleases
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                let action = tui::Action::from_keypress(key_event);
                match action {
                    Action::Exit => self.should_exit = true,
                    Action::DecreaseDelayTime => {
                        if self.delay > 0 {self.delay -= 100}
                        self.timer = Timer::new(Duration::from_millis(self.delay));
                        self.timer.reset();
                    },
                    Action::IncreaseDelayTime => {
                        if self.delay < 10000 {self.delay += 100}
                        self.timer = Timer::new(Duration::from_millis(self.delay));
                        self.timer.reset();
                    },
                    Action::PingRESTAPI => self.current_data = self.ping_api("https://api.wheretheiss.at/v1/satellites/25544".to_string()).expect("Failed to manually update ISS data!"),
                    Action::Error => {},

                }

            }
            _ => {}
        };
        Ok(())
    }
    /// Initializes the logging backend for the entire `App`'s duration.
    /// Returns `Err` if Fern's dispatch fails.
    /// ## Note: It will **not** recreate the log file each time.
    fn init_logging() -> Result<(), fern::InitError> {
        fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        //.chain(io::stdout()) // we don't need this if we're rendering stuff!
        .chain(fern::log_file("issloc.log")?)
        .apply()?;
        Ok(())
    }
    /// Pings the REST API passed by `api` and returns the result in an `IssData` struct, wrapped in an Option for safety.
    /// Returns `None` if the API did not send anything back.
    pub fn ping_api(&mut self, api: String) -> Option<IssData> {
        //absolute wizardry, not my code lol
                let resp = ureq::get(api.as_str())
                    .call().unwrap().into_json::<IssData>().ok();
        match resp {
            Some(_) => info!("Got data from API!"),
            None => warn!("No data from API!"),
        }
        resp
    }
}

impl Drop for App {
    fn drop(&mut self) {
        tui::restore().expect("Failled to shut down the TUI!");
        info!("Shutting down App...");
    }
}