use anchor_lang::prelude::*;
use  crate::{state::*, error::*};

const MIN_DONATION: u64 = 1_000_000;

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"campaign",
            campaign.creator.as_ref(),
            campaign.title.as_bytes(),
        ],
        bump = campaign.bump,
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        init,
        payer = donor,
        space = DonationRecord::SIZE,
        seeds = [
            b"donation_record",
            campaign.key().as_ref(),
            &(campaign.total_donations + 1).to_le_bytes(),
        ],
        bump
    )]
    pub donation_record: Account<'info, DonationRecord>,

    #[account(
        init_if_needed,
        payer = donor,
        space = DonorSummary::SIZE,
        seeds = [
            b"donor_summary",
            campaign.key().as_ref(),
        ],
        bump
    )]
    pub donor_summary: Account<'info, DonorSummary>,

    pub system_program: Program<'info, System>,
}

impl Donate<'_> {
    pub fn handler(
        ctx: Context<Donate>,
        amount: u64,
    ) -> Result<()> {
        require!(amount >= MIN_DONATION, FundraiserError::DonationTooSmall);
        require!(ctx.accounts.campaign.is_active, FundraiserError::CampaignInactive);

        let campaign = &mut ctx.accounts.campaign;

        let program = ctx.accounts.system_program.to_account_info();
        let accounts = Transfer {
            from: ctx.accounts.donor.to_account_info(),
            to: ctx.accounts.campaign.to_account_info(),
            authority: ctx.accounts.donor.to_account_info(),
        };

        system_program::transfer(
            CpiContext::new(program, accounts),
            amount,
        )?;

        let next_num = campaign.total_donations + 1;

        let donation_record = &mut ctx.accounts.donation_record;
        donation_record.donor = ctx.accounts.donor.key();
        donation_record.campaign = campaign.key();
        donation_record.amount = amount;
        donation_record.donation_number = next_num;
        donation_record.bump = ctx.bumps.donation_record;

        let donor_summary = &mut ctx.accounts.donor_summary;
        if donor_summary.total_donations == 0 {
            campaign.total_donors += 1;
            donor_summary.donor = ctx.accounts.donor.key();
            donor_summary.campaign = campaign.key();
            donor_summary.bump = ctx.bumps.donor_summary;
        }
        donor_summary.total_donations = donor_summary.total_donations.saturating_add(amount);
        donor_summary.donation_count = donor_summary.donation_count.saturating_add(1);

        campaign.current_amount = campaign.current_amount.saturating_add(amount);
        campaign.total_donations = next_num;

        msg!("Donation received successfully");
        msg!("Donor: {}", ctx.accounts.donor.key());
        msg!("Amount: {} lamports", amount);
        msg!("Camapign total amount: {} lamports", campaign.current_amount);
        msg!("Progress: {}%", campaign.percentage_raised());
        
        Ok(())
    }
}