use anchor_lang::prelude::*;
use crate::{state::*, error::*};

#[derive(Accounts)]
pub struct UpdateCampaign<'info> {
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
    )]
    pub campaign: Account<'info, Campaign>,
}

impl UpdateCampaign<'_> {
    pub fn handler(
        ctx: Context<UpdateCampaign>,
        new_title: String,
        new_description: String,
        new_image_url: String,
        new_goal_amount: u64,
    ) -> Result<()> {
        require!(new_title.len() <= Campaign::MAX_TITLE_LENGTH, FundraiserError::TitleTooLong);
        require!(new_description.len() <= Campaign::MAX_DESCRIPTION_LENGTH, FundraiserError::DescriptionTooLong);
        require!(new_image_url.len() <= Campaign::MAX_IMAGE_URL_LENGTH, FundraiserError::ImageUrlTooLong);
        require!(new_goal_amount > 0, FundraiserError::InvalidGoalAmount);

        let campaign = &mut ctx.accounts.campaign;
        campaign.title = new_title;
        campaign.description = new_description;
        campaign.image_url = new_image_url;
        campaign.goal_amount = new_goal_amount;

        msg!("Campaign updated successfully");
        msg!("Title: {}", campaign.title);
        msg!("Description: {}", campaign.description);
        msg!("Image URL: {}", campaign.image_url);
        msg!("Goal amount: {} lamports", campaign.goal_amount);
        
        Ok(())
    }
}