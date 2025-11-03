import { AnchorProvider, Program, Wallet, web3, BN } from "@coral-xyz/anchor";
import { LiteSVM } from "../local-litesvm";

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

describe("anchor multi program, litesvm tests", () => {
    
    it("test_cpi_from_program_a_to_program_b", async () => {
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
    
        const result = svm_instance.sendTransaction(tx);
        console.log("Transaction result:", result);
    });

    it("test_non_cpi", async () => {
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

        const result = svm_instance.sendTransaction(tx);
        console.log("Transaction result:", result);
    });
})