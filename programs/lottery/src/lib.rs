use anchor_lang::prelude::*;

declare_id!("BqfFYHEKmpDWLWM5vsw9rr2oJp2WTVhDWUrBBWhtEZvJ");

#[program]
pub mod lottery {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
