use std::thread::current;

pub mod online_sync;
pub mod time_file;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq,)]
pub struct TimeManger{
    /*
    current_time
    containers_time
    
    */     
    pub current_unix_time : Option<u128>
}

impl TimeManger {
    pub fn new() -> anyhow::Result<()> {
        let mut time = TimeManger{
            current_unix_time : None,
        };
        time.update_time()
    }
}