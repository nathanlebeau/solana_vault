import {
  Connection,
  PublicKey,
} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, Idl } from "@coral-xyz/anchor";
import { SolanaVault } from "../../target/types/solana_vault";

// -------------------------------
// Program constants
// -------------------------------
const PROGRAM_ID = new PublicKey("5Dk7aNCHuGDYfnnhtggWYyVvQDA6k7iZN2ETw1ia4YRb");
// const VAULT_PDA = new PublicKey("REPLACE_WITH_VAULT_PDA");
// const IDL: Idl = /* ton IDL Anchor */ {} as any;
// --------------------------------

const status = document.getElementById("status")!;
function log(msg: string) {
  status.textContent = msg;
}

let wallet: any = null;
let provider: AnchorProvider;
const program = anchor.workspace.solanaVault as Program<SolanaVault>;

// -------------------------------
// Wallet connection
// -------------------------------
document.getElementById("connect")!.onclick = async () => {
  try {
    wallet = (window as any).phantom?.solana;
    if (!wallet) throw new Error("Phantom Wallet not found");

    const resp = await wallet.connect();
    const pubkey = resp.publicKey;

    const connection = new Connection("https://api.apr.dev", "confirmed");
    provider = new AnchorProvider(connection, wallet, AnchorProvider.defaultOptions());

    log("Wallet connected: " + pubkey.toBase58());
  } catch (err: any) {
    log("Error: " + err.message);
  }
};

// -------------------------------
// Deposit
// -------------------------------
document.getElementById("deposit")!.onclick = async () => {
  try {
    const lamports = Number((document.getElementById("amount") as HTMLInputElement).value);

    log("Preparing deposit...");

    const tx = await program.methods
      .deposit(new anchor.BN(lamports)) 
      .accounts({
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();

    log("Deposit sent: " + tx);
  } catch (err: any) {
    log("Error deposit: " + err.message);
  }
};

// -------------------------------
// Withdraw
// -------------------------------
document.getElementById("withdraw")!.onclick = async () => {
  try {
    const lamports = Number((document.getElementById("amount") as HTMLInputElement).value);

    log("Preparing withdraw...");

    const tx = await program.methods
      .withdraw(new anchor.BN(lamports)) 
      .accounts({
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();

    log("Withdraw sent: " + tx);
  } catch (err: any) {
    log("Error withdraw : " + err.message);
  }
};
