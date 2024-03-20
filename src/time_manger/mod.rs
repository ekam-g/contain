use std::thread::current;

pub mod online_sync;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq,)]
pub struct TimeManger{
    /*
    current_time
    containers_time
    
    */     
    pub current_unix_time : u128
}