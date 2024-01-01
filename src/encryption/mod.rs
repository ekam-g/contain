pub mod base;
pub mod file;

pub fn test() {
    let data = "Yo yo if this works your lit homie";

    println!("Data to encrypt: \"{}\"", &data);

    println!("Encrypting now");
    let res = base::encrypt(data.as_bytes());
    println!("Encrypted response: {}", res);

    let decrypted_bytes = base::decrypt(res.as_str()).unwrap();
    let decrypted_string = std::str::from_utf8(&decrypted_bytes).unwrap();
    println!("Decrypted response: {}", decrypted_string);
}