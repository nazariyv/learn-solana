use anchor_lang::prelude::*;

declare_id!("HGuq6WzaMQ4DaHyK148JonXBHv74Utw26c6itXZK6Tgs");

#[program]
pub mod sysvar {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
