use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use crate::state::accounts::*;
use crate::error::ErrorCode;

pub fn html_store(
    ctx: Context<HtmlStore>,
    html: String,
) -> Result<()> {
    require!(ctx.accounts.main_account.authority.key() == ctx.accounts.signer.key(), ErrorCode::AuthorityError);
    let html_store: &mut Account<HTML> = &mut ctx.accounts.html_store;
    require!(8 + ctx.accounts.main_account.len < 9995, ErrorCode::TooLong);
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    html_store.html = html;
    main_account.html.push(ctx.accounts.decenwser.total_updates);
    let decenwser: &mut Account<DecenwserAccount> = &mut ctx.accounts.decenwser;
    decenwser.total_updates += 1;
    main_account.len += 8;
    Ok(())
}

#[derive(Accounts)]
pub struct HtmlStore<'info> {
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
    bump, payer = signer, space = HTML::SIZE + 8)]
    pub html_store: Account<'info, HTML>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}