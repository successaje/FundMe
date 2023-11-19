use anchor_lang::prelude::*;

pub mod models;
use crate::models::*;

declare_id!("49wKh7f9yc7qtch5upj52EA4dUqKNSVFb2Pm7SMVUbFH");

#[program]
pub mod fundme {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name : String, age : String) -> Result<()> {

        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.authority.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(
        init, 
        seeds = [authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8+ std::mem::size_of::<UserProfile>(),
    )]

    // pub userProfile
}
