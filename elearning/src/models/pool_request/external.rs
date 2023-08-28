use near_sdk::{ext_contract, json_types::U128, PromiseOrValue};

use crate::models::{mentor::MentoringId, user::UserId};

use super::pool::PoolId;

/// Cross call pool contract and storage pool id
#[ext_contract(cross_pool)]
pub trait CrossCall {
  /// Cross call pool contract and storage pool id
  fn add_pool_id(&mut self, pool_id: PoolId) -> PromiseOrValue<U128>;

  /// Cross call pool contract and refund
  fn refund(&mut self, amount: U128) -> PromiseOrValue<U128>;

  /// Cross call pool contract add storage mentoring id
  fn add_mentoring_id(&mut self, mentoring_id: MentoringId) -> PromiseOrValue<U128>;

  /// Cross call pool contract and tranfer for mentoring owner
  fn mentoring_claim(&mut self, receiver: UserId, amount: U128) -> PromiseOrValue<U128>;

  /// Cross call pool contract and tranfer for study
  fn mentoring_withdraw(&mut self, amount: U128) -> PromiseOrValue<U128>;

  /// Cross call for last lession
  fn transfer_last_lession(
    &mut self,
    receiver: UserId,
    price_per_lession: U128,
    remaining_amout: U128,
  ) -> PromiseOrValue<U128>;
}
