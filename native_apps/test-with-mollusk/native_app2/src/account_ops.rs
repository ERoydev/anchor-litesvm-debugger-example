use solana_program::program::invoke_signed;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, rent::Rent};
use solana_pubkey::Pubkey;

pub fn create_account<'info>(
    program_id: &Pubkey,
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    space: u64,
    pda_account: &AccountInfo<'info>,
) -> ProgramResult {
    // Derive expected PDA and bump (example seeds)
    let seeds = &[b"holder_seed", payer.key.as_ref()];
    let (pda_pubkey, bump) = Pubkey::find_program_address(seeds, program_id);

    // This is the preferred way to load a sysvar. Calling this method does
    // not incur any deserialization overhead, and does not require the sysvar
    // account to be passed to the program.
    let lamports_required = Rent::get()?.minimum_balance(space as usize);
    let create_ix = system_instruction::create_account(
        &payer.key,
        &pda_pubkey,
        lamports_required,
        space,
        program_id,
    );

    // Effectively a CPI.
    let signer_seeds = &[seeds[0], seeds[1], &[bump]];
    invoke_signed(
        &create_ix,
        &[payer.clone(), system_program.clone(), pda_account.clone()],
        &[signer_seeds],
    )?;

    Ok(())
}
