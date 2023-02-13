/*
    -Each change to the navigation bar makes a request to this function. 
    Saving the domain on the rust side saves a lot of time in functions 
    that take domain as an argument.
*/

use anyhow::{Ok, Result};
use rocket::serde::json::{from_str, to_string};
use crate::functions::constants::domain::Domain;
use std::fs::{write, read_to_string};

pub fn store_domain(domain: String) -> Result<()> {
    let contents: String = read_to_string("src/functions/constants/domain.json").unwrap();
    let mut config: Domain = from_str(&contents).unwrap();
    config.domain = domain;
    write("src/functions/constants/domain.json", to_string(&config).unwrap()).unwrap();
    Ok(())
}

#[post("/", data = "<domain>")]
pub fn index(domain: String) {
    store_domain(domain).unwrap()
}