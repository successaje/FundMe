use anchor_lang::prelude::*;

pub mod models;
use crate::models::*;

declare_id!("49wKh7f9yc7qtch5upj52EA4dUqKNSVFb2Pm7SMVUbFH");

#[program]
pub mod fundme {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name : String, created_at : String, age : u8) -> Result<()> {

        let profile = &mut ctx.accounts.profile;
        profile.authority = ctx.accounts.authority.key();
        profile.age = age;
        profile.created_at = created_at;
        profile.name = name;
        profile.donation_requests = 0;

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

    pub profile : Box<Account<'info, UserProfile>>,

    pub system_program : Program<'info, System>,
}
