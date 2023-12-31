use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserState {
    // to avoid reinitialization attack
    pub is_initialized: u8,
    // user
    pub user: Pubkey,
    // last hatch time
    pub last_hatch_time: u64,
    pub claimed_roogs: u64,
    pub miners: u64,
    pub referral: Pubkey,
    pub referral_set: u8,
}
