/*
    -Delivers the configuration state to the user taking the data from config.json
*/

use anyhow::{Ok, Result};
use rocket::serde::json::{Json, from_str};
use crate::functions::constants::config::Config;
use std::fs::read_to_string;

pub fn get_config_settings() -> Result<Config> {
    let contents: String = read_to_string("src/functions/config_settings/config.json").unwrap();
    let config: Config = from_str(&contents).unwrap();
    Ok(config)
}

#[post("/")]
pub fn index() -> Json<Config> {
    Json(get_config_settings().unwrap())
}