/*
    -Modifies the network from Mainnet to Devnet or vice versa. 
    Config.json is used to store the state
*/

use anyhow::{Ok, Result};
use rocket::serde::json::{to_string, from_str};
use crate::functions::constants::state::State;
use std::fs::{write, read_to_string};

pub fn modify_network() -> Result<()> {
    let contents: String = read_to_string("state.json").unwrap();
    let mut config: State = from_str(&contents).unwrap();
    if config.network == true {
        config.network = false
    } else if config.network == false {
        config.network = true
    }
    let serialized: String = to_string(&config).unwrap();
    write("state.json", serialized).unwrap();
    Ok(())
}

#[post("/")]
pub fn index() {
    modify_network().unwrap()
}