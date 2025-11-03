import { AnchorProvider, Program, Wallet, web3, BN } from "@coral-xyz/anchor";
import { FailedTransactionMetadata, LiteSVM, TransactionMetadata } from "../local-litesvm";

import A_IDL from "../target/idl/program_a.json";
import B_IDL from "../target/idl/program_b.json";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
} from "@solana/web3.js";
import { ProgramA} from "../target/types/program_a";
import { ProgramB } from "../target/types/program_b";
import { assert } from "chai";

function moveDebugPortVar() {
  const debugPort = process.env.VM_DEBUG_PORT;
  if (debugPort !== undefined) {
    delete process.env.VM_DEBUG_PORT;
    process.env.DEBUG_PORT = debugPort;
  }
}

describe("anchor multi program, litesvm tests", () => {
    
    it("test_cpi_from_program_a_to_program_b", async () => {
        moveDebugPortVar();

        const svm_instance = new LiteSVM();

        const program_id = new PublicKey("ESHnYJDZfq2giPQeqmhqZucvPHiSVNjPoZxBV6dKKbHA");
        svm_instance.addProgramFromFile(program_id, "target/deploy/program_a.so");
        
        const program_b_Id = new PublicKey("6CSmiViMaAguKgxNVwU8TWMPViQbtL5KKoFrDwWwtYNR");
        svm_instance.addProgramFromFile(program_b_Id, "target/deploy/program_b.so");

        const signer_keypair = new Keypair();
        const signer_pubkey = signer_keypair.publicKey;
        svm_instance.airdrop(signer_pubkey, BigInt(10_000_000));

        const accounts_ix = [
            {
                pubkey: program_b_Id,
                isSigner: false,
                isWritable: true
            }
        ];
        const ix_data = new Uint8Array([
            76,
            173,
            6,
            95,
            181,
            93,
            83,
            206
        ]);

        const cpi_instruction = new web3.TransactionInstruction({
            programId: program_id,
            keys: accounts_ix,
            data: Buffer.from(ix_data),
        });

        const blockhash = svm_instance.latestBlockhash();
        const tx = new web3.Transaction();
        tx.recentBlockhash = blockhash;
        tx.add(cpi_instruction);
        tx.sign(signer_keypair);
    
        const result = sendTransactionDbg(svm_instance, tx);
        console.log("Transaction result:", result);
    });

    it("test_non_cpi", async () => {
        moveDebugPortVar();

        const svm_instance = new LiteSVM();
        
        const program_id = new PublicKey("ESHnYJDZfq2giPQeqmhqZucvPHiSVNjPoZxBV6dKKbHA");
        svm_instance.addProgramFromFile(program_id, "target/deploy/program_a.so");

        const signer_keypair = new Keypair();
        const signer_pubkey = signer_keypair.publicKey;
        svm_instance.airdrop(signer_pubkey, BigInt(10_000_000));

        const accounts_ix = [];
        const ix_data = new Uint8Array([86, 36, 10, 211, 246, 235, 42, 57]);

        const instruction_a = new web3.TransactionInstruction({
            programId: program_id,
            keys: accounts_ix,
            data: Buffer.from(ix_data),
        });

        const blockhash = svm_instance.latestBlockhash();
        const tx = new web3.Transaction();
        tx.recentBlockhash = blockhash;
        tx.add(instruction_a);
        tx.sign(signer_keypair);

        const result = sendTransaction(svm_instance, tx);
        console.log("Transaction result:", result);
    });
})

function sendTransactionDbg(svm: LiteSVM, tx: web3.Transaction): TransactionMetadata | FailedTransactionMetadata  {
    let _debug_port = process.env.DEBUG_PORT;
    process.env.VM_DEBUG_PORT = _debug_port;
    try {
        return svm.sendTransaction(tx);
    } catch (e) {
        throw e;
    } finally {
        if (_debug_port !== undefined) {
            delete process.env.VM_DEBUG_PORT;
        }
    }
}

function sendTransaction(svm: LiteSVM, tx: web3.Transaction): TransactionMetadata | FailedTransactionMetadata {
    return svm.sendTransaction(tx);
}