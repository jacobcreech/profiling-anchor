use anchor_lang::prelude::*;
use mollusk_svm::Mollusk;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use std::collections::HashMap;

const PROGRAM_ID: Pubkey = pubkey!("Bench11111111111111111111111111111111111111");

// Instruction discriminators from IDL
const ACCOUNT_INFO1_DISCRIMINATOR: [u8; 8] = [142, 40, 201, 119, 30, 63, 100, 96];
const ACCOUNT_INFO2_DISCRIMINATOR: [u8; 8] = [144, 73, 58, 143, 27, 156, 101, 137];
const ACCOUNT_INFO4_DISCRIMINATOR: [u8; 8] = [102, 5, 116, 73, 28, 75, 188, 48];
const ACCOUNT_INFO8_DISCRIMINATOR: [u8; 8] = [107, 201, 159, 220, 104, 122, 94, 127];

const ACCOUNT_EMPTY_INIT1_DISCRIMINATOR: [u8; 8] = [155, 119, 90, 210, 97, 111, 126, 120];
const ACCOUNT_EMPTY_INIT2_DISCRIMINATOR: [u8; 8] = [41, 37, 190, 46, 107, 244, 47, 78];
const ACCOUNT_EMPTY_INIT4_DISCRIMINATOR: [u8; 8] = [55, 51, 194, 145, 142, 38, 237, 185];
const ACCOUNT_EMPTY_INIT8_DISCRIMINATOR: [u8; 8] = [209, 227, 164, 233, 201, 97, 233, 0];

const ACCOUNT_EMPTY1_DISCRIMINATOR: [u8; 8] = [27, 215, 208, 247, 9, 145, 249, 91];
const ACCOUNT_EMPTY2_DISCRIMINATOR: [u8; 8] = [84, 66, 246, 144, 7, 252, 184, 237];
const ACCOUNT_EMPTY4_DISCRIMINATOR: [u8; 8] = [176, 94, 99, 249, 243, 103, 165, 234];
const ACCOUNT_EMPTY8_DISCRIMINATOR: [u8; 8] = [130, 71, 184, 16, 7, 115, 6, 48];

const SIGNER1_DISCRIMINATOR: [u8; 8] = [227, 100, 146, 156, 244, 163, 193, 156];
const SIGNER2_DISCRIMINATOR: [u8; 8] = [166, 167, 202, 252, 160, 250, 45, 194];
const SIGNER4_DISCRIMINATOR: [u8; 8] = [100, 119, 97, 65, 25, 33, 50, 54];
const SIGNER8_DISCRIMINATOR: [u8; 8] = [108, 116, 44, 48, 218, 88, 167, 72];

const SYSTEM_ACCOUNT1_DISCRIMINATOR: [u8; 8] = [123, 4, 142, 144, 44, 150, 97, 53];
const SYSTEM_ACCOUNT2_DISCRIMINATOR: [u8; 8] = [161, 170, 72, 203, 228, 66, 26, 122];
const SYSTEM_ACCOUNT4_DISCRIMINATOR: [u8; 8] = [133, 113, 94, 184, 111, 244, 22, 65];
const SYSTEM_ACCOUNT8_DISCRIMINATOR: [u8; 8] = [69, 161, 49, 100, 174, 150, 89, 55];

const UNCHECKED_ACCOUNT1_DISCRIMINATOR: [u8; 8] = [115, 15, 218, 130, 1, 171, 158, 18];
const UNCHECKED_ACCOUNT2_DISCRIMINATOR: [u8; 8] = [164, 133, 222, 212, 120, 70, 224, 45];
const UNCHECKED_ACCOUNT4_DISCRIMINATOR: [u8; 8] = [142, 85, 167, 136, 143, 36, 183, 53];
const UNCHECKED_ACCOUNT8_DISCRIMINATOR: [u8; 8] = [226, 42, 185, 113, 185, 171, 159, 5];

const PROGRAM1_DISCRIMINATOR: [u8; 8] = [17, 90, 225, 90, 34, 230, 203, 235];
const PROGRAM2_DISCRIMINATOR: [u8; 8] = [103, 200, 229, 250, 73, 97, 51, 168];
const PROGRAM4_DISCRIMINATOR: [u8; 8] = [17, 94, 108, 205, 164, 76, 250, 45];
const PROGRAM8_DISCRIMINATOR: [u8; 8] = [40, 117, 246, 134, 102, 141, 205, 225];

const INTERFACE1_DISCRIMINATOR: [u8; 8] = [7, 189, 115, 161, 126, 62, 133, 160];
const INTERFACE2_DISCRIMINATOR: [u8; 8] = [224, 246, 77, 183, 146, 7, 219, 189];
const INTERFACE4_DISCRIMINATOR: [u8; 8] = [23, 167, 153, 44, 26, 252, 217, 165];
const INTERFACE8_DISCRIMINATOR: [u8; 8] = [91, 42, 24, 245, 88, 108, 162, 226];

const INTERFACE_ACCOUNT_MINT1_DISCRIMINATOR: [u8; 8] = [124, 165, 40, 189, 127, 153, 182, 156];
const INTERFACE_ACCOUNT_MINT2_DISCRIMINATOR: [u8; 8] = [186, 87, 70, 200, 16, 162, 202, 142];
const INTERFACE_ACCOUNT_MINT4_DISCRIMINATOR: [u8; 8] = [45, 55, 167, 189, 53, 88, 229, 90];
const INTERFACE_ACCOUNT_MINT8_DISCRIMINATOR: [u8; 8] = [69, 171, 186, 5, 121, 76, 3, 255];

const INTERFACE_ACCOUNT_TOKEN1_DISCRIMINATOR: [u8; 8] = [183, 130, 150, 108, 240, 73, 16, 103];
const INTERFACE_ACCOUNT_TOKEN2_DISCRIMINATOR: [u8; 8] = [76, 85, 59, 31, 177, 245, 249, 74];
const INTERFACE_ACCOUNT_TOKEN4_DISCRIMINATOR: [u8; 8] = [31, 190, 198, 92, 116, 157, 107, 13];

// Account discriminators
const EMPTY_DISCRIMINATOR: [u8; 8] = [15, 64, 23, 223, 220, 243, 41, 219];
const SIZED_DISCRIMINATOR: [u8; 8] = [169, 85, 30, 7, 167, 74, 249, 159];
const UNSIZED_DISCRIMINATOR: [u8; 8] = [236, 83, 14, 167, 30, 250, 19, 183];

#[derive(Debug)]
struct ComputeUnits {
    units: HashMap<String, u64>,
}

impl ComputeUnits {
    fn new() -> Self {
        Self {
            units: HashMap::new(),
        }
    }

    fn record(&mut self, name: &str, units: u64) {
        self.units.insert(name.to_string(), units);
    }

    fn print_results(&self) {
        println!("\n=== Compute Units Results ===");
        let mut sorted: Vec<_> = self.units.iter().collect();
        sorted.sort_by_key(|&(k, _)| k);
        
        for (name, units) in sorted {
            println!("{}: {} CUs", name, units);
        }
    }
}

fn create_test_account(lamports: u64, data: Vec<u8>, owner: Pubkey) -> Account {
    Account {
        lamports,
        data,
        owner,
        executable: false,
        rent_epoch: 0,
    }
}

fn create_empty_account() -> Account {
    let mut data = vec![0u8; 8];
    data[..8].copy_from_slice(&EMPTY_DISCRIMINATOR);
    create_test_account(1_000_000, data, PROGRAM_ID)
}

fn create_sized_account() -> Account {
    let mut data = vec![0u8; 8 + 8]; // 8 bytes discriminator + 8 bytes field
    data[..8].copy_from_slice(&SIZED_DISCRIMINATOR);
    create_test_account(1_000_000, data, PROGRAM_ID)
}

fn create_unsized_account() -> Account {
    let mut data = vec![0u8; 8 + 4]; // 8 bytes discriminator + 4 bytes Vec length (empty)
    data[..8].copy_from_slice(&UNSIZED_DISCRIMINATOR);
    create_test_account(1_000_000, data, PROGRAM_ID)
}

fn create_mint_account() -> Account {
    let mut data = vec![0u8; 82]; // SPL Token Mint size
    data[0..4].copy_from_slice(&[1, 0, 0, 0]); // is_initialized = true
    create_test_account(1_000_000, data, anchor_spl::token::ID)
}

fn create_token_account() -> Account {
    let mut data = vec![0u8; 165]; // SPL Token Account size
    data[0..4].copy_from_slice(&[1, 0, 0, 0]); // is_initialized = true
    create_test_account(1_000_000, data, anchor_spl::token::ID)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mollusk() -> Mollusk {
        Mollusk::new(&PROGRAM_ID, "../../target/deploy/profile")
    }

    #[test]
    fn test_account_info() {
        let mollusk = setup_mollusk();
        let mut compute_units = ComputeUnits::new();
        
        let test_cases = vec![
            (1, ACCOUNT_INFO1_DISCRIMINATOR),
            (2, ACCOUNT_INFO2_DISCRIMINATOR),
            (4, ACCOUNT_INFO4_DISCRIMINATOR),
            (8, ACCOUNT_INFO8_DISCRIMINATOR),
        ];
        
        for (count, discriminator) in test_cases {
            let method_name = format!("accountInfo{}", count);
            
            // Create accounts
            let mut accounts = vec![];
            for _ in 0..count {
                let pubkey = Pubkey::new_unique();
                let account = Account::new(1_000_000, 0, &system_program::ID);
                accounts.push((pubkey, account));
            }
            
            // Create instruction
            let account_metas: Vec<AccountMeta> = accounts
                .iter()
                .map(|(pubkey, _)| AccountMeta::new_readonly(*pubkey, false))
                .collect();
            
            let instruction = Instruction {
                program_id: PROGRAM_ID,
                accounts: account_metas,
                data: discriminator.to_vec(),
            };
            
            // Execute
            let result = mollusk.process_instruction(
                &instruction,
                &accounts,
            );
            
            compute_units.record(&method_name, result.compute_units_consumed);
        }
        
        compute_units.print_results();
    }

    #[test]
    fn test_account_empty() {
        let mollusk = setup_mollusk();
        let mut compute_units = ComputeUnits::new();
        
        // Test init versions
        let init_cases = vec![
            (1, ACCOUNT_EMPTY_INIT1_DISCRIMINATOR),
            (2, ACCOUNT_EMPTY_INIT2_DISCRIMINATOR),
            (4, ACCOUNT_EMPTY_INIT4_DISCRIMINATOR),
            (8, ACCOUNT_EMPTY_INIT8_DISCRIMINATOR),
        ];
        
        for (count, discriminator) in init_cases {
            let method_name = format!("accountEmptyInit{}", count);
            
            // Create accounts
            let mut accounts = vec![];
            
            // Payer
            let payer = Pubkey::new_unique();
            let payer_account = Account::new(10_000_000, 0, &system_program::ID);
            accounts.push((payer, payer_account));
            
            // System program
            accounts.push((system_program::ID, Account::default()));

            // Accounts to initialize
            for _ in 0..count {
                let pubkey = Pubkey::new_unique();
                let account = Account::new(0, 0, &system_program::ID);
                accounts.push((pubkey, account));
            }
            
            // Create instruction
            let mut account_metas = vec![];
            account_metas.push(AccountMeta::new(accounts[0].0, true)); // payer
            account_metas.push(AccountMeta::new_readonly(accounts[1].0, false)); // system program
            
            for i in 2..accounts.len() {
                account_metas.push(AccountMeta::new(accounts[i].0, true));
            }
            
            let instruction = Instruction {
                program_id: PROGRAM_ID,
                accounts: account_metas,
                data: discriminator.to_vec(),
            };
            
            // Execute
            let result = mollusk.process_instruction(
                &instruction,
                &accounts,
            );
            
            compute_units.record(&method_name, result.compute_units_consumed);
        }
        
        // Test non-init versions
        let non_init_cases = vec![
            (1, ACCOUNT_EMPTY1_DISCRIMINATOR),
            (2, ACCOUNT_EMPTY2_DISCRIMINATOR),
            (4, ACCOUNT_EMPTY4_DISCRIMINATOR),
            (8, ACCOUNT_EMPTY8_DISCRIMINATOR),
        ];
        
        for (count, discriminator) in non_init_cases {
            let method_name = format!("accountEmpty{}", count);
            
            // Create accounts
            let mut accounts = vec![];
            for _ in 0..count {
                let pubkey = Pubkey::new_unique();
                let account = create_empty_account();
                accounts.push((pubkey, account));
            }
            
            // Create instruction
            let account_metas: Vec<AccountMeta> = accounts
                .iter()
                .map(|(pubkey, _)| AccountMeta::new_readonly(*pubkey, false))
                .collect();
            
            let instruction = Instruction {
                program_id: PROGRAM_ID,
                accounts: account_metas,
                data: discriminator.to_vec(),
            };
            
            // Execute
            let result = mollusk.process_instruction(
                &instruction,
                &accounts,
            );
            
            compute_units.record(&method_name, result.compute_units_consumed);
        }
        
        compute_units.print_results();
    }

    #[test]
    fn test_signer() {
        let mollusk = setup_mollusk();
        let mut compute_units = ComputeUnits::new();
        
        let test_cases = vec![
            (1, SIGNER1_DISCRIMINATOR),
            (2, SIGNER2_DISCRIMINATOR),
            (4, SIGNER4_DISCRIMINATOR),
            (8, SIGNER8_DISCRIMINATOR),
        ];
        
        for (count, discriminator) in test_cases {
            let method_name = format!("signer{}", count);
            
            // Create accounts
            let mut accounts = vec![];
            for _ in 0..count {
                let pubkey = Pubkey::new_unique();
                let account = Account::new(1_000_000, 0, &system_program::ID);
                accounts.push((pubkey, account));
            }
            
            // Create instruction
            let account_metas: Vec<AccountMeta> = accounts
                .iter()
                .map(|(pubkey, _)| AccountMeta::new_readonly(*pubkey, true))
                .collect();
            
            let instruction = Instruction {
                program_id: PROGRAM_ID,
                accounts: account_metas,
                data: discriminator.to_vec(),
            };
            
            // Execute
            let result = mollusk.process_instruction(
                &instruction,
                &accounts,
            );
            
            compute_units.record(&method_name, result.compute_units_consumed);
        }
        
        compute_units.print_results();
    }

    #[test]
    fn test_system_account() {
        let mollusk = setup_mollusk();
        let mut compute_units = ComputeUnits::new();
        
        let test_cases = vec![
            (1, SYSTEM_ACCOUNT1_DISCRIMINATOR),
            (2, SYSTEM_ACCOUNT2_DISCRIMINATOR),
            (4, SYSTEM_ACCOUNT4_DISCRIMINATOR),
            (8, SYSTEM_ACCOUNT8_DISCRIMINATOR),
        ];
        
        for (count, discriminator) in test_cases {
            let method_name = format!("systemAccount{}", count);
            
            // Create accounts
            let mut accounts = vec![];
            for _ in 0..count {
                let pubkey = Pubkey::new_unique();
                let account = Account::new(1_000_000, 0, &system_program::ID);
                accounts.push((pubkey, account));
            }
            
            // Create instruction
            let account_metas: Vec<AccountMeta> = accounts
                .iter()
                .map(|(pubkey, _)| AccountMeta::new_readonly(*pubkey, false))
                .collect();
            
            let instruction = Instruction {
                program_id: PROGRAM_ID,
                accounts: account_metas,
                data: discriminator.to_vec(),
            };

            // Execute
            let result = mollusk.process_instruction(
                &instruction,
                &accounts,
            );
            
            compute_units.record(&method_name, result.compute_units_consumed);
        }
        
        compute_units.print_results();
    }

    #[test]
    fn test_unchecked_account() {
        let mollusk = setup_mollusk();
        let mut compute_units = ComputeUnits::new();

        let test_cases = vec![
            (1, UNCHECKED_ACCOUNT1_DISCRIMINATOR),
            (2, UNCHECKED_ACCOUNT2_DISCRIMINATOR),
            (4, UNCHECKED_ACCOUNT4_DISCRIMINATOR),
            (8, UNCHECKED_ACCOUNT8_DISCRIMINATOR),
        ];
        
        for (count, discriminator) in test_cases {
            let method_name = format!("uncheckedAccount{}", count);
            
            // Create accounts
            let mut accounts = vec![];
            for _ in 0..count {
                let pubkey = Pubkey::new_unique();
                let account = Account::new(1_000_000, 0, &system_program::ID);
                accounts.push((pubkey, account));
            }
            
            // Create instruction
            let account_metas: Vec<AccountMeta> = accounts
                .iter()
                .map(|(pubkey, _)| AccountMeta::new_readonly(*pubkey, false))
                .collect();
            
            let instruction = Instruction {
                program_id: PROGRAM_ID,
                accounts: account_metas,
                data: discriminator.to_vec(),
            };
            
            // Execute
            let result = mollusk.process_instruction(
                &instruction,
                &accounts,
            );
            
            compute_units.record(&method_name, result.compute_units_consumed);
        }
        
        compute_units.print_results();
    }

    #[test]
    fn test_program() {
        let mollusk = setup_mollusk();
        let mut compute_units = ComputeUnits::new();
        
        let test_cases = vec![
            (1, PROGRAM1_DISCRIMINATOR),
            (2, PROGRAM2_DISCRIMINATOR),
            (4, PROGRAM4_DISCRIMINATOR),
            (8, PROGRAM8_DISCRIMINATOR),
        ];
        
        for (count, discriminator) in test_cases {
            let method_name = format!("program{}", count);
            
            // Create accounts
            let mut accounts = vec![];
            for _ in 0..count {
                accounts.push((system_program::ID, Account::default()));
            }
            
            // Create instruction
            let account_metas: Vec<AccountMeta> = accounts
                .iter()
                .map(|(pubkey, _)| AccountMeta::new_readonly(*pubkey, false))
                .collect();
            
            let instruction = Instruction {
                program_id: PROGRAM_ID,
                accounts: account_metas,
                data: discriminator.to_vec(),
            };
            
            // Execute
            let result = mollusk.process_instruction(
                &instruction,
                &accounts,
            );
            
            compute_units.record(&method_name, result.compute_units_consumed);
        }
        
        compute_units.print_results();
    }

    #[test]
    fn test_interface() {
        let mollusk = setup_mollusk();
        let mut compute_units = ComputeUnits::new();
        
        let test_cases = vec![
            (1, INTERFACE1_DISCRIMINATOR),
            (2, INTERFACE2_DISCRIMINATOR),
            (4, INTERFACE4_DISCRIMINATOR),
            (8, INTERFACE8_DISCRIMINATOR),
        ];
        
        for (count, discriminator) in test_cases {
            let method_name = format!("interface{}", count);
            
            // Create accounts
            let mut accounts = vec![];
            for _ in 0..count {
                accounts.push((anchor_spl::token::ID, Account::default()));
            }
            
            // Create instruction
            let account_metas: Vec<AccountMeta> = accounts
                .iter()
                .map(|(pubkey, _)| AccountMeta::new_readonly(*pubkey, false))
                .collect();
            
            let instruction = Instruction {
                program_id: PROGRAM_ID,
                accounts: account_metas,
                data: discriminator.to_vec(),
            };
            
            // Execute
            let result = mollusk.process_instruction(
                &instruction,
                &accounts,
            );
            
            compute_units.record(&method_name, result.compute_units_consumed);
        }
        
        compute_units.print_results();
    }

    #[test]
    fn test_interface_account_mint() {
        let mollusk = setup_mollusk();
        let mut compute_units = ComputeUnits::new();
        
        let test_cases = vec![
            (1, INTERFACE_ACCOUNT_MINT1_DISCRIMINATOR),
            (2, INTERFACE_ACCOUNT_MINT2_DISCRIMINATOR),
            (4, INTERFACE_ACCOUNT_MINT4_DISCRIMINATOR),
            (8, INTERFACE_ACCOUNT_MINT8_DISCRIMINATOR),
        ];
        
        for (count, discriminator) in test_cases {
            let method_name = format!("interfaceAccountMint{}", count);
            
            // Create accounts
            let mut accounts = vec![];
            for _ in 0..count {
                let pubkey = Pubkey::new_unique();
                let account = create_mint_account();
                accounts.push((pubkey, account));
            }
            
            // Create instruction
            let account_metas: Vec<AccountMeta> = accounts
                .iter()
                .map(|(pubkey, _)| AccountMeta::new_readonly(*pubkey, false))
                .collect();
            
            let instruction = Instruction {
                program_id: PROGRAM_ID,
                accounts: account_metas,
                data: discriminator.to_vec(),
            };
            
            // Execute
            let result = mollusk.process_instruction(
                &instruction,
                &accounts,
            );
            
            compute_units.record(&method_name, result.compute_units_consumed);
        }
        
        compute_units.print_results();
    }

    #[test]
    fn test_interface_account_token() {
        let mollusk = setup_mollusk();
        let mut compute_units = ComputeUnits::new();
        
        let test_cases = vec![
            (1, INTERFACE_ACCOUNT_TOKEN1_DISCRIMINATOR),
            (2, INTERFACE_ACCOUNT_TOKEN2_DISCRIMINATOR),
            (4, INTERFACE_ACCOUNT_TOKEN4_DISCRIMINATOR),
        ];
        
        for (count, discriminator) in test_cases {
            let method_name = format!("interfaceAccountToken{}", count);
            
            // Create accounts
            let mut accounts = vec![];
            for _ in 0..count {
                let pubkey = Pubkey::new_unique();
                let account = create_token_account();
                accounts.push((pubkey, account));
            }
            
            // Create instruction
            let account_metas: Vec<AccountMeta> = accounts
                .iter()
                .map(|(pubkey, _)| AccountMeta::new_readonly(*pubkey, false))
                .collect();
            
            let instruction = Instruction {
                program_id: PROGRAM_ID,
                accounts: account_metas,
                data: discriminator.to_vec(),
            };
            
            // Execute
            let result = mollusk.process_instruction(
                &instruction,
                &accounts,
            );
            
            compute_units.record(&method_name, result.compute_units_consumed);
        }
        
        compute_units.print_results();
    }
}