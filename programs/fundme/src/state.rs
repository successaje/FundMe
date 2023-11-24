use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct TokenState {
   pub bump: u8,
   pub amount: u64,           
}