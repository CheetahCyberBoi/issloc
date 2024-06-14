use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct IssData {
    name: String,
    id: u32,
    latitude: f64,
    longitude: f64,
    altitude: f64,
    velocity: f64,
    visibility: String,
    footprint: f64,
    timestamp: u64,
    daynum: f64,
    solar_lat: f64,
    solar_lon: f64,
    units: String,
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
