use super::base::{decrypt, encrypt};
use crate::encryption::base::KEY;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

struct EncryptedFile {
    m_file : File
}
impl EncryptedFile {
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        Self {
            m_file : Self::read_file(path)?,
        }
    }
    fn read_file(path: PathBuf) -> Result<(File, Vec<u8>), std::io::Error> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;

        let mut buf_reader = BufReader::new(&file);
        let mut data: Vec<u8> = Vec::new();
        buf_reader.read_to_end(&mut data)?;

        Ok((file, data))
    }

    pub fn encrypt_file(path: PathBuf) -> anyhow::Result<File> {
        let (mut file, data) = EncryptedFile::read_file(path)?;
        let encrypted_data = encrypt(&data, KEY.as_ref())?;
        drop(data);
        file.seek(SeekFrom::Start(0)).unwrap();
        file.set_len(0)?;
        file.write_all(&encrypted_data).unwrap();
        file.flush()?;
        Ok(file)
    }

    pub fn decrypt_file(path: PathBuf) -> anyhow::Result<File> {
        let (mut file, data) = EncryptedFile::read_file(path)?;
        let data = decrypt(&data, KEY.as_ref())?;
        file.seek(SeekFrom::Start(0)).unwrap();
        file.set_len(0)?;
        file.write_all(&data).unwrap();
        file.flush()?;
        Ok(file)
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
pub fn example() {
    let mut path = PathBuf::new();
    path.push("src");
    path.push("encryption");
    path.push("test");
    path.set_extension("txt");
    encrypt_file(path.clone()).unwrap();
    let (_, _) = read_file(path.clone()).unwrap();
    decrypt_file(path.clone()).unwrap();
    let (_, data) = read_file(path).unwrap();
    println!("{}", String::from_utf8_lossy(&data));
}
