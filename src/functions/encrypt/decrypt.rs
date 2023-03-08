/*#[macro_use] extern crate magic_crypt;
use magic_crypt::MagicCryptTrait;

pub fn decrypt(encrypted_string: String) -> Result<String> {
    let decrypted_string: String = new_magic_crypt!(
        KEY, 
        256
    ).decrypt_base64_to_string(&encrypted_string).unwrap();
    println!("Decrypted String: {}", decrypted_string);
    decrypted_string
}*/