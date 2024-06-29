use anchor_lang::prelude::*;

declare_id!("BAkgt6Ls3oXxA8mbQoHhV1fwWrrZHjXJsN7qqLxoKYxe");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
