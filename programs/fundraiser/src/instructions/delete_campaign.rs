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
            campaign.creator.as_ref(),
            campaign.title.as_bytes(),
        ],
        bump = campaign.bump,
        has_one = creator @ FundraiserError::NotCampaignCreator,
        close = creator,
    )]
    pub campaign: Account<'info, Campaign>,
}

impl DeleteCampaign<'_> {
    pub fn handler(
        ctx: Context<DeleteCampaign>,
    ) -> Result<()> {
        require!(
            campaign.current_amount == 0 
            && campaign.total_donations == 0
            && campaign.total_donors == 0, 
            FundraiserError::CannotDeleteWithDonations);
        
        msg!("Campaign deleted: {}", campaign.title);
        Ok(())
    }
}