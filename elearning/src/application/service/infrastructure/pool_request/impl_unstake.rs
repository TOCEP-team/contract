use crate::models::{
  contract::{ELearningContract, ELearningContractExt},
  pool_request::{
    external::cross_pool,
    pool::{PoolId, PoolRequestFeatures, PoolState, GAS_FOR_CHECK_RESULT, GAS_FOR_CROSS_CALL},
    unstake::{UnstakeFeatures, UnstakeInfo},
  },
  user::UserId,
};
use near_sdk::{env, json_types::U128, near_bindgen, Balance, PromiseResult};

#[near_bindgen]
impl UnstakeFeatures for ELearningContract {
  /// Check if the staker is eligible for withdrawal
  #[private]
  fn check_withdrawal_availability(&self, pool_id: &PoolId, staker_id: &UserId) -> bool {
    if !self.check_staker(pool_id, staker_id) {
      return false;
    };

    /*
      let pool_info = self.pool_metadata_by_pool_id.get(pool_id).unwrap();
      let stake_info = pool_info.stake_info.get(staker_id).unwrap();
      let stake_time = env::block_timestamp_ms() - stake_info.stake_at;

      if stake_time < pool_info.staking_period {
        return false;
    };
      */
    true
  }
  /// Unstake
  fn unstake(&mut self, pool_id: PoolId) -> U128 {
    let unstaker_id = env::signer_account_id();
    assert!(self.check_registration(&unstaker_id), "You are not a user");
    assert!(self.check_pool_request_exist(&pool_id), "This pool is not exist");
    assert!(
      self.pool_metadata_by_pool_id.get(&pool_id).unwrap().pool_state == PoolState::ACTIVE,
      "Pool is deactive now"
    );
    assert!(self.check_withdrawal_availability(&pool_id, &unstaker_id), "You don't have access");

    let amount = self.pool_metadata_by_pool_id.get(&pool_id).unwrap().stake_info.get(&unstaker_id).unwrap().stake_value;
    cross_pool::ext(self.pool_address.to_owned()).with_static_gas(GAS_FOR_CROSS_CALL).refund(amount.into()).then(
      Self::ext(env::current_account_id()).with_static_gas(GAS_FOR_CHECK_RESULT).remove_stake_info(
        &pool_id,
        &unstaker_id,
        amount.into(),
      ),
    );
    U128(1)
  }

  #[private]
  fn remove_stake_info(&mut self, pool_id: &PoolId, staker_id: &UserId, amount: U128) {
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

    if result == 1 {
      let mut pool_info = self.pool_metadata_by_pool_id.get(pool_id).unwrap();
      let amount: Balance = amount.into();
      let unstake_info =
        UnstakeInfo { staker_id: staker_id.clone(), unstake_value: amount, unstake_at: env::block_timestamp_ms() };

      pool_info.unstake_info.insert(staker_id.clone(), unstake_info);

      // If staker has voted -> remove vote

      let subtract_one_vote = pool_info.stake_info.get(staker_id).unwrap().voted_for.clone();
      if subtract_one_vote.is_some() {
        *pool_info.instructors_votes.get_mut(&subtract_one_vote.unwrap()).unwrap() -= 1;
        self.pool_metadata_by_pool_id.insert(pool_id, &pool_info);
      }
      pool_info.stake_info.remove(staker_id);

      pool_info.current_stake -= amount;

      self.pool_metadata_by_pool_id.insert(pool_id, &pool_info);
    }
  }
}
