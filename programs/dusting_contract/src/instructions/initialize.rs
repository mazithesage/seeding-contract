use anchor_lang::prelude:: *;
use anchor_spl::token::{self, Mint, Token, TokenAccount,};
use crate::state::*;
pub fn initialize_dust(ctx: Context<InitializeDust>) -> Result<()> {
    let dust_account = &mut ctx.accounts.dust_account;
    dust_account.owner = ctx.accounts.owner.key().clone();
    Ok(())
}

#[derive(Accounts, Clone)]
pub struct InitializeDust<'info> {
    #[account(init, payer = owner, space = 8 + 32)]
    pub dust_account: Account<'info, DustUsers>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}