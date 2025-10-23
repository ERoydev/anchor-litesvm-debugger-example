use anchor_lang::prelude::*;

declare_id!("6CSmiViMaAguKgxNVwU8TWMPViQbtL5KKoFrDwWwtYNR");

#[program]
pub mod program_b {
    use super::*;

    pub fn mocked(ctx: Context<AnotherStruct>) -> Result<()> {
        msg!("Hello from program B: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn initialize(ctx: Context<SomeRandomStruct>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AnotherStruct<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SomeRandomStruct {}
