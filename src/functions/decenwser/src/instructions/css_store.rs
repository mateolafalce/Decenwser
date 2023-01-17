use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn css_store(
    ctx: Context<CssStore>,
    css: String,
) -> Result<()> {
    require!(ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(), ErrorCode::AuthorityError);
    let css_store: &mut Account<CSS> = &mut ctx.accounts.css_store;
    require!(8 + ctx.accounts.main_account.len < 9995, ErrorCode::TooLong);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    css_store.css = css;
    main_account.css.push(ctx.accounts.decenwser.total_updates);
    let decenwser: &mut Account<DecenwserAccount> = &mut ctx.accounts.decenwser;
    decenwser.total_updates += 1;
    main_account.len += 8;
    Ok(())
}

#[derive(Accounts)]
pub struct CssStore<'info> {
    #[account(
        mut,
        seeds = [&anchor_lang::solana_program::hash::hash(main_account.web_name.as_bytes()).to_bytes()],
        bump = main_account.bump_original,
        realloc = 8 + main_account.len as usize + 8,
        realloc::payer = signer,
        realloc::zero = false,
    )]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut,seeds = [b"Decenwser"],bump = decenwser.bump_original)]
    pub decenwser: Account<'info, DecenwserAccount>,
    #[account(init, seeds = [&decenwser.total_updates.to_le_bytes()], 
    bump, payer = signer, space = JS::SIZE + 8)]
    pub css_store: Account<'info, CSS>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}