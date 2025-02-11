use anchor_lang::prelude::*;

#[account]
pub struct DustUsers{
    pub owner : Pubkey,
}