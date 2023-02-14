/*
    -This function returns the iterator and the maximum size of the html/js 
    content to upload for the progress bar
*/

use anyhow::{Result, Ok};
use rocket::serde::{Deserialize, Serialize, json::{from_str, Json}};
use crate::functions::{
    encode_output::html,
    encode_output::js,
    constants::state::State
};
use std::{fs::read_to_string};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Iter {
    pub html_iter: u16,
    pub js_iter: u16,
    pub max_html: u16,
    pub max_js: u16,
}

pub fn get_iter() -> Result<Iter> {
    let contents: String = read_to_string("state.json").unwrap();
    let config: State = from_str(&contents).unwrap();
    let iter: Iter = Iter {
        html_iter: config.html_iter,
        js_iter: config.js_iter,
        max_html: html::DATA.len() as u16,
        max_js: js::DATA.len() as u16
    };
    Ok(iter)
}

#[post("/")]
pub fn index() -> Json<Iter> {
    Json(get_iter().unwrap())
}