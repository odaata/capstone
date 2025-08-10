use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::constants::USDC_MINT;
use crate::error::MeditationPlanError;
use crate::state::MeditationPlan;

#[derive(Accounts)]
pub struct Complete<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

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
        has_one = owner @ MeditationPlanError::UnauthorizedAccess,
    )]
    pub meditation_plan: Account<'info, MeditationPlan>,

    #[account(
        mint::token_program = token_program,
        constraint = mint.key() == USDC_MINT @ MeditationPlanError::InvalidMint
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = meditation_plan,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Complete<'info> {
    pub fn complete(&mut self) -> Result<()> {
        self.meditation_plan.complete()?;
        self.transfer_rewards()
    }

    fn transfer_rewards(&mut self) -> Result<()> {
        let rewards = self.meditation_plan.rewards.min(self.vault.amount);
        if rewards < 1 {
            return Ok(());
        }

        let owner_key = self.meditation_plan.owner.key();
        let id_bytes = self.meditation_plan.id.to_le_bytes();
        let seeds = &[
            b"meditation_plan",
            owner_key.as_ref(),
            id_bytes.as_ref(),
            &[self.meditation_plan.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.owner_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.meditation_plan.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        transfer_checked(cpi_ctx, rewards, self.mint.decimals)
    }
}
