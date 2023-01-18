use anchor_client::{
    anchor_lang::solana_program::hash::hash,
    anchor_lang::system_program,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{keypair_from_seed, Signature},
    },
    Client, Cluster,
};
use anyhow::{Ok, Result};
use decenwser::state::DecenwserAccount;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::rc::Rc;
use std::str::FromStr;
use crate::functions::constants::{wallet, program_id};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct JsTx {
    pub tx: String,
    pub pda: String,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Webdata {
    web_name: String,
    js: String
}

pub fn send_js(
    web_name: String,
    js: String 
) -> Result<JsTx> {
    let program = Client::new(
        Cluster::Devnet,
        Rc::new(
            keypair_from_seed(&wallet::ID).expect("Example requires a keypair file"),
        ),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    let (main_account, _bump) =
        Pubkey::find_program_address(&[&hash(web_name.as_bytes()).to_bytes()], &program.id());
    let (decenwser, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[b"Decenwser"], &program.id());
    let account: DecenwserAccount = program.account(decenwser)?;
    let (js_store, _bump): (Pubkey, u8) =
        Pubkey::find_program_address(&[&account.total_updates.to_le_bytes()], &program.id());
    let tx: Signature = program
        .request()
        .accounts(decenwser::accounts::JsStore {
            main_account,
            decenwser,
            js_store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::JsStore { js })
        .send()?;
    let output: JsTx = JsTx {
        tx: tx.to_string(),
        pda: js_store.to_string(),
    };
    Ok(output)
}

#[post("/", data = "<web_data>")]
pub fn index(web_data: Json<Webdata>) -> Json<JsTx> {
    Json(send_js(web_data.web_name.clone(), web_data.js.clone()).unwrap())
}
