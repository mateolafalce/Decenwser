/*
    - The input from the HTML or JS file is received and passed to UTF-8.
*/

use rocket::serde::{
    Deserialize,
    Serialize,
    json::{
        Json,
        from_str,
        to_string
    }
};
use std::{
    fs::{
        write,
        read_to_string
    }
};
use crate::functions::constants::encode::Encode;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EncodeData {
    pub input: String,
    pub html_js: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Price {
    pub price: u64,
    pub html_js: String
}

pub fn encode(input: String, html_js: String) -> Result<Price> {
    // Split the input into chunks of 900 bytes and convert them to a vector of byte vectors.
    let encode: Vec<Vec<u8>> = input.as_bytes()
        .chunks(900)
        .map(|chunk| chunk.to_vec())
        .collect();
    if html_js == "html" {
        // Read the configuration from the encode_html.json file.
        let mut config: Encode = from_str(&read_to_string("src/functions/encode_output/encode_html.json").unwrap()).unwrap();
        config.content = encode;
        // Write the updated configuration back to the encode_html.json file.
        write("src/functions/encode_output/encode_html.json", to_string(&config).unwrap()).unwrap();
        let price: Price = Price {
            // Calculate the price based on the length of the content.
            price: (config.content.len() * 6269000) as u64,
            html_js: html_js
        };
        Ok(price)
    } else {
        // Read the configuration from the encode_js.json file.
        let mut config: Encode = from_str(&read_to_string("src/functions/encode_output/encode_js.json").unwrap()).unwrap();
        config.content = encode;
        // Write the updated configuration back to the encode_js.json file.
        write("src/functions/encode_output/encode_js.json", to_string(&config).unwrap()).unwrap();
        let price: Price = Price {
            // Calculate the price based on the length of the content.
            price: (config.content.len() * 6269000) as u64,
            html_js: html_js
        };
        Ok(price)
    }
}

#[post("/", data = "<data>")]
pub fn index(data: Json<EncodeData>) -> Json<Price> {
    Json(encode(data.input.clone(), data.html_js.clone()).unwrap())
}
