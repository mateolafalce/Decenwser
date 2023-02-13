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
    config_settings::cluster::cluster,
    constants::program_id
};


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AppCreated {
    pub tx: String,
    pub pda: String,
}

pub fn create_app(web_name: String) -> Result<AppCreated> {
    let program: Program = Client::new(
        cluster().unwrap(),
        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    Ok(create_app_tx(&program, web_name).unwrap())
}

pub fn create_app_tx(program: &Program, web_name: String) -> Result<AppCreated> {
    let (pda, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[&hash(web_name.as_bytes()).to_bytes()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::MainAccountStruct {
            main_account: pda,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::MainAccount { web_name })
        .send()?;
    let output: AppCreated = AppCreated {
        tx: tx.to_string(),
        pda: pda.to_string(),
    };
    Ok(output)
}

#[post("/", data = "<web_name>")]
pub fn index(web_name: String) -> Json<AppCreated> {
    Json(create_app(web_name).unwrap())
}
