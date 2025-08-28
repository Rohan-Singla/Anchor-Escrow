use anchor_lang::prelude::*;
mod state;
mod errors;
mod instructions;
use instructions::*;

declare_id!("D999vqfQD5C3y9avFf2tBPLntf7BLg47eBrmnmZsRH7Z");

#[program]
pub mod escrow_program {
    use super::*;

   #[instruction(discriminator = 0)]
    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        //...
    }
 
    #[instruction(discriminator = 1)]
    pub fn take(ctx: Context<Take>) -> Result<()> {
        //...
    }
 
    #[instruction(discriminator = 2)]
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        //...
    }
}


