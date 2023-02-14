use anyhow::{Ok, Result};
use rocket::serde::json::{Json, from_str, to_string};
use crate::functions::constants::state::State;
use std::fs::{write, read_to_string};

pub fn store_wallet(new_wallet: Vec<u8>) -> Result<()> {
    let contents: String = read_to_string("state.json").unwrap();
    let mut wallet: State = from_str(&contents).unwrap();
    wallet.wallet = new_wallet;
    write("state.json", to_string(&wallet).unwrap()).unwrap();
    Ok(())
}

#[post("/", data = "<wallet>")]
pub fn index(wallet: Json<Vec<u8>>) {
    store_wallet(wallet.into_inner()).unwrap()
}