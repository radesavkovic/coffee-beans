use anchor_lang::prelude::*;

#[error_code]
pub enum BeanError {
    #[msg("Not allowed authority")]
    NotAllowedAuthority,

    #[msg("Should be over minimum amount")]
    InsufficientAmount,

    #[msg("Incorrect User State")]
    IncorrectUserState,

    #[msg("Incorrect Referral Pubkey")]
    IncorrectReferral,
}
