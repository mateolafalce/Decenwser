use anyhow::{Ok, Result};
use rocket::serde::json::from_str;
use crate::functions::constants::domain::Domain;
use std::fs::read_to_string;

pub fn get_domain() -> Result<String> {
    let contents: String = read_to_string("src/functions/constants/domain.json").unwrap();
    let config: Domain = from_str(&contents).unwrap();
    Ok(config.domain)
}