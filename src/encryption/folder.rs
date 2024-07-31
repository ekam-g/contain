use super::base::{decrypt, encrypt};
use super::file_sys_trait::Encryptable;
use crate::encryption::base::KEY;
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
        todo!()
    }
    fn decrypt_file(&self) -> anyhow::Result<()> {
        todo!()
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
