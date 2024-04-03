use crate::encryption::file::EncryptedFile;

use serde_derive::Deserialize;
use serde_derive::Serialize;

use super::TimeManger;

impl TimeManger {
    pub fn check_time_file(&self) -> anyhow::Result<()> { 
        let efile = EncryptedFile::new(self.time_file_location.to_path_buf());
        let data = efile.decrypt_read_file()?;
        
        Ok(())
    }

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeFileJson {
    pub time_files: Vec<TimeFile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeFile {
    pub time: i64,
    pub path: String,
    pub nickname: String,
}
