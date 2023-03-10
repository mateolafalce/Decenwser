use anchor_client::solana_sdk::signature::Keypair;
use anyhow::{Result, Ok};
use std::fs::{write, read_to_string};
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

pub fn new_key() -> Result<()>{
    let path: String = "src/functions/constants/key.json".to_string();
    let mut key_rust: Key = from_str(
        &read_to_string(path.clone()).unwrap()
    ).unwrap();
    key_rust.key = Keypair::new().secret().to_bytes().into();
    write(path.clone(), to_string(&key_rust).unwrap()).unwrap();
    Ok(())
}