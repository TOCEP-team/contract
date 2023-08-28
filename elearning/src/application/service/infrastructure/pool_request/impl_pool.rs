use std::collections::HashMap;

use crate::{
  application::repository::convert_pool_title_to_pool_id,
  models::{
    contract::{ELearningContract, ELearningContractExt},
    pool_request::{
      external::cross_pool,
      pool::{PoolId, PoolMetadata, PoolRequestFeatures, PoolState, GAS_FOR_CHECK_RESULT, GAS_FOR_CROSS_CALL},
    },
    user::{EnumUser, Roles, UserId},
  },
};
use near_sdk::{env, json_types::U128, near_bindgen, AccountId, Balance, PromiseResult};

#[near_bindgen]
/// Implement function for pool request
impl PoolRequestFeatures for ELearningContract {
  /// Update pool address
  #[private]
  fn update_pool_address(&mut self, pool_address: AccountId) {
    assert!(env::signer_account_id() == self.owner_id);
    self.pool_address = pool_address;
  }

  /// create a new pool request
  fn create_pool_request(&mut self, pool_title: String, minimum_stake: U128, maximum_stake: U128) {
    assert!(self.check_registration(&env::signer_account_id()), "You are not a user");
    let pool_id = convert_pool_title_to_pool_id(&pool_title);
    assert!(!self.check_pool_request_exist(&pool_id), "Pool id already exist");
    cross_pool::ext(self.pool_address.to_owned())
      .with_static_gas(GAS_FOR_CROSS_CALL)
      .add_pool_id(pool_id.clone())
      .then(Self::ext(env::current_account_id()).with_static_gas(GAS_FOR_CHECK_RESULT).storage_pool_request(
        pool_id,
        minimum_stake,
        maximum_stake,
      ));
  }

  #[private]
  fn storage_pool_request(&mut self, pool_id: PoolId, minimum_stake: U128, maximum_stake: U128) {
    let result = match env::promise_result(0) {
      PromiseResult::NotReady => env::abort(),
      PromiseResult::Successful(value) => {
        if let Ok(refund) = near_sdk::serde_json::from_slice::<U128>(&value) {
          refund.0
          // If we can't properly parse the value, the original amount is returned.
        } else {
          U128(2).into()
        }
      },
      PromiseResult::Failed => U128(2).into(),
    };
    let minimum_stake: Balance = minimum_stake.into();
    let maximum_stake: Balance = maximum_stake.into();
    if result == 1 {
      let pool_metadata = PoolMetadata {
        create_at: env::block_timestamp_ms(),
        current_stake: 0,
        winner: None,
        maximum_stake,
        minimum_stake,
        description: None,
        owner_id: env::signer_account_id(),
        pool_id: pool_id.clone(),
        pool_state: PoolState::ACTIVE,
        staking_period: 259200000,
        instructors_votes: HashMap::new(),
        total_stake: 0,
        stake_info: HashMap::new(),
        unstake_info: HashMap::new(),
        unstaking_period: 864000000,
      };
      self.all_pool_id.insert(&pool_id);
      self.pool_metadata_by_pool_id.insert(&pool_id, &pool_metadata);
    }
  }

  /// update pool request
  fn update_pool_request(
    &mut self,
    pool_id: PoolId,
    description: Option<String>,
    maximum_stake: Option<U128>,
    minimum_stake: Option<U128>,
  ) {
    assert!(self.pool_metadata_by_pool_id.contains_key(&pool_id), "This this is not exist");
    let mut pool_request = self.pool_metadata_by_pool_id.get(&pool_id).unwrap();
    assert!(pool_request.owner_id == env::signer_account_id(), "You are not pool owner");

    if let Some(a) = description {
      pool_request.description = Some(a)
    };

    if let Some(b) = maximum_stake {
      let b: Balance = b.into();
      pool_request.maximum_stake = b
    };
    if let Some(c) = minimum_stake {
      let c: Balance = c.into();
      pool_request.minimum_stake = c
    };

    self.pool_metadata_by_pool_id.insert(&pool_id, &pool_request);
  }

  /// Funtion check pool exist or not
  fn check_pool_request_exist(&self, pool_id: &PoolId) -> bool {
    self.pool_metadata_by_pool_id.contains_key(pool_id)
  }

  /// Funtion check user is a staker in this pool or not
  fn check_staker(&self, pool_id: &PoolId, user_id: &UserId) -> bool {
    self.pool_metadata_by_pool_id.get(pool_id).unwrap().stake_info.contains_key(user_id)
  }

  /// Get all pool metadata
  // TODO: Error unwrap error.

  /// Get all course
  fn get_all_pool_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PoolMetadata> {
    self
      .all_pool_id
      .iter()
      .skip(start.unwrap_or(0) as usize)
      .take(limit.unwrap_or(20) as usize)
      .map(|x| self.get_pool_metadata_by_pool_id(&x).unwrap())
      .collect()
  }

  fn get_all_pool_id(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PoolId> {
    self.all_pool_id.iter().skip(start.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize).collect()
  }
  //fn get_all_pool_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PoolMetadata> {
  //  let mut all_pool = Vec::new();
  //  for pool_id in self.all_pool_id.iter().skip(start.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize) {
  //    all_pool.push(self.pool_metadata_by_pool_id.get(&pool_id).unwrap());
  //  }
  // all_pool
  //}

  /// Instructor apply pool
  fn apply_pool(&mut self, pool_id: PoolId) {
    let instructor_id = env::signer_account_id();
    assert!(self.get_user_role(&instructor_id) == Roles::Instructor, "You must be a Instructor to apply");
    let mut pool_info = self.pool_metadata_by_pool_id.get(&pool_id).unwrap();
    assert!(!pool_info.instructors_votes.contains_key(&instructor_id), "You already apply");
    pool_info.instructors_votes.insert(instructor_id, 0);
    self.pool_metadata_by_pool_id.insert(&pool_id, &pool_info);
  }

  /// stake vote for instructor
  fn vote_instructor(&mut self, pool_id: PoolId, instructor_id: UserId) {
    let staker_id = env::signer_account_id();
    assert!(self.check_staker(&pool_id, &staker_id), "You are not a staker in this pool");
    let mut pool_info = self.pool_metadata_by_pool_id.get(&pool_id).unwrap();
    assert!(pool_info.stake_info.get(&staker_id).unwrap().voted_for.is_none(), "You already voted");
    // let a = pool_info.stake_info.get_mut(&staker_id).unwrap().voted_for.insert(instructor_id.clone());
    pool_info.stake_info.get_mut(&staker_id).unwrap().voted_for = Some(instructor_id.clone());
    *pool_info.instructors_votes.get_mut(&instructor_id).unwrap() += 1;
    self.pool_metadata_by_pool_id.insert(&pool_id, &pool_info);
  }

  /// Get winner and end stake
  fn make_end_stake_process(&mut self, pool_id: PoolId) {
    assert!(self.check_pool_request_exist(&pool_id), "Poll is not exist");
    let mut pool_info = self.pool_metadata_by_pool_id.get(&pool_id).unwrap();
    assert!(pool_info.owner_id == env::signer_account_id(), "You are not pool owner");

    let mut max_value: Option<u32> = None;
    let mut max_keys: Option<AccountId> = None;

    for (key, value) in pool_info.instructors_votes.iter() {
      match max_value {
        Some(current_max) if *value > current_max => {
          //max_keys.clear();
          //max_keys.push(key.clone());
          max_keys = Some(key.clone());
          max_value = Some(*value);
        },
        //Some(current_max) if *value == current_max => {
        //  max_keys.push(key.clone());
        //},
        None => {
          max_keys = Some(key.clone());
          max_value = Some(*value);
        },
        _ => {},
      }
    }

    let min_consensus_value = (pool_info.stake_info.len() * 2 / 3) as u32;
    if max_keys.is_some() && max_value.unwrap() >= min_consensus_value {
      pool_info.winner = max_keys;
      self.pool_metadata_by_pool_id.insert(&pool_id, &pool_info);
    }
    pool_info.pool_state = PoolState::DEACTIVED;
    self.pool_metadata_by_pool_id.insert(&pool_id, &pool_info);
  }

  /// Get pool metadata by pool id
  fn get_pool_metadata_by_pool_id(&self, pool_id: &PoolId) -> Option<PoolMetadata> {
    if let Some(data) = self.pool_metadata_by_pool_id.get(pool_id) {
      Some(data)
    } else {
      None
    }
  }
}
