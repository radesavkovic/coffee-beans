use crate::{constants::*, errors::*, states::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token, self}, associated_token::AssociatedToken};
use std::mem::size_of;
#[derive(Accounts)]
pub struct BuyRoogs<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut, address = global_state.treasury)]
    pub treasury: Account<'info, TokenAccount>,

    #[account(mut, address = global_state.vault)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + size_of::<UserState>()
    )]
    pub user_state: Account<'info, UserState>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn buy_roogs_handle(ctx: Context<BuyRoogs>, token_amount: u64) -> Result<()> {
    let accts = ctx.accounts;
    require!(accts.mint.key().to_string() == String::from(TOKEN_ADDRESS), RoogError::IncorrectTokenAddress);

    let cur_timestamp = Clock::get()?.unix_timestamp;
    if accts.user_state.is_initialized == 0 {
        accts.user_state.is_initialized = 1;
        accts.user_state.last_hatch_time = cur_timestamp as u64;
        accts.user_state.user = accts.user.key();
    } else {
        require!(
            accts.user_state.user.eq(&accts.user.key()),
            RoogError::IncorrectUserState
        );
    }

    let mut roogs_bought = calculate_roogs_buy(&accts.global_state, token_amount, accts.vault.amount)?;

    let roogs_bought_fee = dev_fee(&accts.global_state, roogs_bought)?;
    roogs_bought = roogs_bought.checked_sub(roogs_bought_fee).unwrap();

    let token_fee = dev_fee(&accts.global_state, token_amount)?;
    let real_token = token_amount.checked_sub(token_fee).unwrap();

    // send token_fee to treasury
    let cpi_ctx = CpiContext::new(
        accts.token_program.to_account_info(),
        token::Transfer {
            from: accts.account.to_account_info(),
            authority: accts.user.to_account_info(),
            to: accts.treasury.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, token_fee)?;

    // add vault <- token_amount - token_fee
    let cpi_ctx = CpiContext::new(
        accts.token_program.to_account_info(),
        token::Transfer {
            from: accts.account.to_account_info(),
            authority: accts.user.to_account_info(),
            to: accts.vault.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, real_token)?;

    accts.user_state.claimed_roogs = accts
        .user_state
        .claimed_roogs
        .checked_add(roogs_bought)
        .unwrap();
    
    Ok(())
}
