use anchor_lang::prelude::*;

#[error_code]
pub enum MeditationPlanError {
    #[msg("Commitment stake must be between 10 and 500 USDC")]
    InvalidCommitmentStakeAmount,
    #[msg("Daily frequency must be between 1 and 4")]
    InvalidDailyFrequency,
    #[msg("Duration minutes must be between 5 and 60")]
    InvalidDurationMinutes,
    #[msg("Only USDC mint is allowed")]
    InvalidMint,
    #[msg("Number of days must be between 7 and 30")]
    InvalidNumberOfDays,
}
