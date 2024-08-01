use anyhow::Ok;
use futures::executor::block_on;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use walkdir::WalkDir;

use super::file_sys_trait::Encryptable;
use crate::encryption::file::EncryptedFile;
#[allow(unused_imports)]
use crate::TEST_VALUE;
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
                if entry.file_type().is_file() {
                    let efile = EncryptedFile::new(entry.path().to_owned());
                    efile.encrypt_file()?;
                }
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
                if entry.file_type().is_file() {
                    let efile = EncryptedFile::new(entry.path().to_owned());
                    efile.decrypt_file()?;
                }
                Ok(())
            })?;
        Ok(())
    }
}
#[allow(dead_code)]
type Callback = fn(PathBuf);
#[allow(dead_code)]
fn walk_write(path: &PathBuf, callback : Callback) {
    WalkDir::new(path)
    .into_iter()
    .filter_map(|e| e.ok())
    .par_bridge()
    .into_par_iter()
    .try_for_each(|entry| {
        if entry.file_type().is_file() {
           callback(entry.into_path())
        }
        Ok(())
    }).unwrap();
}
#[allow(dead_code)]
fn write(path: PathBuf) {
    let file = EncryptedFile::new(path);
    file.write_file(TEST_VALUE.as_bytes().to_owned()).unwrap();
}   

#[allow(dead_code)]
fn end_test(path: PathBuf) {
    let file = EncryptedFile::new(path);
    let val = String::from_utf8(block_on(file.read_file()).unwrap()).unwrap();
    assert!(val == TEST_VALUE)
}   
#[allow(dead_code)]
fn encrypt_test(path: PathBuf) {
    let file = EncryptedFile::new(path);
    let val = String::from_utf8(block_on(file.decrypt_read_file()).unwrap()).unwrap();
    assert!(val == TEST_VALUE)
} 

//todo write folder
#[tokio::test]
#[serial_test::serial(file)]
async fn folder_test() {
    let mut path = PathBuf::new();
    path.push("src");
    path.push("encryption");
    path.push("test");
    let folder = EncryptedFolder::new(path.clone());
    walk_write(&path, write);
    folder.encrypt_file().unwrap();
    walk_write(&path, encrypt_test);
    folder.decrypt_file().unwrap();
    walk_write(&path, end_test);
}

