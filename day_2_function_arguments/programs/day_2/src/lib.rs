use anchor_lang::prelude::*;

declare_id!("9qpFStb7Ug6di2i1eoof2vrqLFWiuLzC8M6BK4xr22Ww");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("a: {}, b: {}", a, b);
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
