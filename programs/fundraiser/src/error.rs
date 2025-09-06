use anchor_lang::prelude::*;

#[error_code]
pub enum FundraiserError {
    #[msg("Title is too long (max 100 characters)")]
    TitleTooLong,

    #[msg("Description is too long (max 500 characters)")]
    DescriptionTooLong,

    #[msg("Image URL is too long (max 200 characters)")]
    ImageUrlTooLong,

    #[msg("Goal amount must be greater than 0")]
    InvalidGoalAmount,

    #[msg("Only the campaign creator can do this")]
    NotCampaignCreator,

    #[msg("Donation amount is too small (minimum 0.001 SOL)")]
    DonationTooSmall,

    #[msg("Not enough funds in the campaign")]
    InsufficientFunds,

    #[msg("Cannot delete campaign that has received donations")]
    CannotDeleteWithDonations,

    #[msg("Campaign is inactive")]
    CampaignInactive,

    #[msg("Amount must be greater than 0")]
    InvalidAmount,
}