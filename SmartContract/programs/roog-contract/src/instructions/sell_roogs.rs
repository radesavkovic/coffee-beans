use crate::{constants::*, errors::*, states::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Token, self, Mint}, associated_token::AssociatedToken};
#[derive(Accounts)]
pub struct SellRoogs<'info> {
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
        mut,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        has_one = user
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
}

impl<'info> SellRoogs<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn sell_roogs_handle(ctx: Context<SellRoogs>) -> Result<()> {
    let cur_timestamp = Clock::get()?.unix_timestamp as u64;
    let accts = ctx.accounts;
    
    require!(accts.mint.key().to_string() == String::from(TOKEN_ADDRESS), RoogError::IncorrectTokenAddress);
    msg!("SellRoogs claimed roogs {}", accts.user_state.claimed_roogs);
    let has_roogs = accts
        .user_state
        .claimed_roogs
        .checked_add(get_roogs_since_last_hatch(
            &accts.user_state,
            cur_timestamp,
            accts.global_state.roogs_per_miner,
        )?)
        .unwrap();

    msg!("SellRoogs has_roogs {}", has_roogs);
    let roog_value = calculate_roogs_sell(&accts.global_state, has_roogs, accts.vault.amount)?;

    let fee = dev_fee(&accts.global_state, roog_value)?;
    accts.user_state.claimed_roogs = 0;
    accts.user_state.last_hatch_time = cur_timestamp;
    accts.global_state.market_roogs = accts
        .global_state
        .market_roogs
        .checked_add(has_roogs)
        .unwrap();

    msg!("SellRoogs selling roog_value {}", roog_value);
    msg!("SellRoogs selling fee {}", fee);
    let real_val = roog_value.checked_sub(fee).unwrap();

    // send fee to treasury
    let bump = ctx.bumps.global_state;
    let seeds = &[GLOBAL_STATE_SEED, &[bump]];
    let signer = [&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        accts.token_program.to_account_info(),
        token::Transfer {
            from: accts.vault.to_account_info(),
            authority: accts.global_state.to_account_info(),
            to: accts.treasury.to_account_info(),
        },
        &signer,
    );
    token::transfer(cpi_ctx, fee)?;
    // send to user's token account : token_amount - fee

    let cpi_ctx = CpiContext::new_with_signer(
        accts.token_program.to_account_info(),
        token::Transfer {
            from: accts.vault.to_account_info(),
            authority: accts.global_state.to_account_info(),
            to: accts.account.to_account_info(),
        },
        &signer,
    );
    token::transfer(cpi_ctx, real_val)?;

    Ok(())
}
