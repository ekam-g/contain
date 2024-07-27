use super::{
    time_file_json::{TimeFile, TimeFileJson},
    TimeManger,
};
use crate::{encryption::file::EncryptedFile, TEST_VALUE};
use anyhow::{anyhow, Ok};
use std::{io::Error, path::PathBuf};

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
        efile.encrypt_write_file(serde_json::to_vec(&TimeFileJson::new(&self.time_files))?)?;
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
        let efile: EncryptedFile = EncryptedFile::new(path);
        efile.encrypt_file()?;
        self.write_time_file()?;
        Ok(())
    }
    //Returns a list of files that it failed to decriypt MAKE SURE TO HANDLE THIS
    pub fn decrypt_old_files(&mut self) -> anyhow::Result<Vec<(String, anyhow::Error)>> {
        let mut failed: Vec<(String, anyhow::Error)> = vec![];
        let time: u128 = self
            .current_unix_time
            .ok_or(anyhow!("Current Time is is unknown"))?;
        for file in self.time_files.iter().filter(|s| s.time < time) {
            let mut efile = EncryptedFile::new(file.path.clone().into());
            if let Err(e) = efile.decrypt_file() {
                failed.push((file.path.clone(), e))
            }
        }
        self.time_files = self
            .time_files
            .clone()
            .into_iter()
            .filter(|s| s.time > time || failed.iter().any(|(path, _)| path != &s.path))
            .collect();
        self.write_time_file()?;
        Ok(failed)
    }
    pub fn remove_time_file() {
        
    }
}
#[cfg(test)]
#[serial_test::serial]
mod timefile_tests {
    use super::TimeManger;
    use crate::{
        encryption::file::EncryptedFile, time_manger::time_file_json::TimeFile, TEST_VALUE,
    };
    use core::time;
    use std::{path::PathBuf, thread};
    #[test]
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
        let mut time = TimeManger::path_new(time_path.clone()).unwrap();
        let contents = String::from_utf8(file_check.read_file().unwrap()).unwrap();
        println!("{contents}");
        assert!(&contents == TEST_VALUE);
        time.current_unix_time = Some(0);
        time.add_file(path.clone(), 1).unwrap();
        time.current_unix_time = Some(2);
        let time_check = EncryptedFile::new(time_path);
        println!(
            "{}",
            String::from_utf8(time_check.clone().decrypt_read_file().unwrap()).unwrap()
        );
        time.decrypt_old_files().unwrap();
        let contents = String::from_utf8(file_check.read_file().unwrap()).unwrap();
        println!("{contents}");
        assert!(&contents == TEST_VALUE);
    }
    #[test]
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
        time.current_unix_time = Some(1);
        time.add_file(path, 3).unwrap();
        println!("{:#?}", time);
        time.current_unix_time = Some(3);
        let time_check = EncryptedFile::new(time_path.clone());
        println!(
            "{}",
            String::from_utf8(time_check.decrypt_read_file().unwrap()).unwrap()
        );
        assert!(time.time_files.len() == 1);
        drop(time);
        let mut time = TimeManger::path_new(time_path).unwrap();
        time.current_unix_time = Some(6);
        time.decrypt_old_files().unwrap();
    }
    #[test]
    fn fail_test() {
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
        //actual test
        time.current_unix_time = Some(8);
        time.add_file(path.clone(), 9).unwrap();
        path.push("BADPATH");
        time.time_files.push(TimeFile {
            path: path.to_str().unwrap().to_owned(),
            time: 9,
        });
        time.current_unix_time = Some(10);
        let failed = time.decrypt_old_files().unwrap();
        let (check, _) = &failed[0];
        let check: PathBuf = check.into();
        assert!(check == path);
        assert!(failed.len() == 1);
        assert!(time.time_files.len() == 1);
        time.time_files = vec![];
        time.write_time_file().unwrap();
        
    }

    #[test]
    fn not_done_test() {
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
        //actual test
        time.current_unix_time = Some(12);
        time.add_file(path.clone(), 2).unwrap();
        let data = time.decrypt_old_files().unwrap();
        println!("Failed: {:?}", data);
        println!("time: {:?}", time.time_files);
        assert!(time.time_files.len() == 1);
        time.current_unix_time = Some(16);
        let _ = time.decrypt_old_files().unwrap();
        assert!(time.time_files.len() == 0);
    }
}
