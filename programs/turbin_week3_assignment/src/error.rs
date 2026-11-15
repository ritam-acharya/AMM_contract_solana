use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Pool is Lock")]
    PoolLocked,
    #[msg("Slippage Exceeded")]
    SlippageExceeded,
}
