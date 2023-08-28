#![allow(clippy::too_many_arguments)]

use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  serde::{Deserialize, Serialize},
  Balance,
};

use super::course::CourseId;

pub type ComboId = String;

#[derive(Deserialize, BorshDeserialize, BorshSerialize, Serialize, Default, Debug, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ComboState {
  #[default]
  /// Deactive state
  DEACTIVED,

  /// Active state
  ACTIVE,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WrapCombo {
  /// Course in combo
  pub course_id: CourseId,

  pub price: Balance,
}

/// The `CourseMetadata` struct represents metadata for a course in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ComboMetadata {
  /// Unique identifier for the combo
  pub combo_id: ComboId,

  /// Combo state
  pub combo_state: ComboState,

  /// Enable course
  pub enable_course: Vec<CourseId>,

  /// Price of each course in combo
  pub courses: Vec<WrapCombo>,

  pub description: Option<String>,

  pub media: Option<String>,
}

pub trait ComboFeatures {
  fn create_combo(
    &mut self,
    combo_title: String,
    courses: Vec<WrapCombo>,
    description: Option<String>,
    media: Option<String>,
  );

  fn enable_course(&mut self, combo_id: ComboId, course_id: CourseId);
}

pub trait EnumCombo {
  fn get_combometadata_by_combo_id(&self, combo_id: &ComboId) -> Option<ComboMetadata>;

  fn get_all_combo_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<ComboMetadata>;
}
