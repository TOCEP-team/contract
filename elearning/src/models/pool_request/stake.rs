use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Balance;

use crate::models::user::UserId;

use super::pool::PoolId;

/// This struct represents a stake metadata in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeInfo {
  pub staker_id: UserId,
  pub stake_value: Balance,
  pub stake_at: u64,
  pub voted_for: Option<UserId>,
}

/// Stake Features
pub trait StakeFeatures {
  /// function use to stake
  fn stake_process(&mut self, pool_id: PoolId, amount: U128) -> U128;
}
