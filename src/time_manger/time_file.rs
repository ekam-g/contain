use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
struct WorldTimeApiResponse {
    abbreviation: String,
    client_ip: String,
    datetime: String,
    day_of_week: u32,
    day_of_year: u32,
    dst: bool,
    dst_from: String,
    dst_offset: i32,
    dst_until: String,
    raw_offset: i32,
    timezone: String,
    unixtime: i64,
    utc_datetime: String,
    utc_offset: String,
    week_number: u32,
}

fn fetch_world_time_sync() -> Result<WorldTimeApiResponse, Box<dyn std::error::Error>> {
    // Make the HTTP GET request
    let response = reqwest::blocking::get("http://worldtimeapi.org/api/ip")?;

    // Check if the request was successful
    if response.status().is_success() {
        // Parse the JSON response
        let body = response.text()?;
        let parsed_response: WorldTimeApiResponse = serde_json::from_str(&body)?;

        // Return the parsed response
        Ok(parsed_response)
    } else {
        Err("Request failed".into())
    }
}
#[test]
fn example() {
    match fetch_world_time_sync() {
        Ok(response) => println!("{:#?}", response),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
