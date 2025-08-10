use anchor_lang::prelude::*;

use crate::constants::DAY_IN_SECONDS;
use crate::error::MeditationPlanError;
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

impl MeditationPlan {
    pub fn complete(&mut self) -> Result<()> {
        let total_sessions = self.total_sessions();
        let all_sessions_completed = self.attestations.len() >= total_sessions as usize;
        let now = Clock::get()?.unix_timestamp;
        let is_expired = now > self.end_at;
        // Ensure the plan has ended
        require!(
            all_sessions_completed || is_expired,
            MeditationPlanError::PlanNotEnded
        );

        self.is_completed = true;
        self.is_active = false;

        if all_sessions_completed {
            self.rewards = self.commitment_stake;
            return Ok(());
        }

        // If not all sessions are completed, apply penalties
        let completed_sessions = self.attestations.len() as u64;
        let penalty_per_session = self.reward_per_session(total_sessions);
        let penalties = penalty_per_session
            .checked_mul(total_sessions - completed_sessions)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        self.penalties = penalties;
        self.rewards = self
            .commitment_stake
            .checked_sub(penalties)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }

    pub fn save_attestation(
        &mut self,
        attester: Pubkey,
        started_at: i64,
        ended_at: i64,
    ) -> Result<()> {
        self.validate_attestation(attester, started_at, ended_at)?;

        self.attestations.push(MeditationAttestation {
            attester,
            started_at,
            ended_at,
        });

        let total_sessions = self.total_sessions();
        let reward_per_session = self.reward_per_session(total_sessions);
        self.rewards = self
            .rewards
            .checked_add(reward_per_session)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        Ok(())
    }

    fn reward_per_session(&self, total_sessions: u64) -> u64 {
        self.commitment_stake
            .checked_div(total_sessions)
            .ok_or(ProgramError::ArithmeticOverflow)
            .unwrap()
    }

    fn total_sessions(&self) -> u64 {
        self.number_of_days as u64 * self.daily_frequency as u64
    }

    fn validate_attestation(&self, attester: Pubkey, started_at: i64, ended_at: i64) -> Result<()> {
        // Ensure attester is the owner
        require!(
            self.owner == attester,
            MeditationPlanError::UnauthorizedAccess
        );

        let now = Clock::get()?.unix_timestamp;
        // Ensure timestamps are both in the past
        require!(
            started_at <= now && ended_at <= now,
            MeditationPlanError::InvalidTimestamps
        );

        // // Ensure the session ended after it started
        require_gt!(ended_at, started_at, MeditationPlanError::InvalidTimestamps);

        // Ensure the session started after the plan started
        require_gte!(
            started_at,
            self.start_at,
            MeditationPlanError::PlanNotStarted
        );

        // Ensure the session started before the plan ended
        require_gte!(self.end_at, started_at, MeditationPlanError::PlanExpired);

        let duration = ended_at
            .checked_sub(started_at)
            .ok_or(MeditationPlanError::InvalidTimestamps)?;

        // Ensure the session duration is at least the plan duration
        require_gte!(
            duration,
            self.duration_minutes as i64 * 60,
            MeditationPlanError::AttestationTooShort
        );

        // Ensure the session duration is 8 hours or less
        require_gte!(
            MeditationAttestation::MAX_DURATION,
            duration,
            MeditationPlanError::AttestationTooLong
        );

        // Ensure the daily frequency has not been exceeded
        let start_at = self.start_at;
        let elapsed_seconds = started_at - start_at;
        let day_index = elapsed_seconds / DAY_IN_SECONDS;
        let day_start = start_at + day_index * DAY_IN_SECONDS;
        let day_end = day_start + DAY_IN_SECONDS;
        let sessions_today = self
            .attestations
            .iter()
            .filter(|attestation| {
                attestation.started_at >= day_start && attestation.started_at < day_end
            })
            .count();

        require_gt!(
            self.daily_frequency as usize,
            sessions_today,
            MeditationPlanError::DailyFrequencyExceeded
        );

        Ok(())
    }
}
