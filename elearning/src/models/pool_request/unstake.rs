use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Balance;

use crate::models::{pool_request::pool::PoolId, user::UserId};

/// This struct represents a unstake metadata in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UnstakeInfo {
  pub staker_id: UserId,
  pub unstake_value: Balance,
  pub unstake_at: u64,
}

/// Unstake Features
pub trait UnstakeFeatures {
  /// Check if the staker is eligible for withdrawal
  fn check_withdrawal_availability(&self, pool_id: &PoolId, staker_id: &UserId) -> bool;

  /// Unstake
  fn unstake(&mut self, pool_id: PoolId) -> U128;

  fn remove_stake_info(&mut self, pool_id: &PoolId, staker_id: &UserId, amount: U128);
}
