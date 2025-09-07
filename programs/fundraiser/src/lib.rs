pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2qYrfRPXKsk12Qn8uJkSvtfiMnhjFHJZMEmmQ2DDbffS");

#[program]
pub mod fundraiser {
    use super::*;

    pub fn create_campaign(ctx: Context<CreateCampaign>, title: String, description: String, image_url: String, goal_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_all(ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_partial(ctx: Context<WithdrawPartial>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn update_campaign(ctx: Context<UpdateCampaign>, new_title: String, new_description: String, new_image_url: String, new_goal_amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn delete_campaign(ctx: Context<DeleteCampaign>) -> Result<()> {
        Ok(()
    }
}
