use anchor_lang::prelude::*;

#[account]
pub struct DonationRecord {
    pub donor: Pubkey,
    pub campaign: Pubkey,
    pub amount: u64,
    pub donation_number: u32,
    pub bump: u8,
}

impl DonationRecord {
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 4 + 1;
}