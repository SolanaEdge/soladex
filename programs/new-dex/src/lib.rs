use anchor_lang::prelude::*;

declare_id!("9qUrso2c4cKTF6JPNb8V5GHXWHXmw3hinRM61imH8Fr2");

#[program]
pub mod new_dex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
