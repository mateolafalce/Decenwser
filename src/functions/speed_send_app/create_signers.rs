use anchor_client::{
    solana_sdk::{
        pubkey::Pubkey,
        signature::Keypair,
    },
};
use rocket::serde::json::{Json, to_string, from_str};
use anyhow::{Result, Ok};
use functions::speed_send_app::signers::Signers;

pub fn create_signers(total: u16) -> Result<()> {
    for i in 0..total {
        let contents: String = read_to_string("src/functions/speed_send_app/signers.json").unwrap();
        let mut config: Signers = from_str(&contents).unwrap();
        config[i] = Keypair::new();
        write("src/functions/speed_send_app/signers.json", to_string(&config).unwrap()).unwrap();
    }
    Ok(())
}

#[post("/", data = "<total>")]
pub fn index(total: Json<u16>) {
    create_signers(*total).unwrap()
}
