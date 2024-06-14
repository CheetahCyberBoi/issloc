use serde::Deserialize;

pub mod ui;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world! (main)");
    tokio::spawn(async move {
        let resp = reqwest::get("https://api.wheretheiss.at/v1/satellites/25544")
            .await.expect("Failed to acquire response from wheretheiss.at!")
            .json::<IssData>()
            .await.expect("Failed to convert response to JSON!");

        println!("Hi from worker! Here's the data: {:#?}", resp);
    }).await.expect("Failed to retrieve ISS data on alternate Tokio thread!"); 
    println!("back in main bois");
    Ok(())

}
