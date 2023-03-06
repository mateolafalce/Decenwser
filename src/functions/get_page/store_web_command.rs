/*
    -This function interprets the data from the blockchain and renders the web. 
    First, it checks if the page is saved locally. If it is, it is rendered, if it is not, 
    the app is built.
*/

use anyhow::{
    Result, 
    Ok
};
use std::{
    io::Write,  
    fs::{
        read,
        File, 
        OpenOptions
    },
};

pub fn store_web_command() -> Result<()> {
    let file: File = OpenOptions::new()
        .create(false)
        .write(true)
        .truncate(false)
        .append(true) 
        .open("public/js.js")?;
    let mut write_file: File = file.try_clone().expect("Error");
    write_file.write(&read("src/functions/constants/crt+s.js".to_owned()).unwrap()).unwrap(); 
    Ok(())
}

#[post("/")]
pub fn index() {
    store_web_command().unwrap();
}