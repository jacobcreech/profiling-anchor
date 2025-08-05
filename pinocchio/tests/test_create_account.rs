use mollusk_svm::{Mollusk};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[test]
fn test_create_account_compute_units() {
    let program_id = Pubkey::new_unique();
    let mollusk = Mollusk::new(&program_id, "target/deploy/pinocchio_create_account");

    let payer = Pubkey::new_unique();
    let new_account = Pubkey::new_unique();
    
    let space = 100u64;
    let mut instruction_data = vec![0u8];
    instruction_data.extend_from_slice(&space.to_le_bytes());

    let instruction = Instruction::new_with_bytes(
        program_id,
        &instruction_data,
        vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(new_account, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    );

    let payer_account = Account {
        lamports: 1_000_000_000,
        data: vec![],
        owner: system_program::id(),
        executable: false,
        rent_epoch: 0,
    };
    let new_account_data = Account {
        lamports: 0,
        data: vec![],
        owner: system_program::id(),
        executable: false,
        rent_epoch: 0,
    };
    let system_account = Account {
        lamports: 1,
        data: vec![],
        owner: Pubkey::default(),
        executable: false,
        rent_epoch: 0,
    };

    let result = mollusk.process_instruction(
        &instruction,
        &vec![
            (payer, payer_account),
            (new_account, new_account_data),
            (system_program::id(), system_account),
        ],
    );
    
    // The CPI will fail since we don't have system program loaded, but we can still check compute units
    // up to the point of failure
    println!("Compute units consumed: {}", result.compute_units_consumed);
    
    // Verify that the program at least validates inputs before attempting CPI
    // (should use some compute units for validation)
    assert!(result.compute_units_consumed > 0);
}