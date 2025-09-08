use anchor_lang::prelude::*;
use crate::{state::*, error::*};

#[derive(Accounts)]
#[instruction(title: String, description: String, image_url: String, goal_amount: u64)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator, 
        space = Campaign::SIZE, 
        seeds = [
            b"campaign", 
            creator.key().as_ref(),
            title.as_bytes(),
            description.as_bytes(),
            image_url.as_bytes(),
            goal_amount.to_le_bytes().as_ref(),
        ], 
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    pub system_program: Program<'info, System>,
}

impl CreateCampaign<'_> {
    pub fn handler(
        ctx: Context<CreateCampaign>,
        title: String,
        description: String,
        image_url: String,
        goal_amount: u64,
    ) -> Result<()> {
        require!(title.len() <= Campaign::MAX_TITLE_LENGTH, FundraiserError::TitleTooLong);
        require!(description.len() <= Campaign::MAX_DESCRIPTION_LENGTH, FundraiserError::DescriptionTooLong);
        require!(image_url.len() <= Campaign::MAX_IMAGE_URL_LENGTH, FundraiserError::ImageUrlTooLong);
        require!(goal_amount > 0, FundraiserError::InvalidGoalAmount);

        let campaign = &mut ctx.accounts.campaign;
        campaign.creator = ctx.accounts.creator.key();
        campaign.title = title;
        campaign.description = description;
        campaign.image_url = image_url;
        campaign.goal_amount = goal_amount;
        campaign.current_amount = 0;
        campaign.total_donors = 0;
        campaign.total_donations = 0;
        campaign.total_withdrawals = 0;
        campaign.total_withdrawn = 0;
        campaign.is_active = true;
        campaign.bump = ctx.bumps.campaign;

        msg!("Fundraiser campaign created successfully: {}", campaign.title);
        msg!("Goal: {} lamports", campaign.goal_amount);
        msg!("Campaign created by: {}", campaign.creator);
        Ok(())
    }
}