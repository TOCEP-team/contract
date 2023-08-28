use near_sdk::{env, json_types::U128, near_bindgen, Balance, Promise};

use crate::models::{
  mentor::UserId,
  poolcontract::{PoolContract, PoolContractExt},
};

#[near_bindgen]
impl PoolContract {
  #[private]
  pub(crate) fn internal_refund(&mut self, unused_amount: U128) {
    let unused_amount: Balance = unused_amount.into();
    Promise::new(env::signer_account_id()).transfer(unused_amount);
  }

  #[private]
  pub(crate) fn internal_transfer(&mut self, receiver: UserId, unused_amount: U128) {
    let unused_amount: Balance = unused_amount.into();
    Promise::new(receiver).transfer(unused_amount);
  }
}
