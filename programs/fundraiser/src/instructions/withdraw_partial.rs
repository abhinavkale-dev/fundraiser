use anchor_lang::prelude::*;
use crate::{state::*, error::*};

#[derive(Accounts)]
pub struct WithdrawPartial<'info> {
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

    #[account(
        init,
        payer = creator,
        space = WithdrawalRecord::SIZE,
        seeds = [
            b"withdrawal_record",
            campaign.key().as_ref(),
            &(campaign.total_withdrawals + 1).to_le_bytes(),
        ],
        bump
    )]
    pub withdrawal_record: Account<'info, WithdrawalRecord>,

    pub system_program: Program<'info, System>,
}

