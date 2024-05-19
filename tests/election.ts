import * as anchor from "@coral-xyz/anchor";
import { assert } from "chai";
import { Program } from "@coral-xyz/anchor";
import { SystemProgram } from "@solana/web3.js";
import { Keypair } from "@solana/web3.js";

//import { Election } from "../target/types/election";

describe("election", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.local();
  
  anchor.setProvider(provider);

  const program = anchor.workspace.Election;

  it("Is initialized!", async () => {
    // Add your test here.
    const newElectionAccount = Keypair.generate();
    const tx = await program.methods.initialize(new anchor.BN("apc")).accounts({
      election: newElectionAccount.publicKey,
      payer: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    })
    .signers(newElectionAccount)
    .rpc(); 

    console.log("Your transaction signature", tx);
  })

})

