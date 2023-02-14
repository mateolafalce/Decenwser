use rocket::serde::json::from_str;
use crate::functions::constants::state::State;
use std::fs::read_to_string;

pub fn get_wallet() -> [u8;64] {
    let contents: String = read_to_string("state.json").unwrap();
    let wallet: State = from_str(&contents).unwrap();
    let mut array = [0; 64];
    array[..wallet.wallet.len()].copy_from_slice(&wallet.wallet);
    array
}