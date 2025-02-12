use anchor_lang::prelude::*;

declare_id!("vfEDk65qkioSAhTJz7nV8R7g5K2q8eAsZa7bmsjYDE5");

#[program]
pub mod token_seeder {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
