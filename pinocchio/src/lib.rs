use pinocchio::{
    account_info::AccountInfo,
    instruction::Instruction,
    program::invoke_signed,
    program_error::ProgramError,
    program_entrypoint,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

program_entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        0 => create_account(accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

pub fn create_account(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer, new_account, system_program_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !new_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let system_program_id = Pubkey::from([0u8; 32]);
    if system_program_account.key() != &system_program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let space = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space as usize);

    let system_program_id = Pubkey::from([0u8; 32]);
    
    let mut instruction_data = Vec::with_capacity(52);
    instruction_data.extend_from_slice(&0u32.to_le_bytes());
    instruction_data.extend_from_slice(&lamports.to_le_bytes());
    instruction_data.extend_from_slice(&space.to_le_bytes());
    instruction_data.extend_from_slice(system_program_id.as_ref());

    let accounts = [
        pinocchio::instruction::AccountMeta::new(payer.key(), true, true),
        pinocchio::instruction::AccountMeta::new(new_account.key(), true, true),
    ];
    
    let create_account_ix = Instruction {
        program_id: &system_program_id,
        accounts: &accounts,
        data: &instruction_data,
    };

    invoke_signed(
        &create_account_ix,
        &[
            payer,
            new_account,
            system_program_account,
        ],
        &[],
    )?;

    Ok(())
}
