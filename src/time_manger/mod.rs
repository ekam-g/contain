use std::{path::PathBuf, thread::current};

use crate::encryption::file::EncryptedFile;

use self::time_file_json::{TimeFile, TimeFileJson};

pub mod online_sync;
pub mod time_file;
pub mod time_file_json;
#[derive(Debug, Default, Clone, PartialEq, Eq,)]
pub struct TimeManger{
    /*
    current_time
    containers_time
    
    */     
    pub time_files : Vec<TimeFile>,
    pub current_unix_time : Option<u128>,
    pub time_file_location : PathBuf
}

impl TimeManger {
    pub fn new() -> anyhow::Result<()> {
        let mut time = TimeManger{
            time_files : vec![],
            current_unix_time : None,
            time_file_location : EncryptedFile::file_location(),
        };
        if time.get_time_file().is_err() {
            time.create_time_file(TimeFileJson{ time_files: vec![] })?;
        }
        time.update_time()
    }
}