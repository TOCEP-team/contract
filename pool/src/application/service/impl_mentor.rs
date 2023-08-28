use near_sdk::{env, json_types::U128, near_bindgen, Promise, PromiseOrValue};

use crate::{
  models::{
    external::elearning_contract,
    mentor::{MentorFeatures, MentoringId, UserId},
    poolcontract::{PoolContract, PoolContractExt},
  },
  GAS_FOR_CHECK_STAKE_RESULT, GAS_FOR_CROSS_CALL,
};

#[near_bindgen]
impl MentorFeatures for PoolContract {
  fn add_mentoring_id(&mut self, mentoring_id: MentoringId) -> U128 {
    assert!(self.owner_id == env::predecessor_account_id(), "You don't have permision");
    // owner = pool.vbi-academy == env::predeecessor_account_id()
    self.all_mentoring_id.insert(&mentoring_id);
    U128(1)
  }

  fn check_mentoring_existence(&self, mentoring_id: &MentoringId) -> bool {
    self.all_mentoring_id.contains(mentoring_id)
  }

  fn mentoring_claim(&mut self, receiver: UserId, amount: U128) -> U128 {
    assert!(self.owner_id == env::predecessor_account_id(), "You don't have permision");
    self.internal_transfer(receiver, amount);
    U128(1)
  }

  fn mentoring_withdraw(&mut self, amount: U128) -> U128 {
    assert!(self.owner_id == env::predecessor_account_id(), "You don't have permision");
    self.internal_refund(amount);
    U128(1)
  }

  #[payable]
  fn buy_mentoring(&mut self, mentoring_id: MentoringId) -> PromiseOrValue<U128> {
    assert!(env::attached_deposit() >= 1, "This function require an amount!");
    assert!(self.check_mentoring_existence(&mentoring_id), "This mentoring is not exist");

    let amount = env::attached_deposit();

    Promise::new(env::current_account_id()).transfer(amount);
    elearning_contract::ext(self.owner_id.clone())
      .with_static_gas(GAS_FOR_CROSS_CALL)
      .buy_mentoring_process(mentoring_id, amount.into())
      .then(
        Self::ext(env::current_account_id()).with_static_gas(GAS_FOR_CHECK_STAKE_RESULT).check_result(amount.into()),
      )
      .into()
  }

  fn get_all_mentoring_id(&self, start: Option<u32>, limit: Option<u32>) -> Vec<MentoringId> {
    self.all_mentoring_id.iter().skip(start.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize).collect()
  }

  /// Cross call for last lession
  fn transfer_last_lession(&mut self, receiver: UserId, price_per_lession: U128, remaining_amout: U128) -> U128 {
    assert!(self.owner_id == env::predecessor_account_id(), "You don't have permision");
    self.internal_transfer(receiver, price_per_lession);
    if remaining_amout != U128(0) {
      self.internal_refund(remaining_amout)
    }
    U128(1)
  }
}
