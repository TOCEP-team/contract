#![allow(clippy::too_many_arguments)]

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use std::collections::HashMap;

use super::certificate::CertificateId;
use super::course::CourseId;
use super::skill::SkillId;

/// The `Roles` enum represents the various roles a user can have within the system.
#[derive(Deserialize, BorshDeserialize, BorshSerialize, Serialize, Default, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Roles {
  /// The default role. Subscribers typically have access to consume content.
  #[default]
  Subscriber,
  /// Instructors have the ability to create and manage content, such as courses.
  Instructor,
  /// Manager have the abilities belong to the system's partner
  Manager,
  /// Admins have administrative privileges, typically including the ability to manage users and system settings.
  Admin,
}

/// `UserId` is a type alias for `AccountId`, typically representing a unique identifier for a user in the system.
pub type UserId = AccountId;

/// This struct represents a user's metadata in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserMetadata {
  /// Unique identifier of the user.
  pub user_id: UserId,

  /// Nickname chosen by the user.
  pub nickname: String,

  /// User's role within the system. Default is Subscriber
  pub role: Roles,

  /// User's total credits.
  pub total_credit: u32,

  /// Number of students associated with this user.
  pub students: u32,

  /// User's first name, if provided.
  pub first_name: Option<String>,

  /// User's last name, if provided.
  pub last_name: Option<String>,

  /// Short biographical note about the user, if provided.
  pub bio: Option<String>,

  /// URL or identifier of the user's avatar image, if provided.
  pub avatar: Option<String>,

  /// URL or identifier of the user's resume, if provided.
  pub resume: Option<String>,

  /// Unix timestamp (in seconds) when the user account was created.
  pub created_at: u64,

  /// Unix timestamp (in seconds) when the user account was last updated.
  pub updated_at: u64,

  /// Total number of courses owned by the user.
  pub courses_owned: u32,
}

/// The `JsonUser` struct provides a comprehensive view of a user in the system.
/// It includes metadata and associated skills, certificates, and courses.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonUser {
  /// Unique identifier for the user, of type `UserId`.
  pub user_id: UserId,

  /// Detailed metadata about the user, of type `UserMetadata`.
  pub metadata: UserMetadata,

  /// Map of skills associated with the user.
  /// Keys are of type `SkillId`, and values are of type `SkillMetadata`.
  pub skill: HashMap<SkillId, u32>,

  /// Map of certificates that the user has obtained.
  /// Keys are of type `CertificateId`, and values are of type `CertificateMetadata`.
  pub certificate: Vec<CertificateId>,

  /// Map of courses associated with the user.
  /// Keys are of type `CourseId`, and values are of type `CourseMetadata`.
  // pub courses: HashMap<CourseId, CourseMetadata>,
  pub courses: Vec<CourseId>,
}

/// The `ImplUser` trait defines a set of behaviors associated with a user in the system.
pub trait ImplUser {
  /// Creates a new user with the provided nickname, first name, last name, and bio.
  /// The fields first_name, last_name, and bio are optional.
  fn create_user(
    &mut self,
    nickname: Option<String>,
    avatar: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
  );

  /// Updates the role of a user and returns the updated user as a `JsonUser`.
  fn update_role(&mut self) -> JsonUser;

  /// Update user information
  fn update_user_information(
    &mut self,
    nickname: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
    avatar: Option<String>,
    resume: Option<String>,
  ) -> JsonUser;
}

pub trait EnumUser {
  /// Get all information of users
  fn get_all_user_metadata(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<JsonUser>;

  /// Get all information of Instructors
  fn get_all_instructor_metadata(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<JsonUser>;

  /// Returns a `JsonUser` representation of the user's metadata for the given user ID.
  fn get_user_metadata_by_user_id(&self, user_id: &UserId) -> Option<JsonUser>;

  /// Check dose user is a Instructor or not
  fn get_user_role(&self, user_id: &UserId) -> Roles;
}
