use anchor_client::{
    anchor_lang::solana_program::hash::hash,
    solana_sdk::{pubkey::Pubkey, signature::Keypair},
    Client, Cluster, Program,
};
use anyhow::{Ok, Result};
use block::MainAccount;
use rocket::serde::{Deserialize, Serialize};

use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct App {
    pub html: Vec<String>,
    pub css: Vec<String>,
    pub js: Vec<String>,
}

pub fn get_page(domain: String) -> Result<App> {
    let program_id: Pubkey =
        Pubkey::from_str("9pSAYQj6tgJCv4Msii77smfYoY3sQGGLf89qnH9EBWry").unwrap();
    let client: Client = Client::new(Cluster::Devnet, Rc::new(Keypair::new()));
    let program: Program = client.program(program_id);
    let (pda, _bump) =
        Pubkey::find_program_address(&[&hash(domain.as_bytes()).to_bytes()], &program_id);
    let app_data: MainAccount = program.account(pda)?;
    let app: App = App {
        html: app_data.html,
        css: app_data.css,
        js: app_data.js,
    };
    Ok(app)
}
