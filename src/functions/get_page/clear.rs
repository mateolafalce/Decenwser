/*
    -Removes the content in the page to render, in the domain, 
    in the build app result, and in the encoded result. 
    A request is made at render time /main
*/

use std::{
    io::Write, 
    fs::{
        OpenOptions, 
        File, 
        read_to_string, 
        write
    }
};
use rocket::serde::json::{to_string, from_str};
use crate::functions::{
    constants::{
        pdas::Pdas,
        state::State,
        encode::Encode
    },
    send_app::store_wallet::store_wallet
};

pub fn clear() {
    let mut mut_html_len: State = from_str(&read_to_string("state.json").unwrap()).unwrap();
    mut_html_len.len_html = 0;
    write("state.json", to_string(&mut_html_len).unwrap()).unwrap();
    let mut mut_js_len: State = from_str(&read_to_string("state.json").unwrap()).unwrap();
    mut_js_len.len_js = 0;
    write("state.json", to_string(&mut_js_len).unwrap()).unwrap();
    let mut html_len: Encode = from_str(&read_to_string("src/functions/encode_output/encode_html.json").unwrap()).unwrap();
    let mut js_len: Encode = from_str(&read_to_string("src/functions/encode_output/encode_js.json").unwrap()).unwrap();
    html_len.content = vec![];
    write("src/functions/encode_output/encode_html.json", to_string(&html_len).unwrap()).unwrap();
    js_len.content = vec![];
    write("src/functions/encode_output/encode_js.json", to_string(&js_len).unwrap()).unwrap();
    let path: [String;2] = ["templates/web.html.hbs".to_string(), "public/js.js".to_string()];
    let content: [String;2] = ["".to_string(), "".to_string()];
    let clear_wallet: Vec<u8> = vec![1].iter().cloned().cycle().take(64).collect::<Vec<u8>>();
    store_wallet(clear_wallet).unwrap();
    let mut mut_html_iter: State = from_str(&read_to_string("state.json").unwrap()).unwrap();
    mut_html_iter.html_iter = 0;
    write("state.json", to_string(&mut_html_iter).unwrap()).unwrap();
    let mut mut_js_iter: State = from_str(&read_to_string("state.json").unwrap()).unwrap();
    mut_js_iter.js_iter = 0;
    write("state.json", to_string(&mut_js_iter).unwrap()).unwrap();
    let mut html_pda: Pdas = from_str(&read_to_string("src/functions/get_page/html_pdas.json").unwrap()).unwrap();
    html_pda.pdas = vec![];
    write("src/functions/get_page/html_pdas.json", to_string(&html_pda).unwrap()).unwrap();
    let mut js_pda: Pdas = from_str(&read_to_string("src/functions/get_page/js_pdas.json").unwrap()).unwrap();
    js_pda.pdas = vec![];
    write("src/functions/get_page/js_pdas.json", to_string(&js_pda).unwrap()).unwrap();
    for i in 0..2 {
        let file: File = OpenOptions::new().write(true).truncate(true).open(path[i].to_owned()).expect("Error");
        let mut write_file: File = file.try_clone().expect("Error");
        write_file.write_all(format!("{}",content[i]).as_bytes()).unwrap(); 
    }
}

#[post("/")]
pub fn index() {
    clear()
}
