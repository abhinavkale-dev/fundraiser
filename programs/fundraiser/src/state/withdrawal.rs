use anchor_lang::prelude::*;

#[account]
pub struct WithdrawalRecord {
    pub campaign: Pubkey,
    pub amount: u64,
    pub withdrawal_number: u32,
    pub withdrawal_type: WithdrawalType,
    pub bump: u8,
}

impl WithdrawalRecord {
    pub const SIZE: usize = 8 + 32 + 8 + 4 + 1 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum WithdrawalType {
    Full,
    Partial,
}