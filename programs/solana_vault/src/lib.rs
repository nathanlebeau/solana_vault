pub mod constants;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

// For CPI:
// use anchor_lang::solana_program::system_program;

// Program: everybody can deposit and withdraw from this vault. 
// Fictive amount for the moment.

declare_id!("5Dk7aNCHuGDYfnnhtggWYyVvQDA6k7iZN2ETw1ia4YRb");

#[program]
pub mod solana_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault_account = &mut ctx.accounts.vault_account;
        vault_account.is_initialized = true;
        vault_account.amount = 0;
        msg!("Vault Account initialized");
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault_account = &mut ctx.accounts.vault_account;
        vault_account.amount += amount;
        msg!(
            "Deposited in vault, thanks!\n Total amount in the vault:{}",
            vault_account.amount
        );
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let vault_account = &mut ctx.accounts.vault_account;
        vault_account.amount -= amount;
        msg!(
            "Withdraw from vault.\n Total amount in the vault:{}",
            vault_account.amount
        );
        Ok(())
    }
}

// TODO: move that

// Deposit context
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault_account: Account<'info, VaultData>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

// Withdraw context
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault_account: Account<'info, VaultData>,

    #[account(mut)]
    pub signer: Signer<'info>,
}
