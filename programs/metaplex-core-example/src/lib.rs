use anchor_lang::prelude::*;

declare_id!("Ef14SHa8TGNPKJZ6wShhxwrsWK2sQWH1jiCcjY2AuDuv");

#[program]
pub mod metaplex_core_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
