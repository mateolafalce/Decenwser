/*
    -This function returns the iterator and the maximum size of the html/js 
    content to upload for the progress bar
*/

use anyhow::{
    Result, 
    Ok
};
use rocket::serde::{
    Deserialize, 
    Serialize, 
    json::{
        from_str, 
        Json
    }
};
use crate::functions::{
    constants::{
        state::State,
        encode::Encode
    }
};
use std::{
    fs::read_to_string
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Iter {
    pub html_iter: u16,
    pub js_iter: u16,
    pub max_html: u16,
    pub max_js: u16,
}

pub fn get_iter() -> Result<Iter> {
    let html_len: Encode = from_str(&read_to_string("src/functions/encode_output/encode_html.json").unwrap()).unwrap();
    let js_len: Encode = from_str(&read_to_string("src/functions/encode_output/encode_js.json").unwrap()).unwrap();
    let contents: String = read_to_string("state.json").unwrap();
    let config: State = from_str(&contents).unwrap();
    let iter: Iter = Iter {
        html_iter: config.html_iter,
        js_iter: config.js_iter,
        max_html: html_len.content.len() as u16,
        max_js: js_len.content.len() as u16
    };
    Ok(iter)
}

#[post("/")]
pub fn index() -> Json<Iter> {
    Json(get_iter().unwrap())
}