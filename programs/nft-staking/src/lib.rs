#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("D37tSyjmXxmvA6r8uBQ7T7qLcf6d8Fo5rmZNfDd3UiSn");

#[program]
pub mod staking {
    use super::*;

    pub fn initializeconfig(ctx: Context<InitializeConfig>) -> Result<()> {
        Ok(())
    }

    pub fn initializeuser(ctx: Context<InitializeUser>) -> Result<()> {
         Ok(())
    }
    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        Ok(())
    }
}

