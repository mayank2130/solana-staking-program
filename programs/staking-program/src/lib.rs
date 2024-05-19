use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("9QTpiMkX6W5ZYjAHb7oNVvYcbGyqbXcHA7AAt3W4WFRf");

pub mod constants {
    pub const VAULT_SEED: &[u8] = b"vault";
    pub const STAKE_INFO_SEED: &[u8] = b"stake_info";
    pub const TOKEN_SEED: &[u8] = b"token";
}

#[program]
pub mod staking_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn stake(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn destake(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [constants::VAULT_SEED],
        bump,
        payer = signer,
        token::mint = mint,
        token::authority = token_vault_account,

    )]
    pub token_vault_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
