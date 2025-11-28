use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::VaultData;
use crate::constants::VAULT_SEED;

// Init context
#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(mut, seeds = [VAULT_SEED], bump)]
    pub vault_pda: Account<'info, VaultData>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Deposit context
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [VAULT_SEED], bump)]
    pub vault_pda: Account<'info, VaultData>,

    #[account(mut)]
    pub signer: Signer<'info>,
    system_program: Program<'info, System>,
}

// Withdraw context
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [VAULT_SEED], bump)]
    pub vault_pda: Account<'info, VaultData>,

    #[account(mut)]
    pub signer: Signer<'info>,
    system_program: Program<'info, System>,
}

// SendToRandom context
#[derive(Accounts)]
pub struct SendToRandom<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    system_program: Program<'info, System>,
    #[account(mut)]
    recipient: SystemAccount<'info>, 
}

pub fn handler_init(ctx: Context<InitializeVault>) -> Result<()> {
    let vault_pda = &mut ctx.accounts.vault_pda;
    vault_pda.amount = 0;
    msg!("Vault Account initialized");
    Ok(())
}

pub fn handler_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let vault_pda = &mut ctx.accounts.vault_pda;
    vault_pda.amount += amount;
    msg!(
        "Deposited in vault, thanks!\n Total amount in the vault:{}",
        vault_pda.amount
    );
    Ok(())
}

pub fn handler_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let vault_pda = &mut ctx.accounts.vault_pda;
    vault_pda.amount -= amount;
    msg!(
        "Withdraw from vault.\n Total amount in the vault:{}",
        vault_pda.amount
    );
    Ok(())
}

pub fn handler_send_to_random(ctx: Context<SendToRandom>, amount: u64) -> Result<()> {
    let from_pubkey = ctx.accounts.signer.to_account_info();
    let to_pubkey = ctx.accounts.recipient.to_account_info();
    let program_id = ctx.accounts.system_program.to_account_info();

    let cpi_context = CpiContext::new(
        program_id,
        Transfer {
            from: from_pubkey,
            to: to_pubkey,
        },
    );
    transfer(cpi_context, amount)?;
    Ok(())
}
