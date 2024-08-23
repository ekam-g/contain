use anyhow::anyhow;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use crate::time_manger::TimeManger;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldTimeApiResponse {
    // pub abbreviation: String,
    // #[serde(rename = "client_ip")]
    // pub client_ip: String,
    // pub datetime: String,
    // #[serde(rename = "day_of_week")]
    // pub day_of_week: i64,
    // #[serde(rename = "day_of_year")]
    // pub day_of_year: i64,
    // pub dst: bool,
    // #[serde(rename = "dst_from")]
    // pub dst_from: String,
    // #[serde(rename = "dst_offset")]
    // pub dst_offset: i64,
    // #[serde(rename = "dst_until")]
    // pub dst_until: String,
    // #[serde(rename = "raw_offset")]
    // pub raw_offset: i64,
    // pub timezone: String,
    pub unixtime: u128,
    // #[serde(rename = "utc_datetime")]
    // pub utc_datetime: String,
    // #[serde(rename = "utc_offset")]
    // pub utc_offset: String,
    // #[serde(rename = "week_number")]
    // pub week_number: i64,
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
impl TimeManger {
    async fn fetch_world_time_sync() -> anyhow::Result<WorldTimeApiResponse> {
        let response = reqwest::get("https://worldtimeapi.org/api/timezone/Europe/London").await?;
        if response.status().is_success() {
            let body = response.text().await?;
            let parsed_response: WorldTimeApiResponse = serde_json::from_str(&body)?;
            Ok(parsed_response)
        } else {
            Err(anyhow!(
                "Failed With Bad Status Code Of {}",
                response.status()
            ))
        }
    }
    pub async fn update_time_online(&mut self) -> anyhow::Result<()> {
        match TimeManger::fetch_world_time_sync().await {
            Ok(data) => {
                self.current_unix_time = Some(data.unixtime);
                return Ok(());
            }
            Err(e1) => {
                //todo add logging
                println!("{}", e1);
                match TimeManger::update_time_ntp() {
                    Err(e2) => {
                        return Err(anyhow!(format!(
                            "Connection Error\n api: {}\n ntp:{}",
                            e1, e2
                        )))
                    }
                    Ok(data) => {
                        self.current_unix_time = Some(data);
                        return Ok(());
                    }
                }
            }
        }
    }
    fn update_time_ntp() -> anyhow::Result<u128> {
        let response = ntp::request("0.pool.ntp.org:123");
        match response {
            Err(e) => {
                return Err(anyhow!(e.to_string()));
            }
            Ok(response) => {
                let ntp_time = response.transmit_time;
                return Ok(ntp_time.sec as u128);
            }
        }
    }
    pub fn get_time(&self) -> Option<u128> {
        if let Some(current_time) = self.current_unix_time {
            Some(self.timer.elapsed().as_secs() as u128 + current_time)
        } else {
            None
        }
    }
}
#[tokio::test]
async fn example() {
    match TimeManger::fetch_world_time_sync().await {
        Ok(response) => println!("{:#?}", response.unixtime),
        Err(err) => panic!("Error: {:?}", err),
    }
}
