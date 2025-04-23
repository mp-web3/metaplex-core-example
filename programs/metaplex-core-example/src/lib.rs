use anchor_lang::prelude::*;

declare_id!("9Tc3PoSyNr8JJ2Q47AyovJiihSwXQM2eKTAUf4szziMu");

mod instructions;
use instructions::*;

#[program]
pub mod metaplex_core_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn mint_asset(ctx: Context<MintAsset>) -> Result<()> {
        ctx.accounts.mint_core_asset()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
