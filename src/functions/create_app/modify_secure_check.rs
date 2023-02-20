/*
    -A function of the program running on the blockchain is executed 
    to create a utf-8 domain with a maximum of 32 characters.
*/

use anchor_client::{
    anchor_lang::solana_program::hash::hash,
    anchor_lang::system_program,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{keypair_from_seed, Signature},
    },
    Client, Program,
};
use anyhow::{Ok, Result};
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::{rc::Rc, str::FromStr};
use crate::functions::{
    send_app::get_wallet::get_wallet,
    get_page::get_domain::get_domain,
    config_settings::cluster::cluster,
    constants::program_id
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SecureCheck {
    pub tx: String,
}

pub fn modify_secure_check() -> Result<SecureCheck> {
    let program: Program = Client::new(
        cluster().unwrap(),
        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    let (main_account, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[&hash(&get_domain().unwrap().as_bytes()).to_bytes()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::ModifySecureCheck {
            main_account: main_account,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::ModifySecureCheck {})
        .send()?;
    let output: SecureCheck = SecureCheck {
        tx: tx.to_string(),
    };
    Ok(output)
}

#[post("/")]
pub fn index() -> Json<SecureCheck> {
    Json(modify_secure_check().unwrap())
}
