use anchor_lang::prelude::*;

declare_id!("CRY5Vcb5zLCk4ej8eTu7aCHWpvPT7SDSJyVSrMQyWrY2");

#[program]
pub mod day_3 {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn non_empty_account_example(_ctx: Context<NonEmptyAccount>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct NonEmptyAccount<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>
}