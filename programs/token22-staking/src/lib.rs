pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("Dew7MkBAqFZGNnS2sy5JwqBRbiiPPj6gU6MKCi6kLtXm");

#[program]
pub mod token_22_staking {
    use super::*;
    
    pub fn init_pool(ctx: Context<InitializePool>) -> Result<()> {
        msg!("Creating staking pool...");

        init_pool::handler(ctx)
    }

    pub fn init_stake_entry(ctx: Context<InitializeStakeEntry>) -> Result<()> {
        msg!("Creating staking entry...");

        init_stake_entry::handler(ctx)
    }
    
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        msg!("Staking...");

        stake::handler(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        msg!("Unstaking...");
        
        unstake::handler(ctx)
    }
}
// 68de6a3761499e9b5330102b5c5c18880a229035995fd6189458c1bd11b66ae2
// Dew7MkBAqFZGNnS2sy5JwqBRbiiPPj6gU6MKCi6kLtXm
// Signature: uWnkviPuD8Gtekr5PyXeDjA84mBx7b4rkxNrPYgkSWWC69Q5nrEFYSifbXH9zasJdT4UggDsvcdarmLtYwrvkjR