import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';
  import fs from 'mz/fs';
  import path from 'path';
  
  
  /*
    Our keypair we used to create the on-chain Rust program
  */
  const PROGRAM_KEYPAIR_PATH = path.join(
    path.resolve(__dirname, '../../dist/program'),
    'hello_solana-keypair.json'
  );
  
  
  async function main() {
    
    console.log("Launching client...");
  
    /*
    Connect to local cluster
    */
    let connection = new Connection('http://127.0.0.1:8899', 'confirmed');
  
    /*
    Get our program's public key
    */
    
    let programId = new PublicKey('{insertProgramIdHere}');
    /*
    Generate an account (keypair) to transact with our program
    */
    const triggerKeypair = Keypair.generate();
    const airdropRequest = await connection.requestAirdrop(
      triggerKeypair.publicKey,
      LAMPORTS_PER_SOL,
    );
    await connection.confirmTransaction(airdropRequest);
  
    /*
    Conduct a transaction with our program
    */
    
    console.log('--Pinging Program ', programId.toBase58());
    const instruction = new TransactionInstruction({
      keys: [{pubkey: triggerKeypair.publicKey, isSigner: false, isWritable: true}],
      programId,
      data: Buffer.alloc(0),
    });
    await sendAndConfirmTransaction(
      connection,
      new Transaction().add(instruction),
      [triggerKeypair],
    );
  }
  
  
  main().then(
    () => process.exit(),
    err => {
      console.error(err);
      process.exit(-1);
    },
  );