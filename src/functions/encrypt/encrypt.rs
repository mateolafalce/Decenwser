#[macro_use] extern crate magic_crypt;
use magic_crypt::MagicCryptTrait;

pub fn encrypt(content: String) -> Result<String> {
    let mcrypt = new_magic_crypt!(KEY, 256); 
    let encrypted_string = mcrypt.encrypt_str_to_base64(content); 
    println!("Encrypted String: {}", encrypted_string); 
    encrypted_string
}