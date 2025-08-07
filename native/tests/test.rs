#[cfg(test)]
mod tests {
    use mollusk_svm::Mollusk;
    use solana_sdk::{
        account::{Account, ReadableAccount},
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        pubkey::Pubkey,
        system_program,
    };

    #[test]
    fn test_create_account_happy_path() {
        let program_id = Pubkey::new_unique();
        let mut mollusk = Mollusk::new(&program_id, "target/deploy/create_account_native");

        let payer = Pubkey::new_unique();
        let new_account = Pubkey::new_unique();

        let payer_account = Account {
            lamports: 10 * LAMPORTS_PER_SOL,
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

        let system_program_account = Account {
            lamports: 0,
            data: vec![],
            owner: solana_sdk::native_loader::id(),
            executable: true,
            rent_epoch: 0,
        };

        let instruction = Instruction::new_with_bytes(
            program_id,
            &[],
            vec![
                AccountMeta::new(payer, true),
                AccountMeta::new(new_account, true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (payer, payer_account),
                (new_account, new_account_data),
                (system_program::id(), system_program_account),
            ],
        );

        assert!(matches!(result.program_result, mollusk_svm::result::ProgramResult::Success));
        
        let resulting_accounts = result.resulting_accounts;
        let new_account_after = resulting_accounts.iter()
            .find(|(key, _)| *key == new_account)
            .map(|(_, account)| account)
            .unwrap();
        
        assert_eq!(new_account_after.lamports(), LAMPORTS_PER_SOL);
    }
}