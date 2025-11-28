import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaVault } from "../../target/types/solana_vault";

/* CALL ONLY ONCE */

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.solanaVault as Program<SolanaVault>;

// init vault
const tx = program.methods
  .initializeVault()
  .accounts({
    signer: anchor.getProvider().wallet.publicKey,
  })
  .rpc();
console.log(`✅ Init success: ${tx}`);
