use crate::models::{
  contract::{ELearningContract, ELearningContractExt},
  course::{CourseId, CourseMetadata, EnumCourse, Unit},
  user::UserId,
};
use near_sdk::{env, near_bindgen};

pub trait Collab {
  fn add_instructor(&mut self, course_id: CourseId, new_instructor: UserId);
  fn check_consensus(&self, course_id: CourseId) -> bool;
  fn agree_consensus(&mut self, course_id: CourseId, amount: Unit);
  fn transfer_unit(&mut self, course_id: CourseId, instructor: UserId, amount: Unit) -> CourseMetadata;
  fn remove_instructor(&mut self, course_id: CourseId, instructor_id: UserId);
  fn update_consensus(&mut self, course_id: CourseId, amount: Unit);
}

#[near_bindgen]
impl Collab for ELearningContract {
  #[private]
  fn check_consensus(&self, course_id: CourseId) -> bool {
    let course = self.get_course_metadata_by_course_id(course_id).unwrap();
    let round = (course.instructor_id.len() as f32 / 3.0 * 2.0).ceil() as usize;
    if course.instructor_id.len() == 1 {
      true
    } else {
      course.consensus.len() >= round
    }
  }

  // check again authority
  fn agree_consensus(&mut self, course_id: CourseId, amount: Unit) {
    let mut course = self.get_course_metadata_by_course_id(course_id.clone()).unwrap();
    assert!(
      self.internal_check_instructor_member(&env::signer_account_id(), &course),
      "You aren't members of this courses"
    );

    assert!(self.internal_check_consensus_member(&env::signer_account_id(), &course), "You already consensus");
    assert!(self.internal_check_enough_unit(&env::signer_account_id(), &course, &amount), "Not enough unit");

    course.consensus.insert(env::signer_account_id(), amount);
    self.course_metadata_by_id.insert(&course_id, &course);
  }

  // TODO: calculate for the pricae
  fn add_instructor(&mut self, course_id: CourseId, new_instructor: UserId) {
    assert!(self.check_consensus(course_id.clone()), "You aren't have authority");
    assert!(self.internal_check_instructor_exits(&course_id, &new_instructor), "The instructor already exists");
    assert!(self.course_metadata_by_id.contains_key(&course_id), "The course doesn't exists");
    let mut course = self.get_course_metadata_by_course_id(course_id.clone()).unwrap();

    // Insert new instrutor with each unit from old instructors
    let sum: u32 = course.consensus.values().sum();
    course.instructor_id.insert(new_instructor, sum);

    for i in course.clone().consensus.keys() {
      let unit = *course.instructor_id.get(i).unwrap();
      course.instructor_id.insert(i.clone(), unit - course.consensus.get(i).unwrap());
      course.consensus.remove(i);
    }

    // update metadata
    self.course_metadata_by_id.insert(&course_id, &course);
  }

  // Transfer amount
  fn transfer_unit(&mut self, course_id: CourseId, instructor_id: UserId, amount: Unit) -> CourseMetadata {
    let mut course = self.get_course_metadata_by_course_id(course_id.clone()).unwrap();
    assert!(self.internal_check_instructor_member(&instructor_id, &course), "You aren't members of this courses");
    assert!(self.internal_check_course_exits(&course_id), "The course doesn't exists");
    assert!(amount <= *course.instructor_id.get(&env::signer_account_id()).unwrap(), "Not enough amount");

    course
      .instructor_id
      .insert(env::signer_account_id(), course.instructor_id.get(&env::signer_account_id()).unwrap() - amount);
    course.instructor_id.insert(instructor_id.clone(), course.instructor_id.get(&instructor_id).unwrap() + amount);

    self.course_metadata_by_id.insert(&course_id, &course);
    course
  }

  fn update_consensus(&mut self, course_id: CourseId, amount: Unit) {
    let mut course = self.get_course_metadata_by_course_id(course_id.clone()).unwrap();
    assert!(
      self.internal_check_instructor_member(&env::signer_account_id(), &course),
      "You aren't members of this courses"
    );

    assert!(!self.internal_check_consensus_member(&env::signer_account_id(), &course), "You hasn't consensus");

    course.consensus.insert(env::signer_account_id(), amount);
    self.course_metadata_by_id.insert(&course_id, &course);
  }

  fn remove_instructor(&mut self, course_id: CourseId, instructor_id: UserId) {
    assert!(self.course_metadata_by_id.contains_key(&course_id), "The course doesn't exists");
    assert!(self.check_consensus(course_id.clone()), "You aren't have authority");
    assert!(!self.internal_check_instructor_exits(&course_id, &instructor_id), "Instructor not exists");
    let mut course = self.get_course_metadata_by_course_id(course_id.clone()).unwrap();
    let unit = course.instructor_id.get(&instructor_id).unwrap();
    assert_eq!(*unit, course.consensus.values().sum::<u32>(), "Unit not equal with sum consensus");

    for i in course.clone().consensus.keys() {
      let unit = *course.instructor_id.get(i).unwrap();
      course.instructor_id.insert(i.clone(), unit + course.consensus.get(i).unwrap());
      course.consensus.remove(i);
    }

    course.instructor_id.remove(&instructor_id);
    self.course_metadata_by_id.insert(&course_id, &course);
  }
}
