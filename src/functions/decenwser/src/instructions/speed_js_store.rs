/*
    -Save the js content to the web. It uses different signers so it makes 
    the upload much faster.
*/

use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn speed_js_store(
    ctx: Context<SpeedJsStore>,
    len: u16,
    content: String,
) -> Result<()> {
    require!(ctx.accounts.main_account.secure_check == false, ErrorCode::SecureCheckError);
    require!(ctx.accounts.main_account.len <= 9984, ErrorCode::TooLong);
    let (_pda, bump) = Pubkey::find_program_address(&[b"JS", (len as usize).to_le_bytes().as_ref(), ctx.accounts.main_account.key().as_ref()], ctx.program_id);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let store: &mut Account<StoreAccount> = &mut ctx.accounts.store;
    store.content = content;
    store.bump_original = bump;
    main_account.js.push(len + 1);
    main_account.len += 2;
    Ok(())
}

#[derive(Accounts)]
#[instruction(len: u16)]
pub struct SpeedJsStore<'info> {
    #[account(
        mut,
        seeds = [&anchor_lang::solana_program::hash::hash(main_account.web_name.as_bytes()).to_bytes()],
        bump = main_account.bump_original,
        realloc = 8 + main_account.len as usize + 2,
        realloc::payer = signer,
        realloc::zero = false,
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(init, seeds = 
        [
        b"JS", 
        (len as usize).to_le_bytes().as_ref(),
        main_account.key().as_ref()
        ], 
    bump, payer = signer, 
    space = StoreAccount::SIZE + 8
    )]
    pub store: Account<'info, StoreAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}