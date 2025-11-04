#[cfg(test)]
mod tests {
    use crate::MY_PROGRAM_ID;

    use mollusk_svm::{result::InstructionResult, Mollusk};
    use solana_sdk::{account::Account, message::Instruction, pubkey::Pubkey};

    #[test]
    fn test_process_instruction_mollusk_sbf(
    ) -> Result<(), solana_sdk::instruction::InstructionError> {
        let payer_pk = solana_sdk::pubkey::Pubkey::new_unique();
        println!("Payer address: {}", payer_pk);
        let program_id = solana_sdk::pubkey::Pubkey::new_from_array(*MY_PROGRAM_ID.as_array());
        println!("Program id: {}", program_id);

        let mollusk = Mollusk::new(&program_id, "native_app2");

        // Derive expected PDA and bump (example seeds)
        let seeds = &[b"holder_seed", payer_pk.as_ref()];
        let (pda_pubkey, _bump) =
            solana_sdk::pubkey::Pubkey::find_program_address(seeds, &program_id);

        let (system_program_id, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        let ix_data = borsh::to_vec(&crate::Instruction::Initialize).unwrap();
        let instruction = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &ix_data,
            vec![
                solana_sdk::instruction::AccountMeta::new(payer_pk, true),
                solana_sdk::instruction::AccountMeta::new(system_program_id, false),
                solana_sdk::instruction::AccountMeta::new(pda_pubkey, false),
            ],
        );

        let base_lamports = 100_000_000u64;
        let accounts = vec![
            (
                payer_pk,
                solana_sdk::account::Account::new(base_lamports, 0, &system_program_id),
            ),
            (system_program_id, system_account),
            (pda_pubkey, solana_sdk::account::Account::default()),
        ];

        // Execute the instruction and get the result.
        let result = process_instruction_gdb(&mollusk, &instruction, &accounts);
        println!("mollusk result: {:?}", result);

        result.raw_result
    }

    fn process_instruction_gdb(
        mollusk: &Mollusk,
        instruction: &Instruction,
        accounts: &[(Pubkey, Account)],
    ) -> InstructionResult {
        let _debug_port = DebugPort::open();
        mollusk.process_instruction(instruction, accounts)
    }

    fn process_instruction(
        mollusk: &Mollusk,
        instruction: &Instruction,
        accounts: &[(Pubkey, Account)],
    ) -> InstructionResult {
        let _debug_port = DebugPort::open();
        mollusk.process_instruction(instruction, accounts)
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
}
