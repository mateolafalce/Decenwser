/*
    -The input from the html or js file is received and passed to text that 
    can be entered in the solana blockchain. Certain characters are limited 
    and others collide when sent with the same declaration of a string ("").
    A file is created with a constant that can be iterated over to send the 
    html or js to the blockchain
*/

use rocket::serde::{Deserialize, Serialize, json::Json};
use std::{io::Write, fs::{OpenOptions, File}};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EncodeData {
    pub input: String,
    pub html_js: String
}

pub fn encode(input: String, html_js: String){
    let output: String = input.replace('"', "#~").replace('\\', "#&").replace(',', "#!");
    let lines: Vec<&str> = output.split("\n").collect::<Vec<&str>>();
    let text: String = lines.join("\n");
    let path: String = "src/functions/encode_output/".to_owned() + &html_js + ".rs";
    let modify: File = OpenOptions::new().create(false).write(true).truncate(true).open(path).expect("Error");
    let mut file: File = modify.try_clone().expect("Error");
    let chunks: Vec<String> = text.chars().collect::<Vec<_>>()
        .chunks(900)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();
    file.write_all(format!("pub const DATA: [&str;{}] = {:?};",chunks.len(),chunks).as_bytes()).unwrap();
}


#[post("/", data = "<data>")]
pub fn index(data: Json<EncodeData>)  {
    encode(data.input.clone(), data.html_js.clone())
}
