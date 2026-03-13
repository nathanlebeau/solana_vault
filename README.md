# Solana Vault

Interacting with a simple program built with Anchor on Solana devnet.

## Key Features

- **Anchor framework** used (`anchor init`)
- **Build and deploy:** `anchor build` and `anchor deploy` (define chain in `Anchor.toml`)
- **Launch a test client:**
  ```bash
  ANCHOR_WALLET="path_to_your_solana_key" \
  ANCHOR_PROVIDER_URL="https://api.devnet.solana.com" \
  npx ts-node app/clients/deposit.ts
  ```
- **Features:** deposit / withdraw from vault (PDA address for vault), send SOL to random key (CPI)
- Web interface incoming
