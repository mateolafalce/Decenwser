use crate::functions::{
    speed_send_app::signers::Signers,
    config_settings::cluster::cluster,
    constants::program_id,
    encode_output::{html, js},
    send_app::get_wallet::get_wallet,
};
use anchor_client::{
    anchor_lang::{
        solana_program::{
            program::invoke,
            system_instruction::transfer
        },
    },
    solana_sdk::{
        account::Account,
        account_info::AccountInfo,
        instruction::Instruction, 
        pubkey::Pubkey, 
        signature::{
            keypair_from_seed, 
            Keypair, 
            Signer
        }
    },
    Client, Program,
};
use anyhow::{Result, Ok};
use rocket::serde::json::from_str;
use std::{rc::Rc, str::FromStr, fs::read_to_string, cell::RefCell};

pub fn send_sol_to_signers(html_js: String) -> Result<()> {
    let contents: String = read_to_string("src/functions/speed_send_app/signers.json").unwrap();
    let signers: Signers = from_str(&contents).unwrap();
    let amount: u64 = 7452200;
    let program: Program = Client::new(
        cluster().unwrap(),
        Rc::new(keypair_from_seed(&get_wallet()).expect("Example requires a keypair file")),
    )
    .program(Pubkey::from_str(&program_id::ID).unwrap());
    let mut user_data: Account = program.rpc().get_account(
        &keypair_from_seed(&get_wallet()).unwrap().pubkey()
    ).unwrap();
    let account_info_user: AccountInfo = AccountInfo {
        key: &user_data.owner,
        is_signer: true,
        is_writable: true,
        data: Rc::new(RefCell::new(&mut user_data.data)),
        lamports: Rc::new(RefCell::new(&mut user_data.lamports)),
        owner: &user_data.owner,
        executable: user_data.executable,
        rent_epoch: user_data.rent_epoch,
    };
    let mut counter: usize = 0;
    if html_js == "HTML" {
        while counter < html::DATA.len() {
            let keypair: Keypair = keypair_from_seed(&signers.signers[counter]).unwrap();
            let signer_data: Account = program.rpc().get_account(&keypair.pubkey()).unwrap();
            let owner_rc = Rc::new(signer_data.owner.clone());
            let account_info_signer: AccountInfo = AccountInfo {
                key: &owner_rc,
                is_signer: false,
                is_writable: true,
                data: Rc::new(RefCell::new(&mut signer_data.data.clone())),
                lamports: Rc::new(RefCell::new(&mut signer_data.lamports.clone())),
                owner: &owner_rc,
                executable: signer_data.executable.clone(),
                rent_epoch: signer_data.rent_epoch.clone(),
            };
            invoke( 
                &transfer(&program.payer(), &keypair.pubkey(), amount),
                &[account_info_user.clone(), account_info_signer.clone()]
            );
            counter += 1;
        }
    } /*else {

      }*/

    Ok(())
}
