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
use anyhow::Result;
use decenwser::state::MainAccount;
use rocket::serde::json::from_str;
use std::{
    result::Result::Ok, 
    io::Error, 
    rc::Rc, 
    str::FromStr,
    fs::read_to_string
};
use crate::functions::{
    constants::{
        program_id,
        encode::Encode
    },
    get_page::get_domain::get_domain,
    send_app::{
        get_wallet::get_wallet,
        store_iter::store_iter
    },
    config_settings::cluster::cluster,
};

pub fn send_app(html_js: String) -> Result<(), Error> {
    let html_len: Encode = from_str(&read_to_string("src/functions/encode_output/encode_html.json").unwrap()).unwrap();
    let js_len: Encode = from_str(&read_to_string("src/functions/encode_output/encode_js.json").unwrap()).unwrap();
    let program: Program = Client::new(
        cluster().unwrap(),
        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    let (main_account, _bump): (Pubkey, u8) =
            Pubkey::find_program_address(&[&hash(get_domain().unwrap().as_bytes()).to_bytes()], &program.id());
    if html_js == "HTML" {
        let mut counter: usize = 0;
        while counter < html_len.content.len() {
            let mut success = false;
            let mut retries = 0;
            while !success && retries < 100 {
                let main_account_pda: MainAccount = program.account(main_account).unwrap();
                let (pda, _bump): (Pubkey, u8) = Pubkey::find_program_address(
                    &[
                        b"HTML", 
                        main_account_pda.html.to_le_bytes().as_ref(), 
                        main_account.key().as_ref()
                    ],
                    &Pubkey::from_str(&program_id::ID).unwrap(),
                );
                let balance = program.rpc().get_balance(&pda).unwrap();
                if balance == 0 {
                    let program: Program = Client::new(
                        cluster().unwrap(),
                        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
                    ).program(Pubkey::from_str(&program_id::ID).unwrap());
                    let main_account_pda: MainAccount = program.account(main_account).unwrap();
                    html_store(main_account, main_account_pda, program, html_len.content[counter].to_vec()).unwrap();
                    println!("Successfully sent to the solana blockchain[{}]", counter);
                    store_iter(true).unwrap();
                    counter += 1;
             } else {
                if html_len.content.len() == 1{
                    println!("Content successfully sent to the blockchain. IGNORE THE MESSAGE BELOW");
                    break
                }
                let program: Program = Client::new(
                    cluster().unwrap(),
                    Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
                ).program(Pubkey::from_str(&program_id::ID).unwrap());
                let main_account_pda: MainAccount = program.account(main_account).unwrap();
                match add_html(main_account, main_account_pda, program, html_len.content[counter].to_vec()) {
                    Ok(_) => {
                        success = true;
                        println!("Successfully sent to the solana blockchain[{}]", counter);
                        store_iter(true).unwrap();
                        counter += 1;
                    },
                    Err(e) => {
                        retries += 1;
                        println!("Error sending to the solana blockchain[{}]: {}. Retrying...", counter, e);
                    }
                }
            }
        }
        if !success {
            println!("Failed to send to the solana blockchain after 10 retries. Aborting...");
            break;
        }
        }
    }
    if html_js == "JS" {
        let mut counter: usize = 0;
        while counter < js_len.content.len() {
            let mut success = false;
            let mut retries = 0;
            while !success && retries < 100 {
                let main_account_pda: MainAccount = program.account(main_account).unwrap();
                let (pda, _bump): (Pubkey, u8) = Pubkey::find_program_address(
                    &[
                        b"JS", 
                        main_account_pda.js.to_le_bytes().as_ref(), 
                        main_account.key().as_ref()
                    ],
                    &Pubkey::from_str(&program_id::ID).unwrap(),
                );
                let balance = program.rpc().get_balance(&pda).unwrap();
                if balance == 0 {
                    let program: Program = Client::new(
                        cluster().unwrap(),
                        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
                    ).program(Pubkey::from_str(&program_id::ID).unwrap());
                    let main_account_pda: MainAccount = program.account(main_account).unwrap();
                    js_store(main_account, main_account_pda, program, js_len.content[counter].to_vec()).unwrap();
                    println!("Successfully sent to the solana blockchain[{}]", counter);
                    store_iter(true).unwrap();
                    counter += 1;
             } else {
                if js_len.content.len() == 1{
                    println!("Content successfully sent to the blockchain. IGNORE THE MESSAGE BELOW");
                    break
                }
                let program: Program = Client::new(
                    cluster().unwrap(),
                    Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
                ).program(Pubkey::from_str(&program_id::ID).unwrap());
                let main_account_pda: MainAccount = program.account(main_account).unwrap();
                match add_js(main_account, main_account_pda, program, js_len.content[counter].to_vec()) {
                    Ok(_) => {
                        success = true;
                        println!("Successfully sent to the solana blockchain[{}]", counter);
                        store_iter(true).unwrap();
                        counter += 1;
                    },
                    Err(e) => {
                        retries += 1;
                        println!("Error sending to the solana blockchain[{}]: {}. Retrying...", counter, e);
                    }
                }
             }
        }
        if !success {
            println!("Failed to send to the solana blockchain after 10 retries. Aborting...");
            break;
        }
        }
    }
    Ok(())
}

pub fn html_store(main_account: Pubkey, main_account_pda: MainAccount, program: Program, content: Vec<u8>) -> Result<(), anchor_client::ClientError> {
    let (store, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            b"HTML", 
            main_account_pda.html.to_le_bytes().as_ref(), 
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
pub fn add_html(main_account: Pubkey, main_account_pda: MainAccount, program: Program, content: Vec<u8>) -> Result<(), anchor_client::ClientError> {
    let (store, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            b"HTML", 
            main_account_pda.html.to_le_bytes().as_ref(), 
            main_account.key().as_ref()
        ],
        &program.id(),
    );
    program
        .request()
        .accounts(decenwser::accounts::AddHtml {
            main_account,
            store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::AddHtml { 
            content: content,
        })
        .send()?;
    Ok(())
}
pub fn js_store(main_account: Pubkey, main_account_pda: MainAccount, program: Program, content: Vec<u8>) -> Result<(), anchor_client::ClientError> {
    let (store, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            b"JS", 
            main_account_pda.js.to_le_bytes().as_ref(), 
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
pub fn add_js(main_account: Pubkey, main_account_pda: MainAccount, program: Program, content: Vec<u8>) -> Result<(), anchor_client::ClientError> {
    let (store, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            b"JS", 
            main_account_pda.js.to_le_bytes().as_ref(), 
            main_account.key().as_ref()
        ],
        &program.id(),
    );
    program
        .request()
        .accounts(decenwser::accounts::AddJs {
            main_account,
            store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::AddJs { 
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
