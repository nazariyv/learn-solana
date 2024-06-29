use anchor_lang::prelude::*;

declare_id!("6cZ2rQ5PnfQS1yG8cVeM1RbwHQ2ouSeEfbLFZm1gTRPn");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello World");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program<'info, System>,
}