use anchor_lang::prelude::*;
use crate::{state::*, error::*};

#[derive(Accounts)]
pub struct DeleteCampaign<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"campaign",
            campaign_account.creator.as_ref(),
            campaign_account.title.as_bytes(),
        ],
        bump = campaign_account.bump,
        has_one = creator @ FundraiserError::NotCampaignCreator,
        close = creator,
    )]
    pub campaign_account: Account<'info, Campaign>,
}

impl DeleteCampaign<'_> {
    pub fn handler(
        ctx: Context<DeleteCampaign>,
    ) -> Result<()> {
        let campaign = &ctx.accounts.campaign_account;

        require!(
            campaign.current_amount == 0
            && campaign.total_donations == 0
            && campaign.total_donors == 0,
            FundraiserError::CannotDeleteWithDonations);

        msg!("Campaign deleted: {}", campaign.title);
        Ok(())
    }
}