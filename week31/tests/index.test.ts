import * as borsh from "borsh";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import { expect, test } from "bun:test";
import { COUNTER_SIZE, Schema } from "./types";

let adminAccount = Keypair.generate()
let dataAccount = Keypair.generate()

const PROGRAM_ID = new PublicKey("GXhCKXHm6h7sAdZ3AwJXoGE9QJoKbdcYXbrCympULq7r");
const connection = new Connection("http://127.0.0.1:8899","confirmed");

test("Account is initialized", async() => {
    // airdrop some SOL
    const airdrop = await connection.requestAirdrop(adminAccount.publicKey, 2 * LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdrop);

    // create new counter account
    const lamports = await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);

    const createCounterAccIx = SystemProgram.createAccount({                                // creating instruction for transaction
        fromPubkey: adminAccount.publicKey,
        lamports,
        newAccountPubkey: dataAccount.publicKey,
        space: COUNTER_SIZE,
        programId: PROGRAM_ID,
    });

    const txn = new Transaction();
    txn.add(createCounterAccIx);
    
    const txnHash = await connection.sendTransaction(txn, [adminAccount, dataAccount]);
    await connection.confirmTransaction(txnHash);

    console.log(dataAccount.publicKey.toBase58());
    const data2 = await connection.getAccountInfo(dataAccount.publicKey);
    console.log(data2);


    const dataAccountInfo = await connection.getAccountInfo(dataAccount.publicKey);
    if(!dataAccountInfo) {
        throw new Error("Counter account not found.");
    }
    const counter = borsh.deserialize(Schema, dataAccountInfo?.data);
    //@ts-ignore
    console.log(counter?.count);
    //@ts-ignore
    expect(counter?.count).toBe(0);
})