/*
    -Its saves the rendered app from the solana blockchain pdas locally in the user computer.
    
    TODO: Create interface with the user to have the possibility of saving it
*/

use anyhow::{Result, Ok};
use std::{io::Write, fs::{OpenOptions, File}};
use crate::functions::{
    get_page::get_domain,
};

pub fn store_app() -> Result<()> {
    let iter: [String;2] = [("apps/".to_owned() + &get_domain().unwrap() + ".html").to_string(), ("apps/".to_owned() + &get_domain().unwrap() + ".js").to_string()];
    let to_read: [String;2] = ["templates/web.html.hbs".to_string(), "public/js.js".to_string()];
    for i in 0..2 {
        let file: File = OpenOptions::new().create(true).write(true).truncate(true).open(iter[i].to_owned()).expect("Error");
        let mut write_file: File = file.try_clone().expect("Error");
        write_file.write_all(to_read[i].as_bytes()).unwrap(); 
    }
    Ok(())
}

#[post("/")]
pub fn index() {
    store_app().unwrap()
}       