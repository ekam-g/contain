use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TimeFileJson {
    pub time_files: Vec<TimeFile>,
}


impl TimeFileJson {
    pub fn new(time_files : &Vec<TimeFile>) -> TimeFileJson {
        TimeFileJson {time_files: time_files.to_owned()}
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TimeFile {
    pub time: u128,
    pub path: String,
}
