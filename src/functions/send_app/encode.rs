/*
    -The input from the html or js file is received and passed to utf-8
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EncodeData {
    pub input: String,
    pub html_js: String
}

pub fn encode(input: String, html_js: String){
    let encode: Vec<Vec<u8>> = input.as_bytes()
        .chunks(900)
        .map(|chunk| chunk.to_vec())
        .collect();
    if html_js == "html" {
        let mut config: Encode = from_str(&read_to_string("src/functions/encode_output/encode_html.json").unwrap()).unwrap();
        config.content = encode;
        write("src/functions/encode_output/encode_html.json", to_string(&config).unwrap()).unwrap();
    } else  {
        let mut config: Encode = from_str(&read_to_string("src/functions/encode_output/encode_js.json").unwrap()).unwrap();
        config.content = encode;
        write("src/functions/encode_output/encode_js.json", to_string(&config).unwrap()).unwrap();
    }
}


#[post("/", data = "<data>")]
pub fn index(data: Json<EncodeData>)  {
    encode(data.input.clone(), data.html_js.clone())
}
