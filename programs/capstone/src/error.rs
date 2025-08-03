use anchor_lang::prelude::*;

#[error_code]
pub enum MeditationPlanError {
    #[msg("Only USDC mint is allowed")]
    InvalidMint,
}
