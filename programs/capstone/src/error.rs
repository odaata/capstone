use anchor_lang::prelude::*;

#[error_code]
pub enum MeditationPlanError {
    #[msg("Only USDC mint is allowed")]
    InvalidMint,
    #[msg("Number of days must be between 7 and 30")]
    InvalidNumberOfDays,
}
