use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn add_js(
    ctx: Context<AddJs>,
    content: Vec<u8>
) -> Result<()> {
    require!(
        ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(), 
        ErrorCode::AuthorityError
    );
    require!(
        ctx.accounts.store.content.len() < 9900, 
        ErrorCode::Max9900
    );
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let store: &mut Account<StoreAccount> = &mut ctx.accounts.store;
    store.content.extend(content);
    msg!("The content of the PDA was updated.");
    if store.content.len() == 9900 {
        main_account.js += 1;
    }
    Ok(())
}

#[derive(Accounts)]
pub struct AddJs<'info> {
    #[account(
        mut,
        seeds = [
            &anchor_lang::solana_program::hash::hash(
                main_account.web_name.as_bytes()
            ).to_bytes()
            ],
        bump = main_account.bump_original
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(
        mut, 
        seeds = [
            b"JS", 
            main_account.js.to_le_bytes().as_ref(), 
            main_account.key().as_ref()
        ], 
        bump = store.bump_original,
        realloc = 8 + 4 + 1 + store.content.len() + 900,
        realloc::payer = signer,
        realloc::zero = false,
    )]
    pub store: Account<'info, StoreAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}