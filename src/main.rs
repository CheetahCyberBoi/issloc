use std::io;
use std::time::Duration;
use std::time::SystemTime;

use serde::Deserialize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use pollster::FutureExt as _;
use crate::tui::Action;
use crate::timer::Timer;
use log::{debug, error, info, trace, warn};


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
struct App {
    should_exit: bool,
    current_data: IssData,
    delay: u64, //in ms
    timer: Timer,
}

impl App {

    pub fn new() -> App {
        App {
            should_exit: false,
            current_data: IssData::default(),
            delay: 500,
            timer: Timer::new(Duration::from_millis(500)),
        }
    }
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.should_exit {
            if self.timer.tick() {
                info!("Timer ticked! Pinging API!");
                self.current_data = self.ping_api("https://api.wheretheiss.at/v1/satellites/25544".to_string() /*The ID for the ISS*/).expect("Failed to automatically update ISS data in main thread!");
            }
            self.handle_events()?;
            terminal.draw(|frame| ui::ui(self, frame))?;
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

    pub fn ping_api(&mut self, api: String) -> Option<IssData> {
        //absolute wizardry, not my code lol
                let resp = ureq::get(api.as_str())
                    .call().unwrap().into_json::<IssData>().ok();
        match resp {
            Some(ref resp) => info!("Got data from API!"),
            None => warn!("No data from API!"),
        }
        resp
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Setup logging
    setup_logger()?;
    info!("Initialized logging!");
    //Initialize TUI
    let mut terminal = tui::init()?;
    let mut app = App::new();
    info!("App initialized!");
    let app_result = app.run(&mut terminal);
    
    tui::restore()?;
    Ok(app_result?)
}


fn setup_logger() -> Result<(), fern::InitError> {
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
        .chain(io::stdout())
        .chain(fern::log_file("issloc.log")?)
        .apply()?;
    Ok(())
}
