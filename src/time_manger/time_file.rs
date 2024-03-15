use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldTimeApiResponse {
    pub abbreviation: String,
    #[serde(rename = "client_ip")]
    pub client_ip: String,
    pub datetime: String,
    #[serde(rename = "day_of_week")]
    pub day_of_week: i64,
    #[serde(rename = "day_of_year")]
    pub day_of_year: i64,
    pub dst: bool,
    #[serde(rename = "dst_from")]
    pub dst_from: String,
    #[serde(rename = "dst_offset")]
    pub dst_offset: i64,
    #[serde(rename = "dst_until")]
    pub dst_until: String,
    #[serde(rename = "raw_offset")]
    pub raw_offset: i64,
    pub timezone: String,
    pub unixtime: i64,
    #[serde(rename = "utc_datetime")]
    pub utc_datetime: String,
    #[serde(rename = "utc_offset")]
    pub utc_offset: String,
    #[serde(rename = "week_number")]
    pub week_number: i64,
}
/// Function to fetch world time from an external API synchronously.
///
/// This function makes an HTTP GET request to the WorldTimeAPI to fetch the current time.
/// If the request is successful, it parses the JSON response into a `WorldTimeApiResponse` struct.
/// Returns a `Result` containing either the parsed response or an error.
///
/// # Errors
///
/// Returns an error if the HTTP request fails or if the response status is not successful.
fn fetch_world_time_sync() -> Result<WorldTimeApiResponse, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("http://worldtimeapi.org/api/ip")?;
    if response.status().is_success() {
        let body = response.text()?;
        let parsed_response: WorldTimeApiResponse = serde_json::from_str(&body)?;
        Ok(parsed_response)
    } else {
        Err("Request failed".into())
    }
}
#[test]
fn example() {
    match fetch_world_time_sync() {
        Ok(response) => println!("{:#?}", response),
        Err(err) => panic!("Error: {:?}", err),
    }
}
