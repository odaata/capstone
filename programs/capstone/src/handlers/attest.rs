use anchor_lang::prelude::*;

use crate::error::MeditationPlanError;
use crate::state::MeditationPlan;

#[derive(Accounts)]
pub struct Attest<'info> {
    #[account(mut)]
    pub attester: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"meditation_plan",
            meditation_plan.owner.key().as_ref(),
            meditation_plan.id.to_le_bytes().as_ref()
        ],
        bump = meditation_plan.bump,
        constraint = meditation_plan.is_active @ MeditationPlanError::PlanInactive,
        constraint = !meditation_plan.is_completed @ MeditationPlanError::PlanCompleted,
    )]
    pub meditation_plan: Account<'info, MeditationPlan>,

    pub system_program: Program<'info, System>,
}

impl<'info> Attest<'info> {
    pub fn attest(&mut self, started_at: i64, ended_at: i64) -> Result<()> {
        self.meditation_plan
            .save_attestation(self.attester.key(), started_at, ended_at)
    }
}
