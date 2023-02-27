/*
    -Delivers the configuration state to the user taking the data from config.json
*/

use anyhow::{Ok, Result};
use rocket::serde::json::{Json, from_str};
use crate::functions::constants::state::State;
use std::fs::read_to_string;

pub fn get_config_settings() -> Result<State> {
    let contents: String = read_to_string("state.json").unwrap();
    let config: State = from_str(&contents).unwrap();
    Ok(config)
}

#[post("/")]
pub fn index() -> Json<State> {
    Json(get_config_settings().unwrap())
}