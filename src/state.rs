use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const POOL_STORAGE_TOTAL_BYTES: usize = 32 + 8 + 8 + 8 + 1;

/// Define the type of state stored in
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy)]
pub struct PoolStorageAccount {
    pub pool_authority: Pubkey,

    pub total_staked: u64,
    pub user_count: u64,
    pub rewards_per_token: u64,
    pub is_initialized: bool,
}

impl PoolStorageAccount {
    pub fn is_initialized(self) -> bool {
        self.is_initialized
    }
}
