use std::{path::PathBuf, thread::current};

use crate::encryption::file::EncryptedFile;

pub mod online_sync;
pub mod time_file;
#[derive(Debug, Default, Clone, PartialEq, Eq,)]
pub struct TimeManger{
    /*
    current_time
    containers_time
    
    */     
    pub current_unix_time : Option<u128>,
    pub time_file_location : PathBuf
}

impl TimeManger {
    pub fn new() -> anyhow::Result<()> {
        let mut time = TimeManger{
            current_unix_time : None,
            time_file_location : EncryptedFile::file_location(),
        };
        
        time.update_time()
    }
}