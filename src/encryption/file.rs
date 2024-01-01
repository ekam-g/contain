use std::io::{BufReader, Read, Seek, Write, SeekFrom};
use std::path::PathBuf;
use std::fs::{File, OpenOptions};

use super::base::{encrypt, decrypt};

fn read_file(path: PathBuf) -> Result<(File, String), std::io::Error>{
    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(path)?;
    let mut buf_reader = BufReader::new(&file);
    let mut data = String::new();
    buf_reader.read_to_string(&mut data)?;
    Ok((file, data))
}

pub fn encrypt_file(path : PathBuf) -> Result<File, std::io::Error>{
    let (mut file,mut data) = read_file(path)?;
    data = encrypt(data.as_bytes());
    file.seek(SeekFrom::Start(0)).unwrap();
    file.set_len(0)?;
    file.write_all(data.as_bytes()).unwrap();
    file.flush()?;
    Ok(file)
}

pub fn decrypt_file(path: PathBuf) -> Result<File, Box<dyn std::error::Error>> {
    let (mut file,data) = read_file(path)?;
    let data = decrypt(&data)?;
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
        },
    };
    home_dir.push(".time-lock");
    home_dir.set_extension("contain");
    home_dir
}

pub fn example() {
    let mut path = PathBuf::new();
    path.push("test");
    path.set_extension("txt");
    encrypt_file(path.clone()).unwrap();
    let (_, data) = read_file(path.clone()).unwrap();
    println!("{data}");
    decrypt_file(path.clone()).unwrap();
    let (_, data) = read_file(path).unwrap();
    println!("{data}");
}