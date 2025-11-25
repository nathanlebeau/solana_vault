use anchor_lang::prelude::*;
use std::mem::size_of;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = payer, 
        space = 8 + size_of::<VaultData>(),
        seeds = [b"vault"], 
        bump
    )]
    pub vault_account: Account<'info, VaultData>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct VaultData {
    pub is_initialized: bool,
    pub amount: u64,
    pub bump: u8,
}

impl VaultData {
    // is_initialized (1 byte) + amount (8 bytes) + bump (1 byte)
    pub const LEN: usize = 1 + 8 + 1;
}
