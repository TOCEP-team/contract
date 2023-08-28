use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Balance;
use std::collections::HashMap;

use super::user::UserId;

/// `MentoringId` is a type alias for `String`, typically representing a unique identifier for a mentoring in the system.
pub type MentoringId = String;
/// StudyProcessId, unique identifier for a study process for a student in mentoring
pub type StudyProcessId = String;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MentoringMetadata {
  /// Mentoring title
  pub mentoring_title: String,

  /// Mentoring id
  pub mentoring_id: MentoringId,

  /// Mentoring owner
  pub mentoring_owner: UserId,

  /// Price per lesstion
  pub price_per_lession: Balance,

  /// Description
  pub description: Option<String>,

  /// Map student and study process of student
  pub study_process: HashMap<UserId, StudyProcessList>,
}

/// When user buy a mentoring. Add mentoring data in mentoring list user have
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StudyProcessList {
  /// List study process user have in a mentoring
  pub study_process_list: HashMap<StudyProcessId, StudyProcessMetadata>,
}

/// When user buy a mentoring. All information is storaged by contract. One student can buy many study process
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StudyProcessMetadata {
  pub study_process_id: StudyProcessId,

  /// Student who buy this study process
  pub student_id: UserId,

  /// The time when study process init
  pub start_at: u64,

  /// Total lession student buy
  pub total_lession: u32,

  /// Mentoring completed
  pub mentoring_completed: bool,

  /// Total amount mentor have claimed
  pub total_claim: Balance,

  /// Lession has completed
  pub lession_completed: u32,

  /// Total amount student stake in study process
  pub total_amount: Balance,

  /// Remaining amount of student in study process
  pub remaining_amount: Balance,

  /// Amount student pay for 1 lession
  pub price_per_lession: Balance,
}

pub trait MentorFeatures {
  /// Mentor create a mentoring
  fn create_mentoring(&mut self, mentoring_title: String, price_per_lession: U128, description: Option<String>);

  /// Create and storage new study process, can only call by pool contract
  fn buy_mentoring_process(&mut self, mentoring_id: MentoringId, amount: U128) -> U128;

  /// Update mentoring
  fn update_mentoring(
    &mut self,
    mentoring_id: &MentoringId,
    price_per_lession: Option<U128>,
    description: Option<String>,
  );

  fn mentoring_withdraw(&mut self, mentoring_id: MentoringId, study_process_id: StudyProcessId);

  fn make_lession_completed(&mut self, mentoring_id: MentoringId, study_process_id: StudyProcessId);
}

pub trait EnumMentoring {
  /// Get mentoring metadata by mentoring id
  fn get_mentoring_metadata_by_mentoring_id(&self, mentoring_id: MentoringId) -> Option<MentoringMetadata>;

  /// Get all mentoring
  fn get_all_mentoring_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<MentoringMetadata>;
}
