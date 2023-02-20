/*
    -In this instruction the web account is created where the bump, the admin pubkey, 
    the domain, the html and the js are stored. The secure check has the function of 
    validating or not future updates of the web code. By default it remains false so 
    that any signer can send their app to the blockchain and then close the secure 
    check to avoid modifications. In the event that the user wishes from the beginning 
    to change the status to true to sign only it and all the transactions consecutively, 
    it can be updated with the function modify_secure_check()
*/

use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn main_account(
    ctx: Context<MainAccountStruct>,
    web_name: String
) -> Result<()> {
    require!(web_name.len() <= 32, ErrorCode::TooLong);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let (_pda, bump) = Pubkey::find_program_address(&[&anchor_lang::solana_program::hash::hash(web_name.as_bytes()).to_bytes()], ctx.program_id);
    main_account.bump_original = bump;
    main_account.web_name = web_name;
    main_account.authority = ctx.accounts.signer.key();
    main_account.html = [].to_vec();
    main_account.js = [].to_vec();
    main_account.len = 83;
    main_account.secure_check = false;
    Ok(())
}

#[derive(Accounts)]
#[instruction(web_name: String)]
pub struct MainAccountStruct<'info> {
    #[account(init, seeds = [&anchor_lang::solana_program::hash::hash(web_name.as_bytes()).to_bytes()], bump, payer = signer, space = MainAccount::SIZE + 8)]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}