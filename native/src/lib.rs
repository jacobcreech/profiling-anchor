use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    native_token::LAMPORTS_PER_SOL,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_sdk_ids::system_program;
use solana_system_interface::instruction as system_instruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // Direct array access instead of iterator
    let [payer, new_account, system_program_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Combined signer checks
    if !payer.is_signer || !new_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Direct comparison without intermediate variable
    if system_program_account.key != &system_program::id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Simplified initialization check - only check lamports since that's the most critical
    if new_account.lamports() > 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    invoke(
        &system_instruction::create_account(
            payer.key,
            new_account.key,
            LAMPORTS_PER_SOL,
            0,
            &system_program::id(),
        ),
        &[payer.clone(), new_account.clone(), system_program_account.clone()],
    )?;

    Ok(())
}