use anyhow::Ok;

use crate::encryption::file::EncryptedFile;

use super::{time_file_json::TimeFileJson, TimeManger};

impl TimeManger {
    pub fn get_time_file(&mut self) -> anyhow::Result<()> {
        let efile: EncryptedFile = EncryptedFile::new(self.time_file_location.to_path_buf());
        let data: TimeFileJson = serde_json::from_slice(&efile.decrypt_read_file()?)?;
        self.time_files = data.time_files;
        Ok(())
    }
    pub fn create_time_file(&self, data: TimeFileJson) -> anyhow::Result<()> {
        let efile: EncryptedFile = EncryptedFile::new(self.time_file_location.to_path_buf());
        efile.create_file()?;
        efile.encrypt_write_file(serde_json::to_vec(&data)?)?;
        Ok(())
    }
}
