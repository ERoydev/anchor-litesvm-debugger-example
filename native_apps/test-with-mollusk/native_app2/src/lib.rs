use borsh::{from_slice, to_vec, BorshDeserialize, BorshSerialize};
use solana_msg::msg;
use solana_program::account_info::next_account_info;
use solana_program::entrypoint::{entrypoint, ProgramResult};
use solana_program::sysvar::slot_history::AccountInfo;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

// solana-keygen new --outfile my_program_id.json
// solana-keygen pubkey ./my_program_id.json
pub static MY_PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("4Qh92Yw97hNn32ZSUfW1kV3PTuqTVSsHNVTEs8uGoS7W");

mod account_ops;
mod tests;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct HolderData {
    inner: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Holder {
    pub owner: Pubkey,
    pub data: HolderData,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Instruction {
    Initialize,
    SetHolderData(HolderData),
}

pub fn process_instruction<'info>(
    program_id: &Pubkey,
    account_infos: &[AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    msg!(
        "Hello {} from native app! Expected: {}",
        program_id,
        MY_PROGRAM_ID
    );

    let err = solana_program_error::ProgramError::IncorrectProgramId;
    if program_id != &MY_PROGRAM_ID {
        return Err(err);
    }

    let mut accounts_iter = account_infos.iter();
    let payer = next_account_info(&mut accounts_iter)?;
    let system_program = next_account_info(&mut accounts_iter)?;
    // must pass it even if not created right now!
    let pda_account = next_account_info(&mut accounts_iter)?;

    // deserialize the data back from borsh and see what enum variant we're called!
    let instr: Instruction = from_slice(data)?;
    match instr {
        Instruction::Initialize => {
            msg!("Trying to initialize!");
            // Check if account already exists
            if pda_account.lamports() != 0 {
                return Err(solana_program_error::ProgramError::AccountAlreadyInitialized);
            }
            msg!(
                "before creating - pda account lamports: {}",
                pda_account.lamports()
            );
            let holder = Holder {
                owner: *payer.key,
                data: HolderData {
                    inner: String::new(),
                },
            };
            let new_data = to_vec(&holder)?;
            let space = new_data.len();
            crate::account_ops::create_account(
                program_id,
                payer,
                system_program,
                space as u64,
                pda_account,
            )?;
            msg!(
                "after creation - pda account lamports: {}",
                pda_account.lamports()
            );

            pda_account
                .try_borrow_mut_data()?
                .copy_from_slice(&new_data);
            msg!("Initialized!");
        }
        Instruction::SetHolderData(holder_data) => {
            if pda_account.lamports() == 0 {
                return Err(solana_program_error::ProgramError::UninitializedAccount);
            }
            {
                let current_data = pda_account
                    .data
                    .try_borrow()
                    .map_err(|_| ProgramError::AccountBorrowFailed)?;
                // mind that borsh fails if given full buffer instead of the borsh value!
                let current_holder: Holder = from_slice(&current_data)?;
                if current_holder.owner != *payer.key {
                    return Err(ProgramError::InvalidArgument);
                }
            }
            let new_data = to_vec(&holder_data)?;
            // we have to realloc coz new holder data may be bigger
            pda_account.realloc(new_data.len(), false)?;

            let mut data = pda_account
                .data
                .try_borrow_mut()
                .map_err(|_| ProgramError::AccountBorrowFailed)?;
            data.copy_from_slice(&new_data);
            msg!("Set string: {}!", holder_data.inner);
        }
    }

    Ok(())
}

entrypoint!(process_instruction);
