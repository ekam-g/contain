use super::{
    time_file_json::{TimeFile, TimeFileJson},
    TimeManger,
};
use crate::encryption::file::EncryptedFile;
use anyhow::{anyhow, Error, Ok, Result};
use std::path::PathBuf;

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
    pub fn write_time_file(&self) -> anyhow::Result<()> {
        let efile: EncryptedFile = EncryptedFile::new(self.time_file_location.to_path_buf());
        efile.encrypt_write_file(serde_json::to_vec(&self.time_files)?)?;
        Ok(())
    }
    pub fn add_file(&mut self, path: PathBuf, time: u128) -> anyhow::Result<()> {
        self.time_files.push(TimeFile {
            time: time
                + self
                    .current_unix_time
                    .ok_or(anyhow!("Current Time is is unknown"))?,
            path: path
                .to_str()
                .ok_or(anyhow!("Failed to convert path to string"))?
                .to_owned(),
        });
        self.write_time_file()?;
        let efile: EncryptedFile = EncryptedFile::new(path);
        efile.encrypt_file()?;
        Ok(())
    }
    pub fn decrypt_old_files(&mut self) -> anyhow::Result<()> {
        let time: u128 = self
            .current_unix_time
            .ok_or(anyhow!("Current Time is is unknown"))?;
        for file in self.time_files.iter().filter(|s| s.time < time) {
            let mut efile = EncryptedFile::new(file.path.clone().into());
            efile.decrypt_file()?;
        }
        self.time_files.retain(|s| s.time < time);
        Ok(())
    }
}
