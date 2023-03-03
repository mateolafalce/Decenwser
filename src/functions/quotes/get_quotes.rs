use anyhow::{Ok, Result};
use rocket::serde::json::{Json, from_str};
use crate::functions::constants::state::State;
use std::fs::read_to_string;

pub fn get_quotes() -> Result<Cites> {
    //let contents: String = read_to_string("src/fuctions/cites/citesstate.json").unwrap();
    //let config: Cites = from_str(&contents).unwrap();
    //Ok(config)
}

#[post("/")]
pub fn index() -> Json<Cites> {
    Json(get_cites().unwrap())
}