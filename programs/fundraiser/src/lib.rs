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

    pub fn create_campaign(ctx: Context<CreateCampaign>) -> Result<()> {
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_partial(ctx: Context<WithdrawPartial>) -> Result<()> {
        Ok(())
    }

    pub fn update_campaign(ctx: Context<UpdateCampaign>) -> Result<()> {
        Ok(())
    }

    pub fn delete_campaign(ctx: Context<DeleteCampaign>) -> Result<()> {
        Ok(())
    }

    pub fn pause_campaign(ctx: Context<PauseCampaign>) -> Result<()> {
        Ok(())
    }

    pub fn resume_campaign(ctx: Context<ResumeCampaign>) -> Result<()> {
        Ok(())
    }
}
