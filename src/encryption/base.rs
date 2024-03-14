use std::{error::Error};
use std::str;
use aead::{generic_array::{GenericArray}, Aead, AeadCore, KeyInit};
use aes_gcm::{Aes256Gcm};
use const_random::const_random;
use anyhow::anyhow;

pub const RANDOM_BYTES: [u8; 32] = random();


/// orig must be a string of the form [hexNonce]/[hexCipherText]/[hexMac]. This
/// is the data returned from encrypt(). This function splits the data, removes
/// the hex encoding, and returns each as a list of bytes.
fn split_iv_data_mac(orig: &str) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let split: Vec<&str> = orig.split('/').collect();

    if split.len() != 3 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid input format")));
    }

    let iv = hex::decode(split[0])?;
    let data = hex::decode(split[1])?;
    let mac = hex::decode(split[2])?;

    Ok((iv, data, mac))
}

/// gets a valid key. This must be exactly 16 bytes. if less than 16 bytes, it will be padded with 0.
/// If more than 16 bytes, it will be truncated
pub fn get_valid_key(key: &str) -> Vec<u8> {
    let mut bytes = key.as_bytes().to_vec();
    bytes.resize(32, 0x00); // Resize the vector to 16 bytes, padding with 0s if necessary
    bytes.truncate(32); // Truncate the vector to 16 bytes if longer
    
    bytes
}

pub fn encrypt(contents: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>> {
    let key = GenericArray::from_slice(key);
    let nonce = Aes256Gcm::generate_nonce(&mut aead::OsRng);
    // encryption
    let cipher = Aes256Gcm::new(key);
    cipher
        .encrypt(&nonce, contents.as_ref())
        .map_err(|e| anyhow!(e))
}

pub fn decrypt(cipher_text: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>> {
    let key = GenericArray::from_slice(key);
    let nonce = Aes256Gcm::generate_nonce(&mut aead::OsRng);
    // decryption
    let cipher = Aes256Gcm::new(key);
    cipher.decrypt(&nonce, cipher_text).map_err(|e| anyhow!(e))
}
pub fn example() {
    let data = "Yo yo if this works your lit homie";
    let key = get_valid_key("Hello TEST key");
    println!("Data to encrypt: \"{}\"", &data);

    println!("Encrypting now");
    let res = encrypt(data.as_bytes(), &key).unwrap();
    println!("Encrypted response: {}", String::from_utf8(res.clone()).unwrap());

    let decrypted_bytes = decrypt(&res, &key).unwrap();
    let decrypted_string = std::str::from_utf8(&decrypted_bytes).unwrap();
    println!("Decrypted response: {}", decrypted_string);
}
// TODO: change this to better rust
const fn random() -> [u8 ; 32]{
    let mut random_array: [u8; 32] = [0; 32];    let mut x = 0;
    while x != 32  {
        random_array[x] = const_random!(u8);
        x+=1;
    }
    random_array
}