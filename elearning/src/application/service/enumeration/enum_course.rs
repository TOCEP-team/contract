use crate::models::{
  contract::{ELearningContract, ELearningContractExt},
  course::{CourseId, CourseMetadata, EnumCourse},
  user::{Roles, UserId},
};
use near_sdk::near_bindgen;

#[near_bindgen]
impl EnumCourse for ELearningContract {
  fn get_all_courses_per_instructor(
    &self,
    instructor_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CourseMetadata> {
    assert!(
      self.user_metadata_by_id.get(&instructor_id).unwrap().metadata.role == Roles::Instructor,
      "This user are not Instructor. Please change the Instructor id"
    );
    let courses_per_owner_set = self.courses_per_instructor.get(&instructor_id);
    let courses = if let Some(courses_per_owner) = courses_per_owner_set {
      courses_per_owner
    } else {
      return vec![];
    };

    courses
      .iter()
      .skip(start.unwrap_or(0) as usize)
      .take(limit.unwrap_or(10) as usize)
      .map(|value| self.get_course_metadata_by_course_id(value).unwrap())
      .collect()
  }

  /// Get all the course per user have. Current and complete course
  fn get_purchase_course_by_user_id(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CourseMetadata> {
    // Check the user is exists or not
    assert!(self.check_registration(&user_id), "User does not exist. Please change the user id");
    let mut all_courses = Vec::new();

    // Get course id per user
    let user_course = self.user_metadata_by_id.get(&user_id).unwrap().courses;

    // Get course metadata
    for course in user_course.iter().skip(start.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize) {
      all_courses.push(self.get_course_metadata_by_course_id(course.clone()).unwrap())
    }

    // return
    all_courses
  }

  /// Get all course
  fn get_all_course_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<CourseMetadata> {
    self
      .all_course_id
      .iter()
      .skip(start.unwrap_or(0) as usize)
      .take(limit.unwrap_or(20) as usize)
      .map(|x| self.course_metadata_by_id.get(&x).unwrap())
      .collect()
  }

  fn get_all_course_id(&self, start: Option<u32>, limit: Option<u32>) -> Vec<CourseId> {
    let mut all_courses_id = Vec::new();
    for course_id in self.all_course_id.iter().skip(start.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize) {
      all_courses_id.push(course_id.clone())
    }
    all_courses_id
  }

  /// Get all the course per user have. Current and complete course
  fn get_course_metadata_by_course_id(&self, course_id: CourseId) -> Option<CourseMetadata> {
    /* uncomment this code when use event*/
    // assert!(self.course_metadata_by_id.contains_key(&course_id), "This course is not exist");
    if let Some(course) = self.course_metadata_by_id.get(&course_id) {
      Some(course)
    } else {
      None
    }
  }
}
