use crate::{constants::*, error::*, instructions::*, states::*, utils::*};
use anchor_lang::prelude::*;

use std::mem::size_of;
#[derive(Accounts)]
pub struct HatchEggs<'info> {
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
        address = global_state.vault
    )]
    /// CHECK: this should be set by admin
    pub vault: AccountInfo<'info>,

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

pub fn handle(ctx: Context<HatchEggs>) -> Result<()> {
    let cur_timestamp = Clock::get()?.unix_timestamp as u64;
    if ctx.accounts.referral_state.is_initialized == 0 {
        ctx.accounts.referral_state.is_initialized = 1;
        ctx.accounts.referral_state.last_hatch_time = cur_timestamp as u64;
        ctx.accounts.referral_state.user = ctx.accounts.referral.key();
    } else {
        require!(
            ctx.accounts.referral_state.user.eq(&ctx.accounts.referral.key()),
            BeanError::IncorrectUserState
        );
    }
    msg!(
        "hatch ctx.accounts.user_state.claimed_eggs : {}",
        ctx.accounts.user_state.claimed_eggs
    );
    let eggs_used = ctx.accounts
        .user_state
        .claimed_eggs
        .checked_add(get_eggs_since_last_hatch(
            &ctx.accounts.user_state,
            cur_timestamp,
            ctx.accounts.global_state.eggs_per_miner,
        )?)
        .unwrap();

    msg!("hatch eggs_used: {}", eggs_used);
    msg!(
        "hatch ctx.accounts.global_state.eggs_per_miner: {}",
        ctx.accounts.global_state.eggs_per_miner
    );
    let new_miners = eggs_used
        .checked_div(ctx.accounts.global_state.eggs_per_miner)
        .unwrap();
    msg!("hatch new_miners: {}", new_miners);
    ctx.accounts.user_state.miners = ctx.accounts.user_state.miners.checked_add(new_miners).unwrap();
    ctx.accounts.user_state.claimed_eggs = 0;
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
            BeanError::IncorrectReferral
        );
        ctx.accounts.referral_state.claimed_eggs = ctx.accounts
            .referral_state
            .claimed_eggs
            .checked_add(eggs_used / 8)
            .unwrap();
    }

    ctx.accounts.global_state.market_eggs = ctx.accounts
        .global_state
        .market_eggs
        .checked_add(eggs_used / 5)
        .unwrap();

        
    msg!("last user_state.miners = {}", ctx.accounts.user_state.miners);
    
    //Err(BeanError::NotAllowedAuthority.into())
    Ok(())
}
