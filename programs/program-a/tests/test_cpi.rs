use litesvm::{types::TransactionResult, LiteSVM};
use solana_sdk::{
    message::{AccountMeta, Instruction, Message},
    pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::{Transaction, VersionedTransaction},
};

fn move_debug_port_var() {
    unsafe {
        let _ = std::env::var("VM_DEBUG_PORT").and_then(|debug_port| {
            std::env::remove_var("VM_DEBUG_PORT");
            std::env::set_var("DEBUG_PORT", debug_port);
            Ok(())
        });
    }
}

#[test]
fn test_cpi() {
    // TODO REMOVE
    move_debug_port_var();

    let mut svm: LiteSVM = LiteSVM::new();

    // If test fails fix program_ID's

    let program_id = pubkey!("ESHnYJDZfq2giPQeqmhqZucvPHiSVNjPoZxBV6dKKbHA");
    svm.add_program_from_file(program_id, "../../target/deploy/program_a.so")
        .unwrap();

    // Load program B
    let program_b_id = pubkey!("6CSmiViMaAguKgxNVwU8TWMPViQbtL5KKoFrDwWwtYNR");
    svm.add_program_from_file(program_b_id, "../../target/deploy/program_b.so")
        .unwrap();

    let signer_keypair = Keypair::new();
    let signer_pubkey = signer_keypair.pubkey();
    svm.airdrop(&signer_pubkey, 10_000_000).unwrap();

    // 1. This is the IDL of the cpi method in program A - see programs/program-a/src/lib.rs -> Discriminator of the cpi method
    let ix_data = vec![76, 173, 6, 95, 181, 93, 83, 206];

    // 2. Create the Accounts
    let accounts_ix = vec![
        // AccountMeta::new(signer_pubkey, true),
        AccountMeta::new(program_b_id, false),
        // AccountMeta::new_readonly(system_program::ID, false),
    ];

    // 3. Build instruction for program_a
    let instruction_a = Instruction {
        program_id,
        accounts: accounts_ix,
        data: ix_data,
    };

    // 5. Build and send transaction with both instructions
    // let message = Message::new(&[instruction_a.clone()], Some(&signer_pubkey));
    // let tx = Transaction::new(&[&signer_keypair], message, svm.latest_blockhash());

    // let result = svm.send_transaction(tx);
    // assert!(result.is_ok(), "Transaction failed: {:#?}", result.err()); // TODO: IMPORTANT usage of `#` will format the error
    let message = Message::new(&[instruction_a.clone()], Some(&signer_pubkey));
    let tx = Transaction::new(&[&signer_keypair], message, svm.latest_blockhash());

    // let result = svm.send_transaction(tx);
    let result = send_transaction(&mut svm, tx);
    assert!(result.is_ok(), "Transaction failed: {:#?}", result.err()); // TODO: IMPORTANT usage of `#` will format the error

    svm.expire_blockhash();
    let message = Message::new(&[instruction_a.clone()], Some(&signer_pubkey));
    let tx = Transaction::new(&[&signer_keypair], message, svm.latest_blockhash());

    // let result = svm.send_transaction(tx);
    let result = send_transaction_dbg(&mut svm, tx);
    assert!(result.is_ok(), "Transaction failed: {:#?}", result.err()); // TODO: IMPORTANT usage of `#` will format the error
}

fn test_non_cpi() {
    let mut svm: LiteSVM = LiteSVM::new();

    let program_id = pubkey!("2BJiU3UUhRmroYHXN6iEcbuw7PfDAJqcFRv9AFutQxzQ");
    svm.add_program_from_file(program_id, "../../target/deploy/program_a.so")
        .unwrap();

    let signer_keypair = Keypair::new();
    let signer_pubkey = signer_keypair.pubkey();
    svm.airdrop(&signer_pubkey, 10_000_000).unwrap();

    let accounts_ix = vec![];

    let ix_data = vec![86, 36, 10, 211, 246, 235, 42, 57];

    let instruction_a = Instruction {
        program_id,
        accounts: accounts_ix,
        data: ix_data,
    };

    let message = Message::new(&[instruction_a], Some(&signer_pubkey));
    let tx = Transaction::new(&[&signer_keypair], message, svm.latest_blockhash());

    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "Transaction failed: {:#?}", result.err()); // TODO: IMPORTANT usage of `#` will format the error
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
        match std::env::var("DEBUG_PORT") {
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
