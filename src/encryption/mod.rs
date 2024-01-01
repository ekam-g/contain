use std::error::Error;
use std::str;
use std::iter::repeat;
use std::str::from_utf8;
use crypto::aead::{AeadDecryptor, AeadEncryptor};
use crypto::aes::KeySize;
use crypto::aes_gcm::AesGcm;
use rand::{thread_rng, Rng};
use const_random::const_random;


const RANDOM_BYTES: [u8; 16] = const_random!([u8 ; 16]);


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
fn get_valid_key(key: &str) -> Vec<u8> {
    let mut bytes = key.as_bytes().to_vec();
    bytes.resize(16, 0x00); // Resize the vector to 16 bytes, padding with 0s if necessary
    bytes.truncate(16); // Truncate the vector to 16 bytes if longer
    
    bytes
}


///Decryption using AES-GCM 128
///iv_data_mac is a string that contains the iv/nonce, data, and mac values. All these values
/// must be hex encoded, and separated by "/" i.e. [hex(iv)/hex(data)/hex(mac)]. This function decodes
/// the values. key (or password) is the raw (not hex encoded) password
pub fn decrypt(iv_data_mac: &str,) -> Result<Vec<u8>, Box<dyn Error>> {
    let (iv, data, mac) = split_iv_data_mac(iv_data_mac)?;
    let key_size = crypto::aes::KeySize::KeySize128;

    // I don't use the aad for verification. aad isn't encrypted anyway, so it's just specified
    // as &[].
    let mut decipher = AesGcm::new(key_size, &RANDOM_BYTES.to_vec(), &iv, &[]);

    // create a list where the decoded data will be saved. dst is transformed in place. It must be exactly the same
    // size as the encrypted data
    let mut dst: Vec<u8> = repeat(0).take(data.len()).collect();
    let result = decipher.decrypt(&data, &mut dst, &mac);

    if result { println!("Successful decryption"); }
    println!("\nDecrypted {}", str::from_utf8(&dst).unwrap());

    Ok(dst)
}

/// Creates an initial vector (iv). This is also called a nonce
fn get_iv(size: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

///encrypt "data" using "password" as the password
/// Output is [hexNonce]/[hexCipher]/[hexMac] (nonce and iv are the same thing)
pub fn encrypt(data: &[u8],) -> String {
    let key_size = KeySize::KeySize128;

    let iv = get_iv(12);

    let mut cipher = AesGcm::new(key_size, &RANDOM_BYTES.to_vec(), &iv, &[]);

    let mut encrypted = vec![0u8; data.len()];
    let mut mac = vec![0u8; 16];

    cipher.encrypt(data, &mut encrypted, &mut mac);

    let output = format!(
        "{}/{}/{}",
        hex::encode(iv),
        hex::encode(&encrypted),
        hex::encode(&mac)
    );

    output
}

pub fn main() {
    let data = "hello world";

    println!("Data to encrypt: \"{}\" and password: \"{:?}\"", &data, RANDOM_BYTES);

    println!("Encrypting now");
    let res = encrypt(data.as_bytes());
    println!("Encrypted response: {}", res);

    println!("Decrypting the response");
    let decrypted_bytes = decrypt(res.as_str()).unwrap();
    let decrypted_string = from_utf8(&decrypted_bytes).unwrap();
    println!("Decrypted response: {}", decrypted_string);
}