use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  serde::{Deserialize, Serialize},
};

use super::{certificate::CertificateId, user::UserId};
use std::collections::HashMap;

/// `SkillId` is a type alias for `String`, typically representing a unique identifier for a skill in the system.
pub type SkillId = String;

/// Wrap Skill when finish course and add skill
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WrapSkill {
  /// Unique identifier for the skill, of type `SkillId`.
  pub skill_id: SkillId,

  /// Credit value associated with this skill.
  pub credit: u32,
}

/// The `SkillMetadata` struct represents metadata for a skill in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SkillMetadata {
  /// Unique identifier for the skill, of type `SkillId`.
  pub skill_id: SkillId,

  /// Credit value associated with this skill.
  pub credit: u32,

  /// Skill credit user own from
  pub credit_from: String,

  /// This credit add in credit of user's skill just 1 time
  pub use_skill: bool,
}

/// The Skill Feature trait define a set of acctivity user can do with their skills
pub trait SkillFeatures {
  /// Mint a skill from certificate credit
  fn mint_skill_by_certificate(&mut self, certificate_id: CertificateId, skills: Vec<WrapSkill>);

  /// Add new new skill by user
  fn add_skill(&mut self, student: UserId, skill_id: SkillId, credit: u32);
}

pub trait EnumSkill {
  /// Get all skills per user
  fn get_all_skills_per_user(&self, user_id: UserId) -> HashMap<SkillId, u32>;

  /// Get skill metadata by skill id
  fn get_all_skill_metadata_by_skill_id(
    &self,
    skill_id: SkillId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<SkillMetadata>;
}
