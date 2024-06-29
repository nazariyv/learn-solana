use anchor_lang::prelude::*;

declare_id!("CRY5Vcb5zLCk4ej8eTu7aCHWpvPT7SDSJyVSrMQyWrY2");

#[program]
pub mod day_3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
