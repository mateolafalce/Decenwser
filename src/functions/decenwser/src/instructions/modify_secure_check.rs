/*
    -This function changes the state of the secure check so that only 
    the admin can change the content with his signature
*/

use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn modify_secure_check(
    ctx: Context<ModifySecureCheck>
) -> Result<()> {
    require!(ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(), ErrorCode::AuthorityError);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    if main_account.secure_check == false {
        main_account.secure_check = true
    } else {
        main_account.secure_check = false
    }
    Ok(())
}

#[derive(Accounts)]
pub struct ModifySecureCheck<'info> {
    #[account(
        mut,
        seeds = [&anchor_lang::solana_program::hash::hash(main_account.web_name.as_bytes()).to_bytes()],
        bump = main_account.bump_original
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}