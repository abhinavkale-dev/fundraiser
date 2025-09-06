use anchor_lang::prelude::*;

#[account]
pub struct DonorSummary {
    pub donor: Pubkey,
    pub campaign: Pubkey,
    pub total_donations: u64,
    pub donation_count: u32,
    pub bump: u8,
}

impl DonorSummary {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 4 + 1;
}