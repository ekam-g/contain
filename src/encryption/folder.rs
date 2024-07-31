use anyhow::Ok;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use walkdir::WalkDir;

use super::base::{decrypt, encrypt};
use super::file_sys_trait::Encryptable;
use crate::encryption::base::KEY;
use crate::encryption::file::EncryptedFile;
#[allow(unused_imports)]
use crate::TEST_VALUE;
use std::fs::OpenOptions;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, PartialEq, Eq)]

pub struct EncryptedFolder {
    path: PathBuf,
}
impl EncryptedFolder {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}
impl Encryptable for EncryptedFolder {
    fn encrypt_file(&self) -> anyhow::Result<()> {
        WalkDir::new(&self.path)
            .into_iter()
            .filter_map(|e| e.ok())
            .par_bridge()
            .into_par_iter()
            .try_for_each(|entry| {
                let efile = EncryptedFile::new(entry.path().to_owned());
                efile.encrypt_file()?;
                Ok(())
            })?;
        Ok(())
    }
    fn decrypt_file(&self) -> anyhow::Result<()> {
        WalkDir::new(&self.path)
            .into_iter()
            .filter_map(|e| e.ok())
            .par_bridge()
            .into_par_iter()
            .try_for_each(|entry| {
                let efile = EncryptedFile::new(entry.path().to_owned());
                efile.decrypt_file()?;
                Ok(())
            })?;
        Ok(())
    }
}

//todo write folder
#[tokio::test]
#[serial_test::serial(file)]
async fn folder_test() {
    // let mut path = PathBuf::new();
    // path.push("src");
    // path.push("encryption");
    // path.push("test");
    // path.set_extension("txt");
    // let file_check = EncryptedFile::new(path);
    // file_check
    //     .write_file(TEST_VALUE.to_owned().into_bytes())
    //     .unwrap();
    // file_check
    //     .write_file(TEST_VALUE.to_owned().into_bytes())
    //     .unwrap();
    // let data = String::from_utf8(file_check.read_file().await.unwrap()).unwrap();
    // assert!(data == TEST_VALUE)
}
