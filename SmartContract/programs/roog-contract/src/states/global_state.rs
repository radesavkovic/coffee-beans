use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    // to avoid reinitialization attack
    pub is_initialized: u8,
    // admin
    pub authority: Pubkey,
    // vault
    pub vault: Pubkey,
    // treasury
    pub treasury: Pubkey,
    // todo: should be set as 108000000000
    pub market_roogs: u64,

    // these are constants
    pub dev_fee: u64,
    pub psn: u64,
    pub psnh: u64,
    pub roogs_per_miner: u64,
}
