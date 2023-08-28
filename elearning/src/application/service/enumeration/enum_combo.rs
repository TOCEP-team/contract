use crate::models::{
  combo::{ComboId, ComboMetadata, EnumCombo},
  contract::{ELearningContract, ELearningContractExt},
};
use near_sdk::near_bindgen;

#[near_bindgen]
impl EnumCombo for ELearningContract {
  fn get_all_combo_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<ComboMetadata> {
    self
      .all_combo_id
      .iter()
      .skip(start.unwrap_or(0) as usize)
      .take(limit.unwrap_or(20) as usize)
      .map(|x| self.combo_metadata_by_combo_id.get(&x).unwrap())
      .collect()
  }

  fn get_combometadata_by_combo_id(&self, combo_id: &ComboId) -> Option<ComboMetadata> {
    if let Some(combo) = self.combo_metadata_by_combo_id.get(combo_id) {
      Some(combo)
    } else {
      None
    }
  }
}
