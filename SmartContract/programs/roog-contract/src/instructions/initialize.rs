use crate::{constants::*, states::*, errors::*};
use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token}, associated_token::AssociatedToken};
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        space = 8 + size_of::<GlobalState>(),
        payer = authority,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub treasury: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [VAULT_SEED, mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = global_state
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn validate(&self) -> Result<()> {
        if self.global_state.is_initialized == 1 {
            require!(
                self.global_state.authority.eq(&self.authority.key()),
                RoogError::NotAllowedAuthority
            )
        }
        Ok(())
    }
}

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
///
#[access_control(ctx.accounts.validate())]
pub fn initialize_handle(ctx: Context<Initialize>, new_authority: Pubkey) -> Result<()> {
    let accts = ctx.accounts;
    require!(accts.mint.key().to_string() == String::from(TOKEN_ADDRESS), RoogError::IncorrectTokenAddress);
    accts.global_state.is_initialized = 1;
    accts.global_state.authority = new_authority;
    accts.global_state.vault = accts.vault.key();
    accts.global_state.treasury = accts.treasury.key();

    accts.global_state.market_roogs = 108000000000;
    accts.global_state.dev_fee = 300; // means 3%
    accts.global_state.psn = 10000;
    accts.global_state.psnh = 5000;
    accts.global_state.roogs_per_miner = 1080000;

    Ok(())
}
