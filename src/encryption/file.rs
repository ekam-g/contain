use std::path::{Path, PathBuf};



pub fn encrypt_file() {
    let _path = Path::new(".").display();

}

pub fn decrypt_file() {}

pub fn file_location() -> PathBuf {
    let mut home_dir = match home::home_dir() {
        Some(path) => path,
        None => {
            println!("Failed to get your home dir!");
            PathBuf::new()
        },
    };
    home_dir.push(".time-lock");
    home_dir.set_extension(".contain");
    home_dir
}