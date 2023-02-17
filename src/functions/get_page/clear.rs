/*
    -Removes the content in the page to render, in the domain, 
    in the build app result, and in the encoded result. 
    A request is made at render time /main
*/

use std::{
    io::{Write,Result}, 
    fs::{OpenOptions, self, File, read_to_string, write},
    path::Path 
};
use rocket::serde::json::{to_string, from_str};
use crate::functions::{
    speed_send_app::signers::Signers,
    send_app::{
        store_iter::store_iter,
        store_wallet::store_wallet
    }
};

pub fn clear() {
    let path: [String;4] = ["templates/web.html.hbs".to_string(), "public/js.js".to_string(), "src/functions/encode_output/html.rs".to_string(), "src/functions/encode_output/js.rs".to_string()];
    let content: [String;4] = ["".to_string(), "".to_string(), "pub const DATA: [&str;1] = [\"Ok\"];".to_string(), "pub const DATA: [&str;1] = [\"Ok\"];".to_string()];
    let clear_wallet: Vec<u8> = vec![1].iter().cloned().cycle().take(64).collect::<Vec<u8>>();
    store_wallet(clear_wallet).unwrap();
    store_iter(true, 0).unwrap();
    store_iter(false, 0).unwrap();
    let html: String = read_to_string("src/functions/speed_send_app/html_signers.json").unwrap();
    let mut html_signers: Signers = from_str(&html).unwrap();
    html_signers.signers = vec![];
    write("src/functions/speed_send_app/html_signers.json", to_string(&html_signers).unwrap()).unwrap();
    let js: String = read_to_string("src/functions/speed_send_app/js_signers.json").unwrap();
    let mut js_signers: Signers = from_str(&js).unwrap();
    js_signers.signers = vec![];
    write("src/functions/speed_send_app/js_signers.json", to_string(&js_signers).unwrap()).unwrap();
    for i in 0..4 {
        let file: File = OpenOptions::new().write(true).truncate(true).open(path[i].to_owned()).expect("Error");
        let mut write_file: File = file.try_clone().expect("Error");
        write_file.write_all(format!("{}",content[i]).as_bytes()).unwrap(); 
    }
    delete_all_files(Path::new("src/functions/build_app")).unwrap();
}

#[post("/")]
pub fn index() {
    clear()
}

pub fn delete_all_files(dir: &Path) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                delete_all_files(&path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
    } else {
        fs::remove_file(dir)?;
    }
    Ok(())
}
