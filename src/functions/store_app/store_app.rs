/*
    This function saves the rendered app from the Solana blockchain PDAs locally on the user's computer.
*/

use anyhow::{
    Result,
    Context,
};
use std::{
    fs::{self, OpenOptions, File},
    io::Write,
};
use crate::functions::get_page::get_domain;

pub fn store_app() -> Result<()> {
    // Create an array of file paths for the app HTML and JavaScript files
    let iter: [String; 2] = [
        format!("apps/{}.html", get_domain()?),
        format!("apps/{}.js", get_domain()?),
    ];
    // Create an array of file paths to read from
    let to_read: [String; 2] = [
        "templates/web.html.hbs".to_string(),
        "public/js.js".to_string(),
    ];
    // Iterate over the file paths and process each file
    for i in 0..2 {
        // Open the file for writing, creating it if it doesn't exist, and truncate its content
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(iter[i].to_owned())
            .with_context(|| format!("Failed to open file: {}", iter[i]))?;
        let mut write_file = file.try_clone().with_context(|| format!("Failed to clone file: {}", iter[i]))?;
        // Read the content from the file to be copied
        let content = fs::read(to_read[i].to_owned()).with_context(|| format!("Failed to read file: {}", to_read[i]))?;
        // Write the content to the destination file
        write_file.write_all(&content).with_context(|| format!("Failed to write to file: {}", iter[i]))?;
    }
    Ok(())
}

#[post("/")]
pub fn index() {
    store_app().expect("Failed to store app");
}
