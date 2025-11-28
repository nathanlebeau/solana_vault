use anchor_lang::prelude::*;

#[account]
pub struct VaultData {
    pub amount: u64,
    pub bump: u8, // bump for PDA
}
