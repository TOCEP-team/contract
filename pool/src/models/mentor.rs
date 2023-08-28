use near_sdk::{json_types::U128, AccountId, PromiseOrValue};

pub type MentoringId = String;
pub type UserId = AccountId;

pub trait MentorFeatures {
  fn add_mentoring_id(&mut self, mentoring_id: MentoringId) -> U128;

  fn mentoring_claim(&mut self, receiver: UserId, amount: U128) -> U128;

  fn get_all_mentoring_id(&self, start: Option<u32>, limit: Option<u32>) -> Vec<MentoringId>;

  fn buy_mentoring(&mut self, mentoring_id: MentoringId) -> PromiseOrValue<U128>;

  fn check_mentoring_existence(&self, mentoring_id: &MentoringId) -> bool;

  fn mentoring_withdraw(&mut self, amount: U128) -> U128;

  fn transfer_last_lession(&mut self, receiver: UserId, price_per_lession: U128, remaining_amout: U128) -> U128;
}
