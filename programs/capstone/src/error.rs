use anchor_lang::prelude::*;

#[error_code]
pub enum MeditationPlanError {
    #[msg("Meditation attestation must be under 8 hours")]
    AttestationTooLong,
    #[msg("Meditation attestation is shorter than planned duration")]
    AttestationTooShort,
    #[msg("Daily sessions are already completed for today")]
    DailyFrequencyExceeded,
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
    #[msg("Attestation timestamps are invalid")]
    InvalidTimestamps,
    #[msg("Meditation plan has already been completed")]
    PlanCompleted,
    #[msg("Meditation plan has expired")]
    PlanExpired,
    #[msg("Meditation plan is inactive")]
    PlanInactive,
    #[msg("Meditation plan has not ended yet")]
    PlanNotEnded,
    #[msg("Meditation plan has not started yet")]
    PlanNotStarted,
    #[msg("Unauthorized access to the meditation plan")]
    UnauthorizedAccess,
}
