/*
    -Send the corresponding sun to finance the upload of the app
*/
use crate::functions::{
    speed_send_app::signers::Signers,
    config_settings::cluster::cluster,
    constants::program_id,
    encode_output::{html, js},
    send_app::get_wallet::get_wallet,
};
use anchor_client::{
    solana_sdk::{
        system_transaction::transfer,
        pubkey::Pubkey, 
        signature::{
            keypair_from_seed, 
            Keypair, 
            Signer
        }
    },
    Client, Program,
};
use anyhow::{Result, Ok};
use rocket::serde::json::from_str;
use std::{rc::Rc, str::FromStr, fs::read_to_string};

pub fn send_sol_to_signers(html_js: String) -> Result<()> {
    let html: String = read_to_string("src/functions/speed_send_app/html_signers.json").unwrap();
    let html_signers: Signers = from_str(&html).unwrap();
    let js: String = read_to_string("src/functions/speed_send_app/js_signers.json").unwrap();
    let js_signers: Signers = from_str(&js).unwrap();
    let amount: u64 = 7452200;
    let program: Program = Client::new(
        cluster().unwrap(),
        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    let mut counter: usize = 0;
    if html_js == "HTML" {
        while counter < html::DATA.len() {
            let keypair: Keypair = keypair_from_seed(&html_signers.signers[counter]).unwrap();
            let transfer = transfer(
                &keypair_from_seed(&get_wallet()).unwrap(), 
                &keypair.pubkey(), 
                amount, 
                program.rpc().get_latest_blockhash()?
            );
            program.rpc().send_and_confirm_transaction(&transfer)?;
            counter += 1;
        }
    } else if html_js == "JS" {
        while counter < js::DATA.len() {
            let keypair: Keypair = keypair_from_seed(&js_signers.signers[counter]).unwrap();
            let transfer = transfer(
                &keypair_from_seed(&get_wallet()).unwrap(), 
                &keypair.pubkey(), 
                amount, 
                program.rpc().get_latest_blockhash()?
            );
            program.rpc().send_and_confirm_transaction(&transfer)?;
            counter += 1;
        }
    }
    Ok(())
}

#[post("/", data = "<html_js>")]
pub fn index(html_js: String) {
    send_sol_to_signers(html_js).unwrap()
}
