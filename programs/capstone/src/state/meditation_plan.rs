use anchor_lang::prelude::*;

use crate::MeditationAttestation;

#[account]
#[derive(InitSpace)]
pub struct MeditationPlan {
    #[max_len(120)] // 4 times a day * max of 30 days
    pub attestations: Vec<MeditationAttestation>,
    pub bump: u8,
    pub commitment_stake: u64,
    pub daily_frequency: u8,
    pub duration_minutes: u8,
    pub end_at: i64,
    pub id: u64,
    pub is_active: bool,
    pub is_completed: bool,
    pub number_of_days: u8,
    pub owner: Pubkey,
    pub penalties: u64,
    pub rewards: u64,
    pub start_at: i64,
}
