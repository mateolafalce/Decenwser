/*
    -Take the maximum size of the html and js.
*/

use anchor_client::{
    anchor_lang::{solana_program::hash::hash},
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
    Client, 
};
use anyhow::{Ok, Result};
use rocket::serde::json::{Json, to_string, from_str};
use std::{rc::Rc, str::FromStr, fs::{write, read_to_string}};
use decenwser::state::{MainAccount};
use crate::functions::{
    get_page::get_domain::get_domain,
    config_settings::cluster::cluster,
    constants::{program_id, max_len::Len}
};

pub fn store_len() -> Result<Len> {
    let program_id: Pubkey =
        Pubkey::from_str(&program_id::ID).unwrap();
    let (pda, _bump): (Pubkey, u8) =
            Pubkey::find_program_address(&[&hash(get_domain().unwrap().as_bytes()).to_bytes()], &program_id);
    let client: Client = Client::new(cluster().unwrap(), Rc::new(Keypair::new()));
    let app_data: MainAccount = client.program(program_id).account(pda).unwrap();
    let app: Len = Len {
        len_html: app_data.html.len() as u16,
        len_js: app_data.js.len() as u16 
    };
    let contents: String = read_to_string("src/functions/constants/max_len.json").unwrap();
    let mut config: Len = from_str(&contents).unwrap();
    config.len_html = app.len_html.clone();
    config.len_js = app.len_js.clone();
    write("src/functions/constants/max_len.json", to_string(&config).unwrap()).unwrap();
    Ok(app)
}

#[post("/")]
pub fn index() -> Json<Len> {
    Json(store_len().unwrap())
}