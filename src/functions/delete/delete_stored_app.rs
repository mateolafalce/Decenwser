/*
    -Deletes the specified application, stored in the apps folder
*/

use anyhow::{
    Ok, 
    Result
};
use std::fs::remove_file;

pub fn delete_app(app:String) -> Result<()> {
    let iter: [String;2] = ["apps/".to_owned() + &app + ".html", "apps/".to_owned() + &app + ".js"];
    for line in iter.iter() {
        remove_file(line)?; 
    }
    Ok(())
}

#[post("/", data = "<domain>")]
pub fn index(domain: String) {
    delete_app(domain).unwrap()
}
