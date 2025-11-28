use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod constants;

use instructions::*;

// Program: everybody can deposit and withdraw from this vault. 

declare_id!("5Dk7aNCHuGDYfnnhtggWYyVvQDA6k7iZN2ETw1ia4YRb");

#[program]
pub mod solana_vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::handler_init(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::handler_deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::handler_withdraw(ctx, amount)
    }

    pub fn send_to_random(ctx: Context<SendToRandom>, amount: u64) -> Result<()> {
        instructions::handler_send_to_random(ctx, amount)
    }
}
