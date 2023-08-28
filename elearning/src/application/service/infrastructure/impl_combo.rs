#![allow(clippy::too_many_arguments)]

use crate::{
  application::repository::convert_combo_title_to_combo_id,
  models::{
    combo::{ComboFeatures, ComboId, ComboMetadata, ComboState, WrapCombo},
    contract::{ELearningContract, ELearningContractExt},
    course::CourseId,
  },
};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
impl ComboFeatures for ELearningContract {
  fn create_combo(
    &mut self,
    combo_title: String,
    courses: Vec<WrapCombo>,
    description: Option<String>,
    media: Option<String>,
  ) {
    assert!(self.check_registration(&env::signer_account_id()), "You are not a user");
    for course_info in courses.clone() {
      assert!(
        self.check_course_existence(&course_info.course_id),
        "Please check your course id {}",
        course_info.course_id
      );
    }

    let combo_id = convert_combo_title_to_combo_id(&combo_title);
    assert!(!self.check_combo_existence(&combo_id), "Plase change your title");
    let new_combo = ComboMetadata {
      combo_id: combo_id.clone(),
      combo_state: ComboState::DEACTIVED,
      enable_course: vec![],
      courses,
      description,
      media,
    };
    self.all_combo_id.insert(&combo_id);
    self.combo_metadata_by_combo_id.insert(&combo_id, &new_combo);
  }

  fn enable_course(&mut self, combo_id: ComboId, course_id: CourseId) {
    assert!(self.check_registration(&env::signer_account_id()), "You are not a user");
    assert!(self.check_combo_existence(&combo_id), "Combo is not exsist");
    assert!(self.check_course_in_combo(&combo_id, &course_id), "This course is not in this combo");
    assert!(self.check_course_owner(&course_id, &env::signer_account_id()), "You are not course owner");

    let mut combo_info = self.combo_metadata_by_combo_id.get(&combo_id).unwrap();
    assert!(!combo_info.enable_course.contains(&course_id), "You already enable this course");

    if combo_info.courses.len() - combo_info.enable_course.len() == 1 {
      combo_info.combo_state = ComboState::ACTIVE
    };
    combo_info.enable_course.push(course_id);
    self.combo_metadata_by_combo_id.insert(&combo_id, &combo_info);
  }
}
