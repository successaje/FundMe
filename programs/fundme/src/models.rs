use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub authority : Pubkey,
    pub age : u8,
    pub created_at : String,
    pub name : String,
    pub donation_requests : u8,
}

#[account]
pub struct Donation {
    pub authority : Pubkey,
    pub name : String,
    pub id : u8,
    pub created_at : String,
    pub donators : Vec<Pubkey>,
}