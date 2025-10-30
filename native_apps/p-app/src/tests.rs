#[cfg(test)]
mod tests {
    use litesvm::LiteSVM;
    use pinocchio::ProgramResult;
    use solana_sdk::instruction::Instruction;
    use solana_sdk::transaction::Transaction;
    use solana_sdk::{message::AccountMeta, pubkey, signature::Keypair, signer::Signer};

    #[test]
    fn test_process_instruction_litesvm_sbf() -> ProgramResult {
        let mut svm = LiteSVM::new();
        let payer_kp = Keypair::new();
        let payer_pk = payer_kp.pubkey();
        println!("Payer address: {}", payer_pk);
        let program_id = pubkey!("Papp111111111111111111111111111111111111111");
        println!("Program id: {}", program_id);
        let bytes = include_bytes!("../target/deploy/p_app.so");
        svm.add_program(program_id.clone(), bytes).unwrap();

        svm.airdrop(&payer_pk, 1000000000).unwrap();
        let recent_blockhash = svm.latest_blockhash();

        let ix_accounts = vec![
            AccountMeta::new(payer_pk, true),
            AccountMeta::new(pubkey!("11111111111111111111111111111111"), false),
        ];

        let ix_data = [];
        let instructions = [Instruction::new_with_bytes(
            program_id,
            &ix_data,
            ix_accounts,
        )];

        let trans = Transaction::new_signed_with_payer(
            &instructions[..],
            Some(&payer_pk),
            &[&payer_kp],
            recent_blockhash,
        );

        let res = svm.send_transaction(trans.clone()).unwrap();
        println!(
            "p_app's process_instruction initialize -> {}",
            res.pretty_logs()
        );

        Ok(())
    }
}
