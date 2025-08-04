#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod constants;
pub mod error;
pub mod handlers;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use handlers::*;
pub use state::*;

declare_id!("Bvw5aYMCJDM1136hC5GLqmtq1LbsqSKEgC4owCQj9ZYm");

#[program]
pub mod capstone {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        id: u64,
        number_of_days: u8,
        daily_frequency: u8,
        duration_minutes: u8,
        commitment_stake: u64,
    ) -> Result<()> {
        ctx.accounts.initialize(
            id,
            number_of_days,
            daily_frequency,
            duration_minutes,
            commitment_stake,
            &ctx.bumps,
        )
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_helpers;
