use crate::models::contract::{ELearningContract, ELearningContractExt};
use crate::models::mentor::{EnumMentoring, MentoringId, MentoringMetadata};
use near_sdk::near_bindgen;

#[near_bindgen]
/// Implement function for mentor
impl EnumMentoring for ELearningContract {
  /// Get study process state
  /// Get all mentoring
  // TODO: Error
  fn get_all_mentoring_metadata(&self, start: Option<u32>, limit: Option<u32>) -> Vec<MentoringMetadata> {
    self
      .all_mentoring_id
      .iter()
      .skip(start.unwrap_or(0) as usize)
      .take(limit.unwrap_or(20) as usize)
      .map(|x| self.mentoring_metadata_by_mentoring_id.get(&x).unwrap())
      .collect()
  }

  /// Get mentoring metadata by mentoring id
  fn get_mentoring_metadata_by_mentoring_id(&self, mentoring_id: MentoringId) -> Option<MentoringMetadata> {
    if let Some(metadata) = self.mentoring_metadata_by_mentoring_id.get(&mentoring_id) {
      Some(metadata)
    } else {
      None
    }
  }
}
