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

    pub fn add_donations(
        ctx: Context<Donate>,
        name : String,
        id : u8,
        created_at : String,
        no_of_donators: u8,
    ) -> Result<()> {
        let donation_account = &mut ctx.accounts.donation_account;
        let user_profile = &mut ctx.accounts.user_profile;
        donation_account.id = user_profile.donation_requests;
        user_profile.donation_requests = user_profile.donation_requests.checked_add(1).unwrap();
        donation_account.authority = ctx.accounts.authority.key();
        donation_account.name = name;
        donation_account.name = user_profile.name.clone();
        donation_account.created_at = created_at;
        donation_account.donators = Vec::with_capacity(no_of_donators.try_into().unwrap());
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


#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)] 

    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [ &[user_profile.donation_requests as u8].as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<Donation>(),
    )]
    pub donation_account: Box<Account<'info, Donation>>,

    pub system_program: Program<'info, System>,
}

