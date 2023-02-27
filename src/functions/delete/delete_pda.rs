/*
    - Remove the pdas by iterating one by one until leaving the html or js empty of content.
*/

use anchor_client::{
    anchor_lang::{solana_program::hash::hash, system_program, Key},
    solana_sdk::{
        pubkey::Pubkey,
        signature::{keypair_from_seed},
        },
    Client, Program
};
use anyhow::{Ok, Result};
use decenwser::state::MainAccount;
use std::{rc::Rc, str::FromStr};
use crate::functions::{
    send_app::get_wallet::get_wallet,
    get_page::get_domain::get_domain,
    constants::program_id,
    config_settings::cluster::cluster,
};

pub fn delete_app(html_js: String) {
    let program: Program = Client::new(
        cluster().unwrap(),
        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    let (main_account, _bump): (Pubkey, u8) =
            Pubkey::find_program_address(&[&hash(get_domain().unwrap().as_bytes()).to_bytes()], &program.id());
    if html_js == "HTML" {
        let main_account_pda: MainAccount = program.account(main_account).unwrap();
        let mut counter: usize = 0;
        while counter < main_account_pda.html as usize {
            let program: Program = Client::new(
                cluster().unwrap(),
                Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
            ).program(Pubkey::from_str(&program_id::ID).unwrap());
            delete_html(main_account, program.account(main_account).unwrap(), program).unwrap();
            counter += 1;
        }
    }
    if html_js == "JS" {
        let main_account_pda: MainAccount = program.account(main_account).unwrap();
        let mut counter: usize = 0;
        while counter < main_account_pda.js as usize  {
            let program: Program = Client::new(
                cluster().unwrap(),
                Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
            ).program(Pubkey::from_str(&program_id::ID).unwrap());
            delete_js(main_account, program.account(main_account).unwrap(), program).unwrap();
            counter += 1;
        }
    }
}

pub fn delete_html(main_account: Pubkey, main_account_pda: MainAccount, program: Program) -> Result<()> {
    let (account, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            b"HTML", 
            (main_account_pda.html - 1).to_le_bytes().as_ref(), 
            main_account.key().as_ref()
        ],
        &program.id(),
    );
    program
        .request()
        .accounts(decenwser::accounts::DeleteHtml {
            main_account,
            account,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::DeleteHtml {})
        .send()?;
    Ok(())
}
pub fn delete_js(main_account: Pubkey, main_account_pda: MainAccount, program: Program) -> Result<()> {
    let (account, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            b"JS", 
            (main_account_pda.js - 1).to_le_bytes().as_ref(), 
            main_account.key().as_ref()
        ],
        &program.id(),
    );
    program
        .request()
        .accounts(decenwser::accounts::DeleteJs {
            main_account,
            account,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::DeleteJs {})
        .send()?;
    Ok(())
}
#[post("/", data = "<html_js>")]
pub fn index(html_js: String) {
    delete_app(html_js)
}
