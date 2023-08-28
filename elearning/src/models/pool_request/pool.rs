use crate::models::pool_request::{stake::StakeInfo, unstake::UnstakeInfo};
use crate::models::user::UserId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance, Gas};
use std::collections::HashMap;

pub const GAS_FOR_CHECK_RESULT: Gas = Gas(5_000_000_000_000);
pub const GAS_FOR_CROSS_CALL: Gas = Gas(3_000_000_000_000); // 0.00025

/// The `Roles` enum represents the various roles a user can have within the system.
#[derive(Deserialize, BorshDeserialize, BorshSerialize, Serialize, Default, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum PoolState {
  /// The default pool state is idle
  #[default]
  IDLE,
  /// Active state
  ACTIVE,
  /// Deactived state
  DEACTIVED,
  /// Close state
  CLOSED,
}

/// `PoolId` is a type alias for `String`, typically representing a unique identifier for a user in the system.
pub type PoolId = String;

/// This struct represents a pool's metadata in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PoolMetadata {
  /// Unique identifier of the pool.
  pub pool_id: PoolId,

  /// User create the pool
  pub owner_id: UserId,

  /// The time when the pool is initialized..
  pub create_at: u64,

  /// Pool's state. Default is idle
  pub pool_state: PoolState,

  /// The total amount of money deposited into the pool by all users.
  pub total_stake: Balance,

  /// The amount of money currently being deposited into the pool by current users.
  pub current_stake: Balance,

  /// The minimum amount of money required to be deposited into the pool to participate.
  pub minimum_stake: Balance,

  /// The maximum amount of money a user can deposit into the pool.
  pub maximum_stake: Balance,

  /// The required time to stake in the pool before being able to withdraw funds.
  pub staking_period: u64, // Xem lai doan nay

  /// The required time to wait before being able to withdraw funds from the pool after having unstaked.
  pub unstaking_period: u64,

  /// The total number of instructors who want to teach this course.
  pub instructors_votes: HashMap<UserId, u32>,

  /// Instructor win
  pub winner: Option<AccountId>,

  ///  A hashmap that stores information about users who have staked and the stake information.
  pub stake_info: HashMap<UserId, StakeInfo>,

  ///  A hashmap that stores information about users who have unstake and the unstake information.
  pub unstake_info: HashMap<UserId, UnstakeInfo>,

  /// Pool description
  pub description: Option<String>,
}

/// Poolrequest future
pub trait PoolRequestFeatures {
  /// create a new pool request
  fn create_pool_request(&mut self, pool_id: PoolId, maximum_stake: U128, minimum_stake: U128);

  fn get_all_pool_id(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PoolId>;

  fn storage_pool_request(&mut self, pool_id: PoolId, maximum_stake: U128, minimum_stake: U128);

  /// update pool request
  fn update_pool_request(
    &mut self,
    pool_id: PoolId,
    description: Option<String>,
    maximum_stake: Option<U128>,
    minimum_stake: Option<U128>,
  );

  /// Funtion check pool exist or not
  fn check_pool_request_exist(&self, pool_id: &PoolId) -> bool;

  /// Funtion check user is a staker in this pool or not
  fn check_staker(&self, pool_id: &PoolId, user_id: &UserId) -> bool;

  /// Get pool metadata by pool id
  fn get_pool_metadata_by_pool_id(&self, pool_id: &PoolId) -> Option<PoolMetadata>;

  /// Get all pool metadata
  fn get_all_pool_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PoolMetadata>;

  /// Update pool address
  fn update_pool_address(&mut self, pool_address: AccountId);

  /// Instructor apply pool
  fn apply_pool(&mut self, pool_id: PoolId);

  /// Stake vote for instructor
  fn vote_instructor(&mut self, pool_id: PoolId, instructor_id: UserId);

  /// Get winner and end stake
  fn make_end_stake_process(&mut self, pool_id: PoolId);
}
