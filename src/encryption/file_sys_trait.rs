use std::{fs, path::PathBuf};

use super::{file::EncryptedFile, folder::EncryptedFolder};

pub trait Encryptable: Send + Sync  {
    fn encrypt_file(&self) -> anyhow::Result<()>;
    fn decrypt_file(&self) ->  anyhow::Result<()>;
}

pub struct EncryptedFileOrFolder {}

impl EncryptedFileOrFolder {
    pub fn new(path: PathBuf) ->  anyhow::Result<Box<dyn Encryptable>> {
        let meta_data = fs::metadata(&path)?;
        if meta_data.is_file() {
            Ok(Box::new(EncryptedFile::new(path)))
        } else {
            Ok(Box::new(EncryptedFolder::new(path)))
        }
    }
}
