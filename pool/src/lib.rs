pub mod application;
pub mod models;

use models::poolcontract::{PoolContract, PoolContractExt, PoolContractMetadata, PoolStorageKey};
use near_sdk::borsh::BorshSerialize;
use near_sdk::collections::{LazyOption, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, Gas};

const GAS_FOR_CHECK_STAKE_RESULT: Gas = Gas(5_000_000_000_000);
const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000);

#[near_bindgen]
impl PoolContract {
  #[init]
  pub fn init() -> Self {
    let owner_id = env::signer_account_id();
    Self::new(
      owner_id,
      PoolContractMetadata {
        spec: "elearning-1.0.0".to_string(),
        name: "elearning".to_string(),
        symbol: "EganTeam".to_string(),
        icon: None,
        base_uri: None,
        reference: None,
        reference_hash: None,
      },
    )
  }

  #[init]
  pub fn new(owner_id: AccountId, metadata: PoolContractMetadata) -> Self {
    Self {
      owner_id,
      metadata_pool_contract: LazyOption::new(
        PoolStorageKey::PoolContractMetadata.try_to_vec().unwrap(),
        Some(&metadata),
      ),
      all_pool_id: UnorderedSet::new(PoolStorageKey::AllPoolId.try_to_vec().unwrap()),
      all_mentoring_id: UnorderedSet::new(PoolStorageKey::AllMentoringId.try_to_vec().unwrap()),
    }
  }
}
