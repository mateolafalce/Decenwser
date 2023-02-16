use anchor_client::solana_sdk::signature::Keypair;
use rocket::serde::{Deserialize, Serialize,json::{Json, to_string, from_str}};
use anyhow::{Result, Ok};
use crate::functions::speed_send_app::signers::Signers;
use std::fs::{read_to_string, write};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Total {
    pub total: u16,
}

pub fn create_signers(total: u16) -> Result<()> {
    let contents: String = read_to_string("src/functions/speed_send_app/signers.json").unwrap();
    let mut signers: Signers = from_str(&contents).unwrap();
    for _i in 0..total {
        signers.signers.push(Keypair::new().to_bytes().to_vec());
        write("src/functions/speed_send_app/signers.json", to_string(&signers).unwrap()).unwrap();
    }
    Ok(())
}

#[post("/", data = "<total>")]
pub fn index(total: Json<Total>) {
    create_signers(total.total.clone()).unwrap()
}
