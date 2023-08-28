use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet};

use crate::models::{
  contract::{ELearningContract, ELearningStorageKey},
  course::CourseId,
  user::UserId,
};

use super::hash_account_id;

impl ELearningContract {
  pub(crate) fn internal_add_course_to_instructor(&mut self, account_id: &UserId, course_id: &CourseId) {
    let mut courses_set = self.courses_per_instructor.get(account_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ELearningStorageKey::CoursesPerInstructorInner { instructor_id_hash: hash_account_id(account_id) }
          .try_to_vec()
          .unwrap(),
      )
    });

    //we insert the token ID into the set
    courses_set.insert(course_id);

    self.courses_per_instructor.insert(account_id, &courses_set);
  }

  pub(crate) fn check_course_completed(&self, course_id: CourseId, user_id: UserId) -> bool {
    // Check course exist or not. User is student or not
    assert!(self.course_metadata_by_id.contains_key(&course_id), "This course is not exist");
    let course_set = self.course_metadata_by_id.get(&course_id).unwrap();
    assert!(course_set.students_studying_map.contains_key(&user_id), "This user is not a student of this course");
    // Return
    course_set.students_completed.contains_key(&user_id)
  }

  /// Check course exist or not
  pub(crate) fn check_course_existence(&self, course_id: &CourseId) -> bool {
    self.all_course_id.contains(course_id)
  }

  /// Check coure owner
  pub(crate) fn check_course_owner(&self, course_id: &CourseId, course_owner_id: &UserId) -> bool {
    self.check_course_existence(course_id);
    self.course_metadata_by_id.get(course_id).unwrap().instructor_id.contains_key(course_owner_id)
  }
}
