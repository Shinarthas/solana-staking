use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::processor::process;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    process(program_id, accounts, instruction_data)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::clock::Epoch;
    use state;

    #[test]
    fn test_init() {
        //the unique address of our program
        let program_id = Pubkey::new_unique();
        //the unique address and metadata of our pool authority
        let signer_key = Pubkey::new_unique();
        let mut lamports = 0;
        let mut data = vec![];
        let owner = Pubkey::default();
        //create a pool authority account
        let account1 = AccountInfo::new(
            &signer_key,
            true,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        //the unique address and metadata of our pool storage account
        let pool_storage_key = Pubkey::new_unique();
        let mut lamports = 0;
        let mut data = vec![0; state::POOL_STORAGE_TOTAL_BYTES];
        //create a pool storage account
        let account2 = AccountInfo::new(
            &pool_storage_key,
            false,
            true,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            Epoch::default(),
        );
        let accounts = vec![account1, account2];
        //create an initialize instruction data
        let mut instruction_data = vec![];
        let init_instruction = instruction::Instruction::Initialize {
            rewards_per_token: 42u64,
        };
        init_instruction.serialize(&mut instruction_data).unwrap();
        //call the initialization function via the entrypoint
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        //deserialise the storage account
        let storage_account =
            state::PoolStorageAccount::try_from_slice(&accounts[1].data.borrow()).unwrap();

        //check that our storage data is correct
        assert_eq!(storage_account.is_initialized, true);
        assert_eq!(storage_account.pool_authority, signer_key);
        assert_eq!(storage_account.total_staked, 0u64);
        assert_eq!(storage_account.user_count, 0u64);
        assert_eq!(storage_account.rewards_per_token, 42u64);
    }
}
