use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct MeditationAttestation {
    pub attester: Pubkey,
    pub started_at: i64,
    pub ended_at: i64,
}
