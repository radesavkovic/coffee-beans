use anchor_lang::prelude::*;

#[error_code]
pub enum RoogError {
    /// Not allowed authority
    #[msg("Not allowed authority")]
    NotAllowedAuthority,

    /// Should be over minimum amount
    #[msg("Should be over minimum amount")]
    InsufficientAmount,

    /// Incorrect User State
    #[msg("Incorrect User State")]
    IncorrectUserState,

    /// Incorrect Referral Pubkey
    #[msg("Incorrect Referral Pubkey")]
    IncorrectReferral,

    /// Incorrect Token Address
    #[msg("Incorrect Token Address")]
    IncorrectTokenAddress,
}
