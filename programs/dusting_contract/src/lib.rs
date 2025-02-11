use anchor_lang::prelude::*;

pub mod state;

use crate::instructions::initialize::*;
use crate::instructions::dust_user::*;
use crate::instructions::closing::*;
pub mod instructions;


declare_id!("3RAzYi3Si2HDBXC3LjENBrkgnaM8WmR3zF4hhXgve5mr");

pub mod dusting_contract {
    use super::*;

    pub fn initialize(ctx: Context<InitializeDust>) -> Result<()> {
        initialize_dust(ctx)
    }

    

}
