use super::base::{decrypt, encrypt};
use crate::encryption::base::KEY;
use crate::TEST_VALUE;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, PartialEq, Eq)]

pub struct EncryptedFile {
    path: PathBuf,
}
impl EncryptedFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path: path }
    }
    pub fn create_file(&self) -> Result<std::fs::File, std::io::Error> {
        std::fs::File::create(&self.path)
    }
    pub fn read_file(&self) -> Result<Vec<u8>, std::io::Error> {
        let file = OpenOptions::new().read(true).write(true).open(&self.path)?;
        let mut buf_reader = BufReader::new(file);
        let mut data: Vec<u8> = Vec::new();
        buf_reader.read_to_end(&mut data)?;
        Ok(data)
    }
    pub fn encrypt_write_file(&self, data: Vec<u8>) -> anyhow::Result<()> {
        let mut file = OpenOptions::new().read(true).write(true).open(&self.path)?;
        file.seek(SeekFrom::Start(0)).unwrap();
        file.set_len(0)?;
        let data = encrypt(&data, KEY.as_ref())?;
        file.write_all(&data).unwrap();
        file.flush()?;
        Ok(())
    }
    fn write_file(&self, data: Vec<u8>) -> anyhow::Result<()> {
        let mut file = OpenOptions::new().read(true).write(true).open(&self.path)?;
        file.seek(SeekFrom::Start(0)).unwrap();
        file.set_len(0)?;
        file.write_all(&data).unwrap();
        file.flush()?;
        Ok(())
    }

    pub fn encrypt_file(&self) -> anyhow::Result<()> {
        let data = self.read_file()?;
        let data = encrypt(&data, KEY.as_ref())?;
        self.write_file(data)
    }

    pub fn decrypt_read_file(&self) -> anyhow::Result<Vec<u8>> {
        let data: Vec<u8> = self.read_file()?;
        decrypt(&data, KEY.as_ref())
    }
    pub fn decrypt_file(&mut self) -> anyhow::Result<()> {
        let data: Vec<u8> = self.decrypt_read_file()?;
        self.write_file(data)
    }

    pub fn file_location() -> PathBuf {
        let mut home_dir = match home::home_dir() {
            Some(path) => path,
            None => {
                println!("Failed to get your home dir!");
                PathBuf::new()
            }
        };
        home_dir.push(".time-lock");
        home_dir.set_extension("contain");
        home_dir
    }
}

#[test]
#[serial_test::serial(file)]
pub fn example() {
    let mut path = PathBuf::new();
    path.push("src");
    path.push("encryption");
    path.push("test");
    path.set_extension("txt");
    let mut file_check = EncryptedFile::new(path);
    file_check
        .write_file(TEST_VALUE.to_owned().into_bytes())
        .unwrap();
    file_check.encrypt_file().unwrap();
    let data: Vec<u8> = file_check.read_file().unwrap();
    let println_data = String::from_utf8_lossy(&data);
    println!("{}", println_data);
    file_check.decrypt_file().unwrap();
    let data = String::from_utf8(file_check.read_file().unwrap()).unwrap();
    assert!(data == TEST_VALUE)
}

#[test]
#[serial_test::serial(file)]
pub fn file_test() {
    let mut path = PathBuf::new();
    path.push("src");
    path.push("encryption");
    path.push("test");
    path.set_extension("txt");
    let mut file_check = EncryptedFile::new(path);
    file_check
        .write_file(TEST_VALUE.to_owned().into_bytes())
        .unwrap();
    file_check
        .write_file(TEST_VALUE.to_owned().into_bytes())
        .unwrap();
    let data = String::from_utf8(file_check.read_file().unwrap()).unwrap();
    assert!(data == TEST_VALUE)
}
