use near_sdk::{env, json_types::U128, near_bindgen, Promise, PromiseOrValue, PromiseResult};

use crate::{
  models::{
    external::elearning_contract,
    pool::{PoolFeatures, PoolId},
    poolcontract::{PoolContract, PoolContractExt},
  },
  GAS_FOR_CHECK_STAKE_RESULT, GAS_FOR_CROSS_CALL,
};

#[near_bindgen]
impl PoolFeatures for PoolContract {
  /// Add pool_id when user create a new pool request. Only call by ELearning contract
  fn add_pool_id(&mut self, pool_id: PoolId) -> U128 {
    assert!(self.owner_id == env::predecessor_account_id(), "You don't have permision");
    // owner = pool.vbi-academy == env::predeecessor_account_id()
    self.all_pool_id.insert(&pool_id);
    U128(1)
  }

  /// Check pool exist or not
  fn check_pool_existence(&self, pool_id: &PoolId) -> bool {
    self.all_pool_id.contains(pool_id)
  }

  /// Stake
  #[payable]
  fn stake(&mut self, pool_id: PoolId) -> PromiseOrValue<U128> {
    // TODO: Fix message
    assert!(env::attached_deposit() >= 1, "This function require an amount!");
    assert!(self.check_pool_existence(&pool_id), "This pool is not exist");

    let amount = env::attached_deposit();

    Promise::new(env::current_account_id()).transfer(amount);
    elearning_contract::ext(self.owner_id.clone())
      .with_static_gas(GAS_FOR_CROSS_CALL)
      .stake_process(pool_id, amount.into())
      .then(
        Self::ext(env::current_account_id()).with_static_gas(GAS_FOR_CHECK_STAKE_RESULT).check_result(amount.into()),
      )
      .into()
  }

  /// Get all pool id
  fn get_all_pool_id(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PoolId> {
    self.all_pool_id.iter().skip(start.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize).collect()
  }

  /// Refund
  fn refund(&mut self, amount: U128) -> U128 {
    assert!(self.owner_id == env::predecessor_account_id(), "You don't have permision");
    self.internal_refund(amount);
    U128(1)
  }
}

#[near_bindgen]
impl PoolContract {
  #[private]
  pub fn check_result(&mut self, amount: U128) -> U128 {
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

    if result != 1 {
      self.internal_refund(amount);
      U128(2)
    } else {
      U128(1)
    }
  }
}
