use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::constants::USDC_MINT;
use crate::error::MeditationPlanError;
use crate::state::MeditationPlan;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = MeditationPlan::DISCRIMINATOR.len() + MeditationPlan::INIT_SPACE,
        seeds = [b"meditation_plan", owner.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub meditation_plan: Account<'info, MeditationPlan>,

    #[account(
        mint::token_program = token_program,
        constraint = mint.key() == USDC_MINT @ MeditationPlanError::InvalidMint
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = meditation_plan,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        id: u64,
        number_of_days: u8,
        daily_frequency: u8,
        duration_minutes: u8,
        commitment_stake: u64,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        require_gte!(number_of_days, 7, MeditationPlanError::InvalidNumberOfDays);
        require_gte!(30, number_of_days, MeditationPlanError::InvalidNumberOfDays);

        let start_at = Clock::get()?.unix_timestamp;
        let end_at = start_at + (number_of_days as i64 * 24 * 60 * 60);
        self.meditation_plan.set_inner(MeditationPlan {
            attestations: vec![],
            bump: bumps.meditation_plan,
            commitment_stake,
            daily_frequency,
            duration_minutes,
            end_at,
            id,
            is_active: false,
            is_completed: false,
            number_of_days,
            owner: self.owner.key(),
            penalties: 0,
            rewards: 0,
            start_at,
        });
        self.deposit(commitment_stake)
    }

    fn deposit(&mut self, commitment_stake: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.owner_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_ctx, commitment_stake, self.mint.decimals)
    }
}
