/*
    -In this instruction the web account is created where the bump, the admin pubkey, 
    the domain, the html and the js are stored.
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
    let (_pda, bump) = Pubkey::find_program_address(&
        [&anchor_lang::solana_program::hash::hash(web_name.as_bytes()).to_bytes()
        ], 
        ctx.program_id
    );
    main_account.bump_original = bump;
    main_account.web_name = web_name;
    main_account.authority = ctx.accounts.signer.key();
    main_account.html = 0;
    main_account.js = 0;
    msg!(
        "{} is part of the international decentralized information interchange. Authority: {}",
        main_account.web_name,
        ctx.accounts.signer.key()
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(web_name: String)]
pub struct MainAccountStruct<'info> {
    #[account(init, 
        seeds = [
            &anchor_lang::solana_program::hash::hash(web_name.as_bytes()).to_bytes()
            ], 
            bump, 
            payer = signer, 
            space = MainAccount::SIZE + 8
        )
    ]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}