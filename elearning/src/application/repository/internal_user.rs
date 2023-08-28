use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet};

use crate::models::{
  contract::{ELearningContract, ELearningStorageKey},
  user::UserId,
};

impl ELearningContract {
  /// Check user exist or not
  pub(crate) fn check_registration(&self, user_id: &UserId) -> bool {
    self.user_metadata_by_id.contains_key(user_id)
  }
}
