
/*
DOES NOT WORK...The working version will be mighty similar to this, but need to invest into js a lot more to fully undertand...
*/


import * as anchor from "@project-serum/anchor";
import { Hashvault } from "../target/types/hashvault";



function shortKey(key: anchor.web3.PublicKey) {
  return key.toString().substring(0, 8);
}


describe("hashvault", () => {
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Hashvault as anchor.Program<Hashvault>;

  async function generateKeypair() {
    let keypair = anchor.web3.Keypair.generate();
    await provider.connection.requestAirdrop(
      keypair.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await new Promise( resolve => setTimeout(resolve, 3 * 1000) ); // Sleep 3s
    return keypair;
  }

  async function derivePda(hashvalue: string, seedstring: string, pubkey: anchor.web3.PublicKey) {
    let [pda, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        pubkey.toBuffer(),
        Buffer.from(seedstring),
        Buffer.from(hashvalue),
      ],
      program.programId
    );
    return pda;
  }

  async function createLedgerAccount(
    hashvalue: string,
    seedstring: string,
    pda: anchor.web3.PublicKey, 
    wallet: anchor.web3.Keypair
  ) {
    await program.methods.createLedger(hashvalue, seedstring)
      .accounts({
        ledgerAccount: pda,
        wallet: wallet.publicKey,
      })
      .signers([wallet])
      .rpc();
  }

    console.log("--------------------------------------------------");
    let data;
    let pda = await derivePda(hashvalue, seedstring, wallet.publicKey);

    console.log(`Checking if account ${shortKey(pda)} exists for hashvalue: ${hashvalue}...`);
    try {

      data = await program.account.ledger.fetch(pda);
      console.log("It does.");
    
    } catch (e) {
    
      console.log("It does NOT. Creating...");
      await createLedgerAccount(hashvalue, seedstring, pda, wallet);
      data = await program.account.ledger.fetch(pda);
    };

    console.log("Success.");
    console.log("Data:")
    console.log(`    Hashvalue: ${data.hashvalue}`);

}

)
