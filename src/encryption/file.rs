use super::base::{decrypt, encrypt};
use crate::encryption::base::KEY;
#[allow(unused_imports)]
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
        Self { path }
    }
    pub async fn create_file(&self) -> Result<std::fs::File, std::io::Error> {
        std::fs::File::create(&self.path)
    }
    pub async fn read_file(&self) -> Result<Vec<u8>, std::io::Error> {
        let file = OpenOptions::new().read(true).write(true).open(&self.path)?;
        let mut buf_reader = BufReader::new(file);
        let mut data: Vec<u8> = Vec::new();
        buf_reader.read_to_end(&mut data)?;
        Ok(data)
    }
    pub async fn encrypt_write_file(&self, data: Vec<u8>) -> anyhow::Result<()> {
        let mut file = OpenOptions::new().read(true).write(true).open(&self.path)?;
        file.seek(SeekFrom::Start(0)).unwrap();
        file.set_len(0)?;
        let data = encrypt(&data, KEY.as_ref())?;
        file.write_all(&data).unwrap();
        file.flush()?;
        Ok(())
    }
    fn  write_file(&self, data: Vec<u8>) -> anyhow::Result<()> {
        let mut file = OpenOptions::new().read(true).write(true).open(&self.path)?;
        file.seek(SeekFrom::Start(0)).unwrap();
        file.set_len(0)?;
        file.write_all(&data).unwrap();
        file.flush()?;
        Ok(())
    }

    pub async fn encrypt_file(&self) -> anyhow::Result<()> {
        let data = self.read_file().await?;
        let data = encrypt(&data, KEY.as_ref())?;
        self.write_file(data)
    }

    pub  async fn decrypt_read_file(&self) -> anyhow::Result<Vec<u8>> {
        let data: Vec<u8> = self.read_file().await?;
        decrypt(&data, KEY.as_ref())
    }
    pub async fn decrypt_file(&self) -> anyhow::Result<()> {
        let data: Vec<u8> = self.decrypt_read_file().await?;
        self.write_file(data)
    }
}

#[tokio::test]
#[serial_test::serial(file)]
async fn example() {
    let mut path = PathBuf::new();
    path.push("src");
    path.push("encryption");
    path.push("test");
    path.set_extension("txt");
    let  file_check = EncryptedFile::new(path);
    file_check
        .write_file(TEST_VALUE.to_owned().into_bytes())
        .unwrap();
    file_check.encrypt_file().await.unwrap();
    let data: Vec<u8> = file_check.read_file().await.unwrap();
    let println_data = String::from_utf8_lossy(&data);
    println!("{}", println_data);
    file_check.decrypt_file().await.unwrap();
    let data = String::from_utf8(file_check.read_file().await.unwrap()).unwrap();
    assert!(data == TEST_VALUE)
}

#[tokio::test]
#[serial_test::serial(file)]
async fn file_test() {
    let mut path = PathBuf::new();
    path.push("src");
    path.push("encryption");
    path.push("test");
    path.set_extension("txt");
    let file_check = EncryptedFile::new(path);
    file_check
        .write_file(TEST_VALUE.to_owned().into_bytes())
        .unwrap();
    file_check
        .write_file(TEST_VALUE.to_owned().into_bytes())
        .unwrap();
    let data = String::from_utf8(file_check.read_file().await.unwrap()).unwrap();
    assert!(data == TEST_VALUE)
}
