use anchor_lang::prelude::*;

#[error_code]
pub enum MeditationPlanError {
    #[msg("Meditation attestation is shorter than planned duration")]
    AttestationTooShort,
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
    #[msg("Meditation plan has already been completed")]
    PlanCompleted,
    #[msg("Meditation plan is inactive")]
    PlanInactive,
    #[msg("Unauthorized access to the meditation plan")]
    UnauthorizedAccess,
}
