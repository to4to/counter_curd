use anchor_lang::prelude::*;

declare_id!("4rVANt2eVBVUpPE7d6EfBHcWRX81KjoeNJ389N5ZvNrh");

#[program]
pub mod counter_curd {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
