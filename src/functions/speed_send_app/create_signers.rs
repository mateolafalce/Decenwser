/*
    -Create in the respective json file a list of wallets with private keys
*/
use anchor_client::solana_sdk::signature::Keypair;
use rocket::serde::{
    json::{
        to_string, 
        from_str
    }
};
use anyhow::{Result, Ok};
use crate::functions::{
    encode_output::{html, js},
    speed_send_app::signers::Signers
};
use std::fs::{read_to_string, write};

pub fn create_signers() -> Result<()> {
    let mut html_signers: Signers = from_str(&read_to_string("src/functions/speed_send_app/html_signers.json").unwrap()).unwrap();
    let mut js_signers: Signers = from_str(&read_to_string("src/functions/speed_send_app/js_signers.json").unwrap()).unwrap();
    for _i in 0..html::DATA.len() {
        html_signers.signers.push(Keypair::new().to_bytes().to_vec());
        write("src/functions/speed_send_app/html_signers.json", to_string(&html_signers).unwrap()).unwrap();
    }
    for _i in 0..js::DATA.len() {
        js_signers.signers.push(Keypair::new().to_bytes().to_vec());
        write("src/functions/speed_send_app/js_signers.json", to_string(&js_signers).unwrap()).unwrap();
    }
    Ok(())
}

#[post("/")]
pub fn index() {
    create_signers().unwrap()
}
