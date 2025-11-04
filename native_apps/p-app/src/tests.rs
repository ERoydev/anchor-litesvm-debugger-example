use litesvm::{types::TransactionResult, LiteSVM};
use solana_sdk::transaction::VersionedTransaction;

#[cfg(test)]
mod tests {
    use litesvm::LiteSVM;
    use pinocchio::ProgramResult;
    use solana_sdk::instruction::Instruction;
    use solana_sdk::transaction::Transaction;
    use solana_sdk::{message::AccountMeta, pubkey, signature::Keypair, signer::Signer};

    use crate::tests::send_transaction_dbg;

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

        let res = send_transaction_dbg(&mut svm, trans.clone()).unwrap();
        println!(
            "p_app's process_instruction initialize -> {}",
            res.pretty_logs()
        );

        Ok(())
    }
}

fn send_transaction_dbg(
    litesvm: &mut LiteSVM,
    tx: impl Into<VersionedTransaction>,
) -> TransactionResult {
    let _debug_port = DebugPort::open();
    litesvm.send_transaction(tx)
}

fn send_transaction(
    litesvm: &mut LiteSVM,
    tx: impl Into<VersionedTransaction>,
) -> TransactionResult {
    litesvm.send_transaction(tx)
}

static ENV_VARS_MTX: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub struct DebugPort<'guard> {
    _guard: std::sync::MutexGuard<'guard, ()>,
}

impl<'guard> DebugPort<'guard> {
    pub fn open() -> Option<Self> {
        match std::env::var("SBPF_DEBUG_PORT") {
            Err(_) => None,
            Ok(debug_port) => {
                let guard = ENV_VARS_MTX.lock().unwrap();
                unsafe {
                    std::env::set_var("VM_DEBUG_PORT", debug_port);
                }
                Some(Self { _guard: guard })
            }
        }
    }
}

impl<'guard> Drop for DebugPort<'guard> {
    fn drop(&mut self) {
        unsafe {
            std::env::remove_var("VM_DEBUG_PORT");
        }
    }
}
