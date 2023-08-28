use crate::models::{
  contract::{ELearningContract, ELearningContractExt},
  skill::{EnumSkill, SkillId, SkillMetadata},
  user::UserId,
};

use near_sdk::near_bindgen;
use std::collections::HashMap;

#[near_bindgen]
impl EnumSkill for ELearningContract {
  /// Get all skills per user
  fn get_all_skills_per_user(&self, user_id: UserId) -> HashMap<SkillId, u32> {
    assert!(self.check_registration(&user_id), "User is not exists");
    self.user_metadata_by_id.get(&user_id).unwrap().skill
  }

  /// Get skill metadata by skill id
  fn get_all_skill_metadata_by_skill_id(
    &self,
    skill_id: SkillId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<SkillMetadata> {
    let data = if let Some(skill_id_set) = self.skill_metadata_by_skill_id.get(&skill_id) {
      skill_id_set
    } else {
      return vec![];
    };
    data.iter().skip(start.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize).collect()
  }
}
