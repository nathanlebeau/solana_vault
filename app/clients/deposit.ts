import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaVault } from "../../target/types/solana_vault";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.solanaVault as Program<SolanaVault>;

// deposit in vault
const tx = program.methods
    .deposit(new anchor.BN(1))
    .accounts({
      signer: anchor.getProvider().wallet.publicKey,
    })
    .rpc();
console.log(`✅ Transaction de deposit réussie: ${tx}`);
