use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::state::*;

fn dust_user(ctx: Context<DustUsers>, amount: u64, to: Pubkey, from: Pubkey) -> Result<()> {
    Ok(())
}
#[derive(Accounts)]
pub struct Dust<'info> {
    #[account(mut)]
    pub dust_account: Account<'info, Dust>,
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Safe because this is a system transfer
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}