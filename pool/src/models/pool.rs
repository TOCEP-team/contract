use near_sdk::{json_types::U128, PromiseOrValue};

pub type PoolId = String;

pub trait PoolFeatures {
  /// Check pool exist or not
  fn check_pool_existence(&self, pool_id: &PoolId) -> bool;

  /// Get all pool id
  fn get_all_pool_id(&self, start: Option<u32>, limit: Option<u32>) -> Vec<PoolId>;

  /// Add pool_id when user create a new pool request. Only call by ELearning contract
  fn add_pool_id(&mut self, pool_id: PoolId) -> U128;

  /// Stake in the pool
  fn stake(&mut self, pool_id: PoolId) -> PromiseOrValue<U128>;

  /// Refund
  fn refund(&mut self, amount: U128) -> U128;
}
