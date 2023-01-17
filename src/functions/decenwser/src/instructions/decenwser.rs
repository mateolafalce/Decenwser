use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey
}; 
use crate::state::accounts::*;

pub fn decenwser(
    ctx: Context<Decenwser>
) -> Result<()> {
    let (_pda, bump) = Pubkey::find_program_address(&[b"Decenwser"], ctx.program_id);
    let decenwser: &mut Account<DecenwserAccount> = &mut ctx.accounts.decenwser;
    decenwser.pages_online = 0; 
    decenwser.total_updates = 0;
    decenwser.bump_original = bump;
    Ok(())
}

#[derive(Accounts)]
pub struct Decenwser<'info> {
    #[account(init, seeds = [b"Decenwser"], bump, payer = signer, space = DecenwserAccount::SIZE + 8)]
    pub decenwser: Account<'info, DecenwserAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}