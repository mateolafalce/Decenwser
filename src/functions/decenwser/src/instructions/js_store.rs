/*
    -Save js content on the web. This saving is consecutive and only 
    the private key of the authority can sign the transactions.
*/

use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn js_store(
    ctx: Context<JsStore>,
    content: Vec<u8>
) -> Result<()> {
    require!(ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(), ErrorCode::AuthorityError);
    let (_pda, bump) = Pubkey::find_program_address(&[
        b"JS", 
        ctx.accounts.main_account.js.to_le_bytes().as_ref(), 
        ctx.accounts.main_account.key().as_ref()
        ], 
        ctx.program_id);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let store: &mut Account<StoreAccount> = &mut ctx.accounts.store;
    store.content = content;
    store.bump_original = bump;
    msg!(
        "New account stores the JS content with seed = [JS, {}, {}]", 
        main_account.js,
        main_account.key()
    );
    Ok(())
}

#[derive(Accounts)]
pub struct JsStore<'info> {
    #[account(
        mut,
        seeds = [
            &anchor_lang::solana_program::hash::hash(
            main_account.web_name.as_bytes()
        ).to_bytes()
        ],
        bump = main_account.bump_original,
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(
        init, 
        seeds = 
            [
            b"JS", 
            main_account.js.to_le_bytes().as_ref(), 
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