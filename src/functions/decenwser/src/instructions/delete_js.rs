/*
    -This function deletes the content that exists on the pda and returns 
    all the SOL to the respective authority. After doing that, the account 
    is closed.
*/

use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn delete_js(
    ctx: Context<DeleteJs>
) -> Result<()> {
    require!(ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(), ErrorCode::AuthorityError);
    let lamport: u64 = 7433280 - 890880;
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    main_account.js.pop();
    main_account.len -= 2;
    **ctx.accounts.account.to_account_info().try_borrow_mut_lamports()? -= lamport;
    **ctx.accounts.signer.to_account_info().try_borrow_mut_lamports()? += lamport;
    Ok(())
}
#[derive(Accounts)]
pub struct DeleteJs<'info> {
    #[account(
        mut,
        seeds = [&anchor_lang::solana_program::hash::hash(main_account.web_name.as_bytes()).to_bytes()],
        bump = main_account.bump_original,
        realloc = 8 + main_account.len as usize - 2,
        realloc::payer = signer,
        realloc::zero = false,
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(
        mut, 
        seeds = [
            b"JS", 
            (main_account.js.len() - 1_usize).to_le_bytes().as_ref(),  
            main_account.key().as_ref()
        ],  
        bump = account.bump_original, 
        close = signer
    )]
    pub account: Account<'info, StoreAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}