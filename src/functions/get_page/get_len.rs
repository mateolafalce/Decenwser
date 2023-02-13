use anyhow::{Ok, Result};
use rocket::serde::json::from_str;
use crate::functions::constants::max_len::Len;
use std::fs::read_to_string;

pub fn get_len() -> Result<Len> {
    let contents: String = read_to_string("src/functions/constants/max_len.json").unwrap();
    let config: Len = from_str(&contents).unwrap();
    Ok(config)
}