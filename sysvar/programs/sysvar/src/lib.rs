use anchor_lang::prelude::*;

declare_id!("HGuq6WzaMQ4DaHyK148JonXBHv74Utw26c6itXZK6Tgs");

#[program]
pub mod sysvar {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let block_timestamp = Clock::get()?;
        msg!("Current block timestamp: {:?}", block_timestamp);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
