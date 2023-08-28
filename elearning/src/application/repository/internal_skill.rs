use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet};

use crate::models::{
  contract::{ELearningContract, ELearningStorageKey},
  skill::{SkillId, SkillMetadata},
};

use super::hash_skill_id;

impl ELearningContract {
  pub(crate) fn internal_add_skill_metadata_to_skill_id(&mut self, skill_id: &SkillId, skill_metadata: &SkillMetadata) {
    let mut skill_set = self.skill_metadata_by_skill_id.get(skill_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ELearningStorageKey::SkillMetadataPerSkillIdInner { skill_id_hash: hash_skill_id(skill_id) }
          .try_to_vec()
          .unwrap(),
      )
    });

    //we insert the token ID into the set
    skill_set.insert(skill_metadata);

    self.skill_metadata_by_skill_id.insert(skill_id, &skill_set);
  }
}
