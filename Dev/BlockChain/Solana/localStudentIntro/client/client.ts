import * as web3 from "@solana/web3.js";
import "dotenv/config"
import { getKeypairFromEnvironment, airdropIfRequired } from "@solana-developers/helpers";
import * as borsh from "@project-serum/borsh"
import { FC } from 'react'
//import { StudentIntro } from '../models/StudentIntro'
import { useState } from 'react'
import { Box, Button, FormControl, FormLabel, Input, NumberDecrementStepper, NumberIncrementStepper, NumberInput, NumberInputField, NumberInputStepper, Textarea } from '@chakra-ui/react'


/*
1. Get key for the account I want to add data from ✅
2. Connect to local blockchain ✅
3. Define tartget program id ✅
4. Define the struct I wanna send
5. Serialize that shit
6. Send that shit
7. Await for response and deserialize
8. Print everything or verify somehow

*/
const payer = getKeypairFromEnvironment('SECRET_KEY')
const connection = new web3.Connection("http://127.0.0.1:8899")

const PROGRAM_ADDRESS = new web3.PublicKey('{deployedProgramAddress}')
const transaction = new web3.Transaction()
const programId = new web3.PublicKey(PROGRAM_ADDRESS)

export class StudentIntro {
    name: string;
    message: string;

    constructor(name: string, message: string) {
        this.name = name;
        this.message = message;
    }

    static mocks: StudentIntro[] = [
        new StudentIntro('Elizabeth Holmes', `Learning Solana so I can use it to build sick NFT projects.`),
        new StudentIntro('Jack Nicholson', `I want to overhaul the world's financial system. Lower friction payments/transfer, lower fees, faster payouts, better collateralization for loans, etc.`),
        new StudentIntro('Terminator', `i'm basically here to protect`),
    ]

    borshInstructionSchema = borsh.struct([
        borsh.u8('variant'),
        borsh.str('name'),
        borsh.str('message'),
    ])

    static borshAccountSchema = borsh.struct([
        borsh.bool('initialized'),
        borsh.str('name'),
        borsh.str('message'),
    ])

    serialize(): Buffer {
        const buffer = Buffer.alloc(1000)
        this.borshInstructionSchema.encode({ ...this, variant: 0 }, buffer)
        return buffer.slice(0, this.borshInstructionSchema.getSpan(buffer))
    }

    static deserialize(buffer?: Buffer): StudentIntro|null {
        if (!buffer) {
            return null
        }

        try {
            const { name, message } = this.borshAccountSchema.decode(buffer)
            return new StudentIntro(name, message)
        } catch(e) {
            console.log('Deserialization error:', e)
            return null
        }
    }
}


const [name, setName] = useState('')
const [message, setMessage] = useState('')

const publicKey = new web3.PublicKey("8t4V9aYB27oeu5VNBirjAUohg1M3GZgcZ5NcQLti5Z2i")

    const handleSubmit = (event: any) => {
        event.preventDefault()
        const studentIntro = new StudentIntro(name, message)
        handleTransactionSubmit(studentIntro)
    }

    const handleTransactionSubmit = async (studentIntro: StudentIntro) => {
        if (!publicKey) {
            alert('Please connect your wallet!')
            return
        }

        const buffer = studentIntro.serialize()
        const transaction = new web3.Transaction()

        const [pda] = await web3.PublicKey.findProgramAddress(
            [publicKey.toBuffer()],
            new web3.PublicKey(programId)
        )

        const instruction = new web3.TransactionInstruction({
            keys: [
                {
                    pubkey: publicKey,
                    isSigner: true,
                    isWritable: false,
                },
                {
                    pubkey: pda,
                    isSigner: false,
                    isWritable: true
                },
                {
                    pubkey: web3.SystemProgram.programId,
                    isSigner: false,
                    isWritable: false
                }
            ],
            data: buffer,
            programId: new web3.PublicKey(programId)
        })

        transaction.add(instruction)

        try {
            let txid = await sendTransaction(transaction, connection)
            alert(`Transaction submitted:${txid}`)
            console.log(`Transaction submitted:${txid}`)
        } catch (e) {
            console.log(JSON.stringify(e))
            alert(JSON.stringify(e))
        }
    }
