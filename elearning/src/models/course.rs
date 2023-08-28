use std::collections::HashMap;

use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  json_types::U128,
  serde::{Deserialize, Serialize},
  AccountId, Balance,
};

use super::{skill::SkillId, user::UserId};

/// `CourseId` is a type alias for `String`, typically representing a unique identifier for a course in the system.
pub type Unit = u32;
pub type CourseId = String;

/// The `CourseMetadata` struct represents metadata for a course in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CourseMetadata {
  /// Unique identifier for the course, of type `CourseId`.
  pub course_id: CourseId,

  /// Skill user can own after finish course
  pub skills: Vec<SkillId>,

  /// Name of the course.
  pub title: String,

  /// Detailed description of the course.
  pub description: Option<String>,

  /// Thumbnail of the course
  pub media: Option<String>,

  /// Instructor's account ID.
  pub instructor_id: HashMap<UserId, Unit>,

  /// Date when the course was created, represented as a timestamp.
  pub created_at: u64,

  /// Date when the course update
  pub update_at: u64,

  /// Price of this course, of type `U128`.
  pub price: Balance,

  /// Map student with their hash
  pub students_studying_map: HashMap<AccountId, String>,

  /// Number of students who have completed this course. And time stamp
  pub students_completed: HashMap<AccountId, u16>,

  /// Average of all the ratings this course has received.
  pub rating: u8,

  /// Number of ratings this course has received.
  pub rating_count: u32,

  /// The Content of this course
  pub content: String,

  /// Consensus for add new instructor
  pub consensus: HashMap<AccountId, Unit>,
}

pub trait CourseFeatures {
  fn create_course(
    &mut self,
    title: String,
    description: Option<String>,
    media: Option<String>,
    price: U128,
    skills: Vec<SkillId>,
  ) -> CourseMetadata;

  /// Make user completed the course
  fn make_user_finish_course(
    &mut self,
    course_id: CourseId,
    media: String,
    rating: u16,
    hash_collection: String,
    //skills: Vec<WrapSkill>,
  );

  /// Update course
  fn update_course(
    &mut self,
    course_id: CourseId,
    content: Option<String>,
    description: Option<String>,
    media: Option<String>,
    price: Option<U128>,
    title: Option<String>,
  );
}

pub trait EnumCourse {
  /// Get all course
  fn get_all_course_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<CourseMetadata>;

  fn get_all_course_id(&self, start: Option<u32>, limit: Option<u32>) -> Vec<CourseId>;

  /// Get all the course per user have. Current and complete course
  fn get_purchase_course_by_user_id(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CourseMetadata>;

  fn get_course_metadata_by_course_id(&self, course_id: CourseId) -> Option<CourseMetadata>;

  fn get_all_courses_per_instructor(
    &self,
    instructor_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CourseMetadata>;
}
