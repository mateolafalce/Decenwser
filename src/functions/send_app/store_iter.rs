/*
    -This function has the purpose of saving the iteration each time a transaction 
    is successfully sent to the blockchain. It is used later to display the progress 
    in the /convert and y main loading progress bar.
*/

use anyhow::{Ok, Result};
use rocket::serde::json::{from_str, to_string};
use crate::functions::constants::state::State;
use std::fs::{write, read_to_string};

pub fn store_iter(html_js: bool) -> Result<()> {
    // true == html, false == js
    let contents: String = read_to_string("state.json").unwrap();
    let mut config: State = from_str(&contents).unwrap();
    if html_js == true {
        config.html_iter = config.html_iter + 1;
        write("state.json", to_string(&config).unwrap()).unwrap();
    } else if html_js == false {
        config.js_iter = config.js_iter + 1;
        write("state.json", to_string(&config).unwrap()).unwrap();
    }
    Ok(())
}
