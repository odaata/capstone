use anchor_lang::prelude::*;

use crate::constants::HOUR_IN_SECONDS;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct MeditationAttestation {
    pub attester: Pubkey,
    pub started_at: i64,
    pub ended_at: i64,
}

impl MeditationAttestation {
    pub const MAX_DURATION: i64 = 8 * HOUR_IN_SECONDS; // 8 hours in seconds
}
