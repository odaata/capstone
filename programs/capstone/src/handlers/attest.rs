use anchor_lang::prelude::*;

use crate::error::MeditationPlanError;
use crate::state::MeditationPlan;
use crate::MeditationAttestation;

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
        self.validate_input(started_at, ended_at)?;

        self.meditation_plan
            .attestations
            .push(MeditationAttestation {
                attester: self.attester.key(),
                started_at,
                ended_at,
            });
        Ok(())
    }

    fn validate_input(&self, started_at: i64, ended_at: i64) -> Result<()> {
        // Ensure attester is the owner
        require!(
            self.meditation_plan.owner == self.attester.key(),
            MeditationPlanError::UnauthorizedAccess
        );

        let duration = ended_at - started_at;
        require_gte!(
            duration,
            self.meditation_plan.duration_minutes as i64 * 60,
            MeditationPlanError::AttestationTooShort
        );

        Ok(())
    }
}
