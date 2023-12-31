use crate::{constants::*, errors::*, states::*, utils::*};
use anchor_lang::prelude::*;

use std::mem::size_of;
#[derive(Accounts)]
pub struct HatchRoogs<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        has_one = user,
    )]
    pub user_state: Account<'info, UserState>,

    /// CHECK:
    #[account(
        constraint = referral.key() != user.key()
    )]
    pub referral: AccountInfo<'info>,

    #[account(
        init_if_needed,
        seeds = [USER_STATE_SEED, referral.key().as_ref()],
        bump,
        payer = user,
        space = 8 + size_of::<UserState>()
    )]
    pub referral_state: Account<'info, UserState>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn hatch_roogs_handle(ctx: Context<HatchRoogs>) -> Result<()> {
    let cur_timestamp = Clock::get()?.unix_timestamp as u64;
    if ctx.accounts.referral_state.is_initialized == 0 {
        ctx.accounts.referral_state.is_initialized = 1;
        ctx.accounts.referral_state.last_hatch_time = cur_timestamp as u64;
        ctx.accounts.referral_state.user = ctx.accounts.referral.key();
    } else {
        require!(
            ctx.accounts.referral_state.user.eq(&ctx.accounts.referral.key()),
            RoogError::IncorrectUserState
        );
    }
    msg!(
        "hatch ctx.accounts.user_state.claimed_roogs : {}",
        ctx.accounts.user_state.claimed_roogs
    );
    let roogs_used = ctx.accounts
        .user_state
        .claimed_roogs
        .checked_add(get_roogs_since_last_hatch(
            &ctx.accounts.user_state,
            cur_timestamp,
            ctx.accounts.global_state.roogs_per_miner,
        )?)
        .unwrap();

    msg!("hatch roogs_used: {}", roogs_used);
    msg!(
        "hatch ctx.accounts.global_state.roogs_per_miner: {}",
        ctx.accounts.global_state.roogs_per_miner
    );
    let new_miners = roogs_used
        .checked_div(ctx.accounts.global_state.roogs_per_miner)
        .unwrap();
    msg!("hatch new_miners: {}", new_miners);
    ctx.accounts.user_state.miners = ctx.accounts.user_state.miners.checked_add(new_miners).unwrap();
    ctx.accounts.user_state.claimed_roogs = 0;
    ctx.accounts.user_state.last_hatch_time = cur_timestamp;
    msg!("user_state.miners = {}", ctx.accounts.user_state.miners);
    if ctx.accounts.referral.key().eq(&ctx.accounts.user.key()) {
        ctx.accounts.user_state.referral_set = 0;
    } else {
        if ctx.accounts.user_state.referral_set == 0 {
            ctx.accounts.user_state.referral_set = 1;
            ctx.accounts.user_state.referral = ctx.accounts.referral.key();
        }
    }

    if ctx.accounts.user_state.referral_set == 1 {
        require!(
            ctx.accounts.user_state.referral.eq(&ctx.accounts.referral.key()),
            RoogError::IncorrectReferral
        );
        ctx.accounts.referral_state.claimed_roogs = ctx.accounts
            .referral_state
            .claimed_roogs
            .checked_add(roogs_used / 8)
            .unwrap();
    }

    ctx.accounts.global_state.market_roogs = ctx.accounts
        .global_state
        .market_roogs
        .checked_add(roogs_used / 5)
        .unwrap();

    msg!("last user_state.miners = {}", ctx.accounts.user_state.miners);
    
    Ok(())
}
