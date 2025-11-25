import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaVault } from "../../target/types/solana_vault";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.solanaVault as Program<SolanaVault>;

// withdraw from vault
const tx = program.methods
    .withdraw(new anchor.BN(20))
    .accounts({
      signer: anchor.getProvider().wallet.publicKey,
    })
    .rpc();
console.log(`✅ Transaction de withdraw réussie: ${tx}`);
