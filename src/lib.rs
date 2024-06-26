//! # ISSloc-rs
//! 
//! `issloc_rs` is a binary crate that, when run, displays the current position of the International
//! Space Station on a map, alongside data on the right.



use serde::Deserialize;

pub mod ui;
pub mod tui;
pub mod timer;
pub mod app;

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

