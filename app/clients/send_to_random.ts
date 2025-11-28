import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaVault } from "../../target/types/solana_vault";
import { Keypair, LAMPORTS_PER_SOL} from "@solana/web3.js";

// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.solanaVault as Program<SolanaVault>;

// Recipient: random
const recipient = new Keypair();

const transferAmount = 0.01 * LAMPORTS_PER_SOL;

const tx = program.methods
    .sendToRandom(new anchor.BN(transferAmount))
    .accounts({
      signer: anchor.getProvider().wallet.publicKey,
      recipient: recipient.publicKey,
    })
    .rpc();
console.log(`✅ Send success: ${tx}`);
