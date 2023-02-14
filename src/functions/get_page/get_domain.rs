use anyhow::{Ok, Result};
use rocket::serde::json::from_str;
use crate::functions::constants::state::State;
use std::fs::read_to_string;

pub fn get_domain() -> Result<String> {
    let contents: String = read_to_string("state.json").unwrap();
    let config: State = from_str(&contents).unwrap();
    Ok(config.domain)
}