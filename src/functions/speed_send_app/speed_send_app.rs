use anchor_client::{
    anchor_lang::{solana_program::hash::hash},
    solana_sdk::{
        pubkey::Pubkey,
        signature::{keypair_from_seed},
        },
    Client, Program
};
use anyhow::{Result};
use decenwser::state::MainAccount;
use rocket::serde::{Deserialize, Serialize, json::{Json,from_str}};
use std::{result::Result::Ok, io::Error, rc::Rc, str::FromStr, fs::read_to_string};
use crate::functions::{
    speed_send_app::signers::Signers,
    constants::program_id,
    get_page::get_domain::get_domain,
    send_app::{
        send_app::send_html,
        send_app::send_js,
        store_iter::store_iter,
    },
    config_settings::cluster::cluster,
    encode_output::{html, js}
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Webdata {
    pub html_js: String,
    pub iter: usize,
}

pub fn speed_send_app(html_js: String, iter: usize) -> Result<(), Error> {
    let contents: String = read_to_string("src/functions/speed_send_app/signers.json").unwrap();
    let signers: Signers = from_str(&contents).unwrap();
    let program: Program = Client::new(
        cluster().unwrap(),
        Rc::new(keypair_from_seed(&signers.signers[iter]).expect("Example requires a keypair file")),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    let (main_account, _bump): (Pubkey, u8) =
            Pubkey::find_program_address(&[&hash(get_domain().unwrap().as_bytes()).to_bytes()], &program.id());
    if html_js == "HTML" {
        let main_account_pda: MainAccount = program.account(main_account).unwrap();
        send_html(main_account, main_account_pda, program, html::DATA[iter].to_string()).unwrap();
        store_iter(true,iter as u16).unwrap();
    }else {
        let main_account_pda: MainAccount = program.account(main_account).unwrap();
        send_js(main_account, main_account_pda, program, js::DATA[iter].to_string()).unwrap();
        store_iter(false,iter as u16).unwrap();
    }
    Ok(())
}

#[post("/", data = "<web_data>")]
pub fn index(web_data: Json<Webdata>) {
    match speed_send_app(web_data.html_js.clone(), web_data.iter.clone()) {
        Ok(()) => println!("Account successfully sent to the solana blockchain"),
        Err(error) => println!("The account cannot be sent to the blockchain. Error: {}", error),
    }
}
