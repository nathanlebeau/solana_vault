# solana_vault
Interacting with simple program built with Anchor on Solana devnet

# key Features
- Anchor framework used (anchor init)

- Build and deploy: anchor build and anchor deploy (define chain in Anchor.toml)

- Launch a test client: ANCHOR_WALLET="path_to_your_solana_key" ANCHOR_PROVIDER_URL="https://api.
devnet.solana.com" npx ts-node app/clients/deposit.ts

- features: deposit / withdraw from vault (PDA address for vault), send SOL to random key (CPI)

- web interface incoming
