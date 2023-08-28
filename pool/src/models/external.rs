use near_sdk::{self, ext_contract, json_types::U128, PromiseOrValue};

use super::{mentor::MentoringId, pool::PoolId};

#[ext_contract(elearning_contract)]
pub trait CrossCall {
  /// Cross call in Stake
  fn stake_process(&mut self, pool_id: PoolId, amount: U128) -> PromiseOrValue<U128>;

  /// Unstake
  fn unstake_process(&mut self, pool_id: PoolId, amount: U128) -> PromiseOrValue<U128>;

  /// Buy mentoring
  fn buy_mentoring_process(&mut self, mentoring_id: MentoringId, amount: U128) -> PromiseOrValue<U128>;
}
