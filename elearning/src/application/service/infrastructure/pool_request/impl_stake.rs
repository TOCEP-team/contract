use crate::models::{
  contract::{ELearningContract, ELearningContractExt},
  pool_request::{
    pool::{PoolId, PoolRequestFeatures, PoolState},
    stake::{StakeFeatures, StakeInfo},
  },
};
use near_sdk::{env, json_types::U128, near_bindgen, Balance};

#[near_bindgen]
impl StakeFeatures for ELearningContract {
  /// function use to stake
  fn stake_process(&mut self, pool_id: PoolId, amount: U128) -> U128 {
    assert!(self.pool_address == env::predecessor_account_id(), "You don't have permision");
    let staker = env::signer_account_id();
    let value: Balance = amount.into();

    if self.check_registration(&staker)
      && self.check_pool_request_exist(&pool_id)
      && self.pool_metadata_by_pool_id.get(&pool_id).unwrap().pool_state != PoolState::CLOSED
    {
      let mut pool_info = self.pool_metadata_by_pool_id.get(&pool_id).unwrap();

      if !pool_info.stake_info.contains_key(&staker) {
        let stake_info = StakeInfo {
          staker_id: staker.clone(),
          stake_value: value,
          stake_at: env::block_timestamp_ms(),
          voted_for: None,
        };
        pool_info.stake_info.insert(staker, stake_info);
        pool_info.current_stake += value;
        pool_info.total_stake += value;
        self.pool_metadata_by_pool_id.insert(&pool_id, &pool_info);
      } else {
        pool_info.stake_info.get_mut(&staker).unwrap().stake_value += value;
        pool_info.current_stake += value;
        pool_info.total_stake += value;
        self.pool_metadata_by_pool_id.insert(&pool_id, &pool_info);
      }
      U128(1)
    } else {
      U128(2)
    }
  }
}
