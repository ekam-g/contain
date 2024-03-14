use aead::{generic_array::GenericArray, Aead, KeyInit};
use aes_gcm::Aes256Gcm;
use anyhow::anyhow;
use const_random::const_random;
use std::str;

pub const KEY: [u8; 32] = random_key();
pub const NOICE: [u8; 12] = random_noice();
/// Generates a valid key. This must be exactly 32 bytes.
/// If the input key is shorter than 32 bytes, it will be padded with 0s.
/// If the input key is longer than 32 bytes, it will be truncated.
pub fn get_valid_key(key: &str) -> [u8; 32] {
    let mut bytes = [0; 32];
    let key_bytes = key.as_bytes();
    let len = key_bytes.len().min(32);
    bytes[..len].copy_from_slice(&key_bytes[..len]);
    bytes
}

pub fn encrypt(contents: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>> {
    let key: &GenericArray<u8, _> = GenericArray::from_slice(key);
    let nonce = GenericArray::from_slice(&NOICE); // You need to generate your nonce securely, here I'm using a zero nonce for simplicity    // encryption
    let cipher = Aes256Gcm::new(key);
    cipher
        .encrypt(&nonce, contents.as_ref())
        .map_err(|e| anyhow!(e))
}

pub fn decrypt(cipher_text: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>> {
    let key = GenericArray::from_slice(key);
    // let nonce = Aes256Gcm::generate_nonce(&mut aead::OsRng);
    let nonce = GenericArray::from_slice(&NOICE); // Same as above, using a zero nonce for simplicity    // decryption
    let cipher = Aes256Gcm::new(key);
    cipher.decrypt(&nonce, cipher_text).map_err(|e| anyhow!(e))
}

const fn random_key() -> [u8; 32] {
    let mut random_array: [u8; 32] = [0; 32];
    let mut x = 0;
    while x != 32 {
        random_array[x] = const_random!(u8);
        x += 1;
    }
    random_array
}
const fn random_noice() -> [u8; 12] {
    let mut random_array: [u8; 12] = [0; 12];
    let mut x = 0;
    while x != 12 {
        random_array[x] = const_random!(u8);
        x += 1;
    }
    random_array
}

#[test]
pub fn example() {
    let data = "Yo yo if this works your lit homie";
    let key = get_valid_key("Hello TEST key IHBGEJHFBWOHDFBJSHDBFJHASDBFJHASJHDBsspp>");
    println!("Data to encrypt: \"{}\"", &data);
    let res = encrypt(data.as_bytes(), &key).unwrap();
    println!("Encrypied Data {}", String::from_utf8_lossy(&res));
    let decrypted_bytes = decrypt(&res, &key).unwrap();
    let decrypted_string = std::str::from_utf8(&decrypted_bytes).unwrap();
    println!("Decrypted response: {}", decrypted_string);
    assert!(decrypted_string == data.to_string());
}
