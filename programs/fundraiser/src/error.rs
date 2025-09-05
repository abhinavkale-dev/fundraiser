use anchor_lang::prelude::*;

#[error_code]
pub enum FundraiserError {
    #[msg("Campaign title too long (max 50 characters)")]
    TitleTooLong,
    #[msg("Campaign description too long (max 200 characters)")]
    DescriptionTooLong,
    #[msg("Target amount must be greater than 0")]
    InvalidTargetAmount,
    #[msg("Campaign duration must be beteen 1 and 365 days")]
    InvalidDuration,
    #[msg("Campaign has ended")]
    CampaignEnded,
    #[msg("Campaign is paused")]
    CampaignPaused,
    #[msg("Only campaign creator can perform this action")]
    UnauthorizedAccess,
    #[msg("Target amount not reached yet")]
    TargetNotReached,
    #[msg("Insufficient funds in campaign")]
    InsufficientFunds,
    #[msg("Cannot delete campaign with funds")]
    CannotDeleteWithFunds,
    #[msg("Campaign is not paused")]
    CannotDeletWithFunds,
    #[msg("Donation amount too small")]
    DonationTooSmall,
}


