use crate::models::contract::{ELearningContract, ELearningContractExt};
use crate::models::user::{EnumUser, JsonUser, Roles, UserId};
use near_sdk::near_bindgen;

#[near_bindgen]
/// Implement function for user
impl EnumUser for ELearningContract {
  /// Get user information. From 'index' to 'index + limit'
  fn get_all_user_metadata(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<JsonUser> {
    let mut all_user = Vec::new();
    for user_id in
      self.subscriber_users.iter().skip(from_index.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize)
    {
      all_user.push(self.user_metadata_by_id.get(&user_id).unwrap());
    }
    all_user
  }

  /// Get Instructor information. From 'index' to 'index + limit'
  fn get_all_instructor_metadata(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<JsonUser> {
    let mut all_instructor = Vec::new();
    for user_id in self.intructor_users.iter().skip(from_index.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize)
    {
      all_instructor.push(self.user_metadata_by_id.get(&user_id).unwrap());
    }
    all_instructor
  }

  /// Check user role
  fn get_user_role(&self, user_id: &UserId) -> Roles {
    // Chek user exist or not
    assert!(self.check_registration(user_id), "User does not exist");
    // Return
    self.user_metadata_by_id.get(user_id).unwrap().metadata.role
  }

  /// Get information of user
  fn get_user_metadata_by_user_id(&self, user_id: &UserId) -> Option<JsonUser> {
    if let Some(metadata) = self.user_metadata_by_id.get(user_id) {
      Some(metadata)
    } else {
      None
    }
  }
}
