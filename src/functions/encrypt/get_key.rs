use anyhow::{Result, Ok};
use std::fs::read_to_string;
use rocket::serde::{
    Deserialize, 
    Serialize, 
    json::{
        to_string, 
        from_str
    }
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Key {
    pub key: Vec<u8>,
}

pub fn get_key() -> Result<Vec<u8>>{
    let path: String = "src/functions/constants/key.json".to_string();
    let key_rust: Key = from_str(
        &read_to_string(path.clone()).unwrap()
    ).unwrap();
    Ok(key_rust.key)
}