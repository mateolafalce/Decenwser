use anyhow::{Ok, Result};
use rocket::serde::json::{Json, from_str, to_string};
use crate::functions::constants::wallet::Wallet;
use std::fs::{write, read_to_string};

pub fn store_wallet(new_wallet: Vec<u8>) -> Result<()> {
    let contents: String = read_to_string("src/functions/constants/wallet.json").unwrap();
    let mut wallet: Wallet = from_str(&contents).unwrap();
    wallet.wallet = new_wallet;
    write("src/functions/constants/wallet.json", to_string(&wallet).unwrap()).unwrap();
    Ok(())
}

#[post("/", data = "<wallet>")]
pub fn index(wallet: Json<Vec<u8>>) {
    store_wallet(wallet.into_inner()).unwrap()
}