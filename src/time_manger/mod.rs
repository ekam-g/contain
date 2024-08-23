use std::{path::PathBuf, time::Instant};


use self::time_file_json::{TimeFile, TimeFileJson};

pub mod online_sync;
pub mod time_file;
pub mod time_file_json;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeManger {
    pub time_files: Vec<TimeFile>,
    current_unix_time: Option<u128>,
    pub time_file_location: PathBuf,
    pub timer : std::time::Instant,
}

impl TimeManger {
    pub async fn new() -> anyhow::Result<TimeManger> {
        Self::path_new(Self::file_location()).await
    }
    pub async fn path_new(path: PathBuf) -> anyhow::Result<TimeManger> {
        let mut time = TimeManger {
            time_files: vec![],
            current_unix_time: None,
            time_file_location: path,
            timer : Instant::now()
        };
        if time.get_time_file().await.is_err() {
            time.create_time_file(TimeFileJson { time_files: vec![] }).await?;
        }
        time.update_time_online().await?;
        Ok(time)
    }
}
