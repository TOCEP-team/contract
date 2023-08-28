use crate::models::{
  certificate::CertificateId,
  contract::{ELearningContract, ELearningContractExt},
  skill::{SkillFeatures, SkillId, SkillMetadata, WrapSkill},
  user::UserId,
};

use near_sdk::{env, near_bindgen};

#[near_bindgen]
impl SkillFeatures for ELearningContract {
  /// Add new new skill by user
  #[private]
  fn add_skill(&mut self, student: UserId, skill_id: SkillId, credit: u32) {
    let mut user = self.user_metadata_by_id.get(&student).unwrap();
    user.skill.entry(skill_id).and_modify(|x| *x += credit).or_insert(credit);
    self.user_metadata_by_id.insert(&student, &user);
  }

  /// Mint a skill from certificate credit
  #[private]
  fn mint_skill_by_certificate(&mut self, certificate_id: CertificateId, skills: Vec<WrapSkill>) {
    // Create new skillmetadata by skill id in system contract
    for skill_to_add in skills.iter() {
      let new_skill_data = SkillMetadata {
        skill_id: skill_to_add.skill_id.clone(),
        credit: skill_to_add.credit,
        credit_from: certificate_id.clone(),
        use_skill: true,
      };

      //internal skill to add new skill metadata to skill id
      self.internal_add_skill_metadata_to_skill_id(&skill_to_add.skill_id, &new_skill_data);

      // Add new skill credit for user
      self.add_skill(env::signer_account_id(), skill_to_add.skill_id.clone(), skill_to_add.credit);
    }
  }
}
