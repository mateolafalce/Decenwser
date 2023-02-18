use anchor_client::{
    anchor_lang::{solana_program::hash::hash, system_program, Key},
    solana_sdk::{
        pubkey::Pubkey,
        signature::{keypair_from_seed},
        },
    Client, Program
};
use anyhow::{Result};
use rocket::serde::{Deserialize, Serialize, json::{Json,from_str}};
use std::{result::Result::Ok, io::Error, rc::Rc, str::FromStr, fs::read_to_string};
use crate::functions::{
    speed_send_app::signers::Signers,
    constants::program_id,
    get_page::get_domain::get_domain,
    config_settings::cluster::cluster,
    encode_output::{html, js}
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Webdata {
    pub html_js: String,
    pub iter: u16,
}

pub fn speed_send_app(html_js: String, iter: u16) -> Result<(), Error> {
    let html: String = read_to_string("src/functions/speed_send_app/html_signers.json").unwrap();
    let html_signers: Signers = from_str(&html).unwrap();
    let js: String = read_to_string("src/functions/speed_send_app/js_signers.json").unwrap();
    let js_signers: Signers = from_str(&js).unwrap();
    if html_js == "HTML" {
        let program: Program = Client::new(
            cluster().unwrap(),
            Rc::new(keypair_from_seed(&html_signers.signers[iter as usize]).expect("Example requires a keypair file")),
        )
        .program(Pubkey::from_str(&program_id::ID).unwrap());
        let (main_account, _bump): (Pubkey, u8) =
                Pubkey::find_program_address(&[&hash(get_domain().unwrap().as_bytes()).to_bytes()], &program.id());
        send_html(main_account, program, html::DATA[iter as usize].to_string(), iter).unwrap();
    } else if html_js == "JS" {
        let program: Program = Client::new(
            cluster().unwrap(),
            Rc::new(keypair_from_seed(&js_signers.signers[iter as usize]).expect("Example requires a keypair file")),
        )
        .program(Pubkey::from_str(&program_id::ID).unwrap());
        let (main_account, _bump): (Pubkey, u8) =
                Pubkey::find_program_address(&[&hash(get_domain().unwrap().as_bytes()).to_bytes()], &program.id());
        send_js(main_account, program, js::DATA[iter as usize].to_string(), iter).unwrap();
    }
    Ok(())
}

pub fn send_html(main_account: Pubkey, program: Program, content: String, len: u16) -> Result<()> {
    let (store, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            b"HTML", 
            (len as usize).to_le_bytes().as_ref(), 
            main_account.key().as_ref()
        ],
        &program.id(),
    );
    program
        .request()
        .accounts(decenwser::accounts::SpeedHtmlStore {
            main_account,
            store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::SpeedHtmlStore { 
            content: content,
            len: len,
        })
        .send()?;
    Ok(())
}
pub fn send_js(main_account: Pubkey, program: Program, content: String, len: u16) -> Result<()> {
    let (store, _bump): (Pubkey, u8) = Pubkey::find_program_address(
        &[
            b"JS", 
            (len as usize).to_le_bytes().as_ref(), 
            main_account.key().as_ref()
        ],
        &program.id(),
    );
    program
        .request()
        .accounts(decenwser::accounts::SpeedJsStore {
            main_account,
            store,
            signer: program.payer(),
            system_program: system_program::ID,
        })
        .args(decenwser::instruction::SpeedJsStore { 
            content: content,
            len: len,
        })
        .send()?;
    Ok(())
}

#[post("/", data = "<web_data>")]
pub fn index(web_data: Json<Webdata>) {
    match speed_send_app(web_data.html_js.clone(), web_data.iter.clone()) {
        Ok(()) => println!("Account successfully sent to the solana blockchain"),
        Err(error) => println!("The account cannot be sent to the blockchain. Error: {}", error),
    }
}
