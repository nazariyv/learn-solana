use anchor_lang::prelude::*;

declare_id!("EmRmNn6Zsjq5SVzS1T1Zo5ykMNnzgFPywPTfKFXrkLPD");

#[program]
pub mod day_4_require_revert_custom_errors {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
