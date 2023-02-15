/*
    -Return the size saved in the json file to internal functions and frontend
*/

use anyhow::{Ok, Result};
use rocket::serde::{Deserialize, Serialize, json::from_str};
use crate::functions::constants::state::State;
use std::fs::read_to_string;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Len {
    pub len_html: u16,
    pub len_js: u16,
}

pub fn get_len() -> Result<Len> {
    let contents: String = read_to_string("state.json").unwrap();
    let config: State = from_str(&contents).unwrap();
    let len: Len = Len {
        len_html: config.len_html,
        len_js: config.len_js
    };
    Ok(len)
}