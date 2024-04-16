import {
    Connection,
    Transaction,
    SystemProgram,
    sendAndConfirmTransaction,
    PublicKey,
    TransactionInstruction,
  } from "@solana/web3.js";
  import "dotenv/config"
  import * as borsh from "@project-serum/borsh"
  import { LAMPORTS_PER_SOL} from "@solana/web3.js";
  import "dotenv/config"
  import { getKeypairFromEnvironment } from "@solana-developers/helpers";
  
  console.log(
      `Starting interactions client for local testing...`
    );
    const borshInstructionSchema = borsh.struct([
      borsh.u8("variant"),
      borsh.str("title"),
      borsh.u8("rating"),
      borsh.str("description"),
    ])
  
    class Movie {
      title: string
      rating: number
      description: string
      reviewer: PublicKey
    
      constructor(
        title: string,
        rating: number,
        description: string,
        reviewer: PublicKey
      ) {
        this.title = title
        this.rating = rating
        this.description = description
        this.reviewer = reviewer
      }
    }
    
  
  const publicKey = new PublicKey("8t4V9aYB27oeu5VNBirjAUohg1M3GZgcZ5NcQLti5Z2i");
  async function main() 
    
  {
    const programId = new PublicKey("7XHq7jXjKHhtcasiH5s6jKGa5Qkd6pLrH5ii1861zf1p")
  
  const connection = new Connection("http://127.0.0.1:8899", "confirmed");
  
  console.log(
    `💰 Connected to localhost!`
  );
  
  const keypair = getKeypairFromEnvironment("SECRET_KEY");
  
  console.log(
      `✅ Finished! We've loaded our secret key securely, using an env file!`
    );
  
    const transaction = new Transaction()
  
          
  
          const mocks: Movie[] = [
            new Movie(
              "The Shawshank Redemption",
              5,
              `For a movie shot entirely in prison where there is no hope at all, shawshank redemption's main massage and purpose is to remind us of hope, that even in the darkest places hope exists, and only needs someone to find it. Combine this message with a brilliant screenplay, lovely characters and Martin freeman, and you get a movie that can teach you a lesson everytime you watch it. An all time Classic!!!`,
              new PublicKey("7ySRnnNLcYho8ks4gU6KwJvcGtVCZLnB5eTvkkw3vTn5")
            ),
            new Movie(
              "The Godfather",
              5,
              `One of Hollywood's greatest critical and commercial successes, The Godfather gets everything right; not only did the movie transcend expectations, it established new benchmarks for American cinema.`,
              new PublicKey("EurMFhvwKScjv469XQoUm1Qj6PFJQoVwXYmdgeXCqg5m")
            ),
            new Movie(
              "The Godfather: Part II",
              4,
              `The Godfather: Part II is a continuation of the saga of the late Italian-American crime boss, Francis Ford Coppola, and his son, Vito Corleone. The story follows the continuing saga of the Corleone family as they attempt to successfully start a new life for themselves after years of crime and corruption.`,
              new PublicKey("EurMFhvwKScjv469XQoUm1Qj6PFJQoVwXYmdgeXCqg5m")
            ),
            new Movie(
              "The Dark Knight",
              5,
              `The Dark Knight is a 2008 superhero film directed, produced, and co-written by Christopher Nolan. Batman, in his darkest hour, faces his greatest challenge yet: he must become the symbol of the opposite of the Batmanian order, the League of Shadows.`,
              new PublicKey("EurMFhvwKScjv469XQoUm1Qj6PFJQoVwXYmdgeXCqg5m")
            ),
          ]
          const [pda] = await PublicKey.findProgramAddress(
            [mocks[0].reviewer.toBuffer(), Buffer.from("The Shawshank Redemption")], // new TextEncoder().encode(movie.title)],
            programId
        )
  
        const [pdaCounter] = await PublicKey.findProgramAddress(
            [pda.toBuffer(), Buffer.from("comment")], // new TextEncoder().encode(movie.title)],
            programId
        )
  
  
          
            const bufferRow = Buffer.alloc(800)
            borshInstructionSchema.encode(
            mocks[0],
              bufferRow
            )
            const buffer = bufferRow.slice(0, borshInstructionSchema.getSpan(bufferRow))
          
  
    const instruction = new TransactionInstruction({
      keys: [
          {
              pubkey: mocks[0].reviewer,
              isSigner: true,
              isWritable: false,
          },
          {
              pubkey: pda,
              isSigner: false,
              isWritable: true,
          },
          {
              pubkey: pdaCounter,
              isSigner: false,
              isWritable: true,
          },
          {
              pubkey: SystemProgram.programId,
              isSigner: false,
              isWritable: false,
          },
      ],
      data: buffer,
      programId: new PublicKey(programId),
  })
  
  transaction.add(instruction);
  
  const signature = await sendAndConfirmTransaction(connection, transaction, [
    keypair,
  ]);
  
  console.log(
    `💸 Finished! Stuff to the address ${programId}. `
  );
  console.log(`Transaction signature is ${signature}!`);
  }
  main()