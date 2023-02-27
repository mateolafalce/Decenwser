use anchor_client::{
    anchor_lang::{solana_program::hash::hash, Key},
    solana_sdk::{
        pubkey::Pubkey,
        signature::{keypair_from_seed},
        },
    Client, Program
};
use anyhow::{Ok, Result};
use rocket::serde::json::{from_str, to_string};
use std::{str::FromStr, rc::Rc, fs::{write, read_to_string}};
use crate::functions::{
    constants::{program_id, pdas::Pdas},
    get_page::{get_domain::get_domain, get_len::get_len},
    send_app::get_wallet::get_wallet,
    config_settings::cluster::cluster,
};

pub fn store_pdas() -> Result<()> {
    let mut html_pda: Pdas = from_str(&read_to_string("src/functions/get_page/html_pdas.json").unwrap()).unwrap();
    let mut js_pda: Pdas = from_str(&read_to_string("src/functions/get_page/js_pdas.json").unwrap()).unwrap();
    let program: Program = Client::new(
        cluster().unwrap(),
        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    let (main_account, _bump): (Pubkey, u8) =
            Pubkey::find_program_address(&[&hash(get_domain().unwrap().as_bytes()).to_bytes()], &program.id());
    for i in 0..(get_len().unwrap().len_html + 1) {
        let (pbk, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[
            b"HTML",
            (i as u16).to_le_bytes().as_ref(),
            main_account.key().as_ref(),
        ], &program.id());
        html_pda.pdas.push(pbk.to_string());
    }
    write("src/functions/get_page/html_pdas.json", to_string(&html_pda).unwrap()).unwrap();
    for i in 0..(get_len().unwrap().len_js + 1) {
        let (pbk, _bump): (Pubkey, u8) =
            Pubkey::find_program_address(&[
                b"JS",
                (i as u16).to_le_bytes().as_ref(),
                main_account.key().as_ref(),
            ], &program.id());
        js_pda.pdas.push(pbk.to_string());
    }
    write("src/functions/get_page/js_pdas.json", to_string(&js_pda).unwrap()).unwrap();
    Ok(())
}

#[post("/")]
pub fn index() {
    store_pdas().unwrap()
}