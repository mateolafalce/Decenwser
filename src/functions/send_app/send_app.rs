/*
    -It sends the html or js depending on what was requested from the frontend. 
    Repetitive requests truncate the transaction, so the way in which errors are 
    minimized at the time of sending is to do it with rust, controlling the results. 
    The wallet is inserted locally to sign each time it is requested
*/

use anchor_client::{
    anchor_lang::{solana_program::hash::hash, system_program, Key},
    solana_sdk::{
        pubkey::Pubkey,
        signature::{keypair_from_seed},
        },
    Client, Program
};
use anyhow::{Result};
use decenwser::state::MainAccount;
use std::{result::Result::Ok, io::Error, rc::Rc, str::FromStr};
use crate::functions::{
    constants::program_id,
    get_page::get_domain::get_domain,
    send_app::{
        store_iter::store_iter,
        get_wallet::get_wallet
    },
    config_settings::cluster::cluster,
    encode_output::{html, js}
};

pub fn send_app(html_js: String) -> Result<(), Error> {
    let program: Program = Client::new(
        cluster().unwrap(),
        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    let (main_account, _bump): (Pubkey, u8) =
            Pubkey::find_program_address(&[&hash(get_domain().unwrap().as_bytes()).to_bytes()], &program.id());
    if html_js == "HTML" {
        let mut counter: usize = 0;
        while counter < html::DATA.len() {
            let program: Program = Client::new(
                cluster().unwrap(),
                Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
            ).program(Pubkey::from_str(&program_id::ID).unwrap());
            let main_account_pda: MainAccount = program.account(main_account).unwrap();
            send_html(main_account, main_account_pda, program, html::DATA[counter].to_string()).unwrap();
            store_iter(true,counter as u16).unwrap();
            counter += 1;
        }
    }
    if html_js == "JS" {
        let mut counter: usize = 0;
        while counter < js::DATA.len() {
            let program: Program = Client::new(
                cluster().unwrap(),
                Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
            ).program(Pubkey::from_str(&program_id::ID).unwrap());
            let main_account_pda: MainAccount = program.account(main_account).unwrap();
            send_js(main_account, main_account_pda, program, js::DATA[counter].to_string()).unwrap();
            store_iter(false,counter as u16).unwrap();
            counter += 1;
        }
    }
    Ok(())
}

pub fn send_html(main_account: Pubkey, main_account_pda: MainAccount, program: Program, content: String) -> Result<()> {
    let (store, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            b"HTML", 
            main_account_pda.html.len().to_le_bytes().as_ref(), 
            main_account.key().as_ref()
        ],
        &program.id(),
    );
    program
        .request()
        .accounts(decenwser::accounts::HtmlStore {
            main_account,
            store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::HtmlStore { 
            content: content,
        })
        .send()?;
    Ok(())
}
pub fn send_js(main_account: Pubkey, main_account_pda: MainAccount, program: Program, content: String) -> Result<()> {
    let (store, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            b"JS", 
            main_account_pda.js.len().to_le_bytes().as_ref(), 
            main_account.key().as_ref()
        ],
        &program.id(),
    );
    program
        .request()
        .accounts(decenwser::accounts::JsStore {
            main_account,
            store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::JsStore { 
            content: content,
        })
        .send()?;
    Ok(())
}
#[post("/", data = "<html_js>")]
pub fn index(html_js: String) {
    match send_app(html_js) {
        Ok(()) => println!("Account successfully sent to the solana blockchain"),
        Err(error) => println!("The account cannot be sent to the blockchain. Error: {}", error),
    }
}
