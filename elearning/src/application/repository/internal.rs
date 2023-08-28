use near_sdk::near_bindgen;

use crate::models::{
  contract::{ELearningContract, ELearningContractExt},
  course::{CourseFeatures, CourseId, CourseMetadata, EnumCourse, Unit},
  user::UserId,
};

#[near_bindgen]
impl ELearningContract {
  pub(crate) fn internal_check_course_exits(&self, course_id: &CourseId) -> bool {
    let course = self.get_course_metadata_by_course_id(course_id.to_string());
    course.is_some()
  }

  pub(crate) fn internal_check_user_has_register(&self, user_id: &UserId) -> bool {
    self.user_metadata_by_id.contains_key(user_id)
  }

  pub(crate) fn internal_check_subscriber_has_course(&self, user_id: &UserId, course_id: &CourseId) -> bool {
    let course = self.get_course_metadata_by_course_id(course_id.to_string()).unwrap();
    !course.students_studying_map.contains_key(user_id) && !course.students_completed.contains_key(user_id)
  }

  pub(crate) fn internal_check_instructor_member(&self, instructor_id: &UserId, course: &CourseMetadata) -> bool {
    course.instructor_id.contains_key(instructor_id)
  }

  pub(crate) fn internal_check_consensus_member(&self, instructor_id: &UserId, course: &CourseMetadata) -> bool {
    !course.consensus.contains_key(instructor_id)
  }

  pub(crate) fn internal_check_enough_unit(
    &self,
    instructor_id: &UserId,
    course: &CourseMetadata,
    amount: &Unit,
  ) -> bool {
    course.instructor_id.get(instructor_id).unwrap() >= amount
  }

  pub(crate) fn internal_check_instructor_exits(&self, course_id: &CourseId, new_instructor: &UserId) -> bool {
    !self.get_course_metadata_by_course_id(course_id.clone()).unwrap().instructor_id.contains_key(new_instructor)
  }
}
