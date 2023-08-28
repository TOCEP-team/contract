use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet};

use crate::models::{
  combo::{ComboId, ComboState},
  contract::{ELearningContract, ELearningStorageKey},
  course::CourseId,
  user::UserId,
};

impl ELearningContract {
  pub(crate) fn check_combo_existence(&self, combo_id: &ComboId) -> bool {
    self.combo_metadata_by_combo_id.contains_key(combo_id)
  }

  pub(crate) fn check_course_in_combo(&self, combo_id: &ComboId, course_id: &CourseId) -> bool {
    let courses = self.combo_metadata_by_combo_id.get(combo_id).unwrap().courses;

    for course_info in courses {
      if *course_id == course_info.course_id {
        return true;
      }
    }
    false
  }

  pub(crate) fn check_combo_state(&self, combo_id: &ComboId) -> ComboState {
    self.combo_metadata_by_combo_id.get(&combo_id).unwrap().combo_state
  }
}
