use anchor_lang::prelude::*;

#[account]
pub struct Campaign {
    pub creator: Pubkey,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub goal_amount: u64,
    pub current_amount: u64,
    pub total_donors: u32,
    pub total_donations: u32,
    pub total_withdrawals: u32,
    pub total_withdrawn: u64,
    pub is_active: bool,
    pub bump: u8,
}

impl Campaign {
    pub const MAX_TITLE_LENGTH: usize = 100;
    pub const MAX_DESCRIPTION_LENGTH: usize = 200;
    pub const MAX_IMAGE_URL_LENGTH: usize = 200;

    pub const SIZE: usize = 8 + 32 + 4 + Self::MAX_TITLE_LENGTH + 4 + Self::MAX_DESCRIPTION_LENGTH + 4 + Self::MAX_IMAGE_URL_LENGTH + 8 + 8 + 4 + 4 + 4 + 8 + 1 + 1;

    pub fn percentage_raised(&self) -> u64 {
        if self.goal_amount == 0 {
            return 0;
        }
        (self.current_amount * 100) / self.goal_amount
    }

    pub fn goal_reached(&self) -> bool {
        self.current_amount >= self.goal_amount
    }

}