/*
    -Modifies the network from Mainnet to Devnet or vice versa. 
    Confi.json is used to store the state
*/

use anyhow::{Ok, Result};
use rocket::serde::json::{to_string, from_str};
use crate::functions::constants::config::Config;
use std::fs::{write, read_to_string};

pub fn modify_network() -> Result<()> {
    let contents: String = read_to_string("src/functions/config_settings/config.json").unwrap();
    let mut config: Config = from_str(&contents).unwrap();
    if config.network == true {
        config.network = false
    } else if config.network == false {
        config.network = true
    }
    let serialized: String = to_string(&config).unwrap();
    write("src/functions/config_settings/config.json", serialized).unwrap();
    Ok(())
}

#[post("/")]
pub fn index() {
    modify_network().unwrap()
}