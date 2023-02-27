use anchor_client::{
    anchor_lang::{Key, solana_program::hash::hash},
    solana_sdk::{
        pubkey::Pubkey,
        signature::{keypair_from_seed},
        },
    Client, Program
};
use rocket::serde::{Deserialize, Serialize, json::Json};
use anyhow::{Ok, Result};
use decenwser::state::MainAccount;
use std::{rc::Rc, str::FromStr};
use crate::functions::{
    send_app::get_wallet::get_wallet,
    constants::program_id,
    config_settings::cluster::cluster,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Domain {
    pub pubkey: String,
    pub bump_original: u8,   
    pub authority: String,   
    pub web_name: String,     
    pub html: u16,       
    pub js: u16,                          
}

pub fn get_data_domain(domain: String) -> Result<Domain> {
    let program: Program = Client::new(
        cluster().unwrap(),
        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    let (main_account, _bump): (Pubkey, u8) =
            Pubkey::find_program_address(&[&hash(domain.as_bytes()).to_bytes()], &program.id());    
    let main_account_pda: MainAccount = program.account(main_account).unwrap();
    let output: Domain = Domain {
        pubkey: main_account.key().to_string(),
        bump_original: main_account_pda.bump_original,
        authority: main_account_pda.authority.key().to_string(),
        web_name: main_account_pda.web_name,
        html: main_account_pda.html,
        js: main_account_pda.js,
    };
    Ok(output)
}

#[post("/", data = "<domain>")]
pub fn index(domain: String) -> Json<Domain>{
    Json(get_data_domain(domain).unwrap())
}
