use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  collections::{LazyOption, UnorderedSet},
  json_types::Base64VecU8,
  near_bindgen,
  serde::{Deserialize, Serialize},
  AccountId, PanicOnDefault,
};

use crate::models::pool::PoolId;

use super::mentor::MentoringId;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolContractMetadata {
  /// Specification associated with the pool contract.
  pub spec: String,

  /// Name of the pool contract.
  pub name: String,

  /// Symbol associated with the pool contract.
  pub symbol: String,

  /// Optional icon for the pool contract.
  pub icon: Option<String>,

  /// Optional base URI for the pool contract.
  pub base_uri: Option<String>,

  /// Optional reference string for the pool contract.
  pub reference: Option<String>,

  /// Optional hash of the reference, encoded in base64.
  pub reference_hash: Option<Base64VecU8>,
}

/// The `ELearningContract` struct represents an pool contract in the system.
#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct PoolContract {
  /// Account ID of the owner of the contract.
  pub owner_id: AccountId,

  /// Metadata associated with the pool contract.
  pub metadata_pool_contract: LazyOption<PoolContractMetadata>,

  /// Storage all user_id of subscriber users -> For count all of users in the system
  pub all_pool_id: UnorderedSet<PoolId>,

  /// Storage all mentoring
  pub all_mentoring_id: UnorderedSet<MentoringId>,
}

/// The `ELearningContractStorageKey` enum represents keys for different persistent collections in the contract storage.
#[derive(BorshSerialize)]
pub enum PoolStorageKey {
  PoolContractMetadata,
  AllPoolId,
  AllMentoringId,
}
