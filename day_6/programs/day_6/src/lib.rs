use anchor_lang::prelude::*;

declare_id!("2Qv2RPwMfcrSYVK5bN8BDUDdTAmeEEHWwHp4BBy4NZri");

#[program]
pub mod day_6 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
