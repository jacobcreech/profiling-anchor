use anchor_lang::prelude::*;
use mollusk_svm::{Mollusk, result::ProgramResult};
use solana_sdk::{
    account::{Account, ReadableAccount},
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    rent::Rent,
};

const PROGRAM_ID: Pubkey = pubkey!("Bench11111111111111111111111111111111111111");

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mollusk() -> Mollusk {
        Mollusk::new(&PROGRAM_ID, "../target/deploy/profile")
    }

    #[test]
    fn test_create_system_account_success() {
        let mollusk = setup_mollusk();
        
        let payer = Keypair::new();
        let new_account = Keypair::new();
        
        let rent = Rent::default();
        let lamports = rent.minimum_balance(0);
        
        let accounts = vec![
            (
                payer.pubkey(),
                Account {
                    lamports: 1_000_000_000,
                    data: vec![],
                    owner: system_program::id(),
                    executable: false,
                    rent_epoch: 0,
                },
            ),
            (
                new_account.pubkey(),
                Account {
                    lamports: 0,
                    data: vec![],
                    owner: system_program::id(),
                    executable: false,
                    rent_epoch: 0,
                },
            ),
            (
                system_program::id(),
                Account {
                    lamports: 1,
                    data: vec![],
                    owner: solana_sdk::native_loader::id(),
                    executable: true,
                    rent_epoch: 0,
                },
            ),
        ];
        
        let instruction_data = anchor_lang::InstructionData::data(
            &profile::instruction::CreateSystemAccount {}
        );
        
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &instruction_data,
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(new_account.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );
        
        let result = mollusk.process_instruction(
            &instruction,
            &accounts,
        );
        
        match result.program_result {
            ProgramResult::Success => {},
            _ => panic!("Expected success"),
        }
        
        let resulting_account = result.resulting_accounts
            .iter()
            .find(|(key, _)| key == &new_account.pubkey())
            .map(|(_, account)| account)
            .expect("Account should exist");
        
        assert_eq!(resulting_account.lamports(), lamports);
        assert_eq!(resulting_account.owner(), &system_program::id());
        assert_eq!(resulting_account.data().len(), 0);
    }
}