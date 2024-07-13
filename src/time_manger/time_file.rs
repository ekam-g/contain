use super::{
    time_file_json::{TimeFile, TimeFileJson},
    TimeManger,
};
use crate::{encryption::file::EncryptedFile, TEST_VALUE};
use anyhow::{anyhow, Ok};
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

#[test]
#[serial_test::serial(time)]
pub fn normal() {
    let mut path = PathBuf::new();
    path.push("src");
    path.push("time_manger");
    path.push("test");
    path.set_extension("txt");
    let mut file_check = EncryptedFile::new(path.clone());
    file_check.create_file().unwrap();
    file_check
        .encrypt_write_file(TEST_VALUE.to_owned().into_bytes())
        .unwrap();
    file_check.decrypt_file().unwrap();
    let mut time_path = PathBuf::new();
    time_path.push("src");
    time_path.push("time_manger");
    time_path.push("test");
    time_path.set_extension("timelock");
    let mut time = TimeManger::path_new(time_path).unwrap();
    let contents = String::from_utf8(file_check.read_file().unwrap()).unwrap();
    println!("{contents}");
    assert!(&contents == TEST_VALUE);
    time.current_unix_time = Some(0);
    time.add_file(path.clone(), 1).unwrap();
    time.current_unix_time = Some(2);
    time.decrypt_old_files().unwrap();
    let contents = String::from_utf8(file_check.read_file().unwrap()).unwrap();
    println!("{contents}");
    assert!(&contents == TEST_VALUE);
    time.add_file(path, 3).unwrap();
}
#[test]
#[serial_test::serial(time)]
fn on_off_test() {
    let mut time_path = PathBuf::new();
    time_path.push("src");
    time_path.push("time_manger");
    time_path.push("test");
    time_path.set_extension("timelock");
    let mut path = PathBuf::new();
    path.push("src");
    path.push("time_manger");
    path.push("test");
    path.set_extension("txt");
    let mut time = TimeManger::path_new(time_path.clone()).unwrap();
    println!("{:#?}", time);
    time.current_unix_time = Some(2);
    assert!(time.current_unix_time == Some(2));
    let time_check = EncryptedFile::new(time_path);
    println!(
        "{}",
        String::from_utf8(time_check.decrypt_read_file().unwrap()).unwrap()
    );
    assert!(time.time_files.len() == 1);
    time.current_unix_time = Some(4);
    time.decrypt_old_files().unwrap();
}