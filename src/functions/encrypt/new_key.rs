use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::{keypair_from_seed, Keypair}},
    Client, Program,
};
use anyhow::{Result, Ok};
use std::{ 
    rc::Rc, str::FromStr, 
    fs, 
    io::Write, 
    fs::{write, read_to_string, File, OpenOptions,},
    path::Path
};
use decenwser::state::StoreAccount;
use crate::functions::{
    config_settings::cluster::cluster,
    send_app::{
        store_iter::store_iter,
        get_wallet::get_wallet
    },
    get_page::{
        get_len::get_len,
        get_domain::get_domain
    },
    constants::{program_id, pdas::Pdas}
};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Key {
    pub key: Vec<u8>,
}

fn new_key() -> Result<()>{
    let mut key_rust: Key = from_str(
        &read_to_string("src/functions/constants/key.json").unwrap()
    ).unwrap();
    key_rust.key = keypair_from_seed(Keypair::new().secret()).unwrap();
    write("src/functions/constants/key.json", to_string(&key_rust).unwrap()).unwrap();
}