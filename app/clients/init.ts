import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaVault } from "../../target/types/solana_vault";

/* CALL ONLY ONCE */

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.solanaVault as Program<SolanaVault>;

// // Calculate PDA (useless)
// let [vault_acc] = anchor.web3.PublicKey.findProgramAddressSync(
//   [Buffer.from("vault")],
//   program.programId
// );

// init vault
const tx = program.methods
  .initialize()
  .accounts({
    payer: anchor.getProvider().wallet.publicKey,
  })
  .rpc();
console.log(`✅ Transaction d'init réussie: ${tx}`);
