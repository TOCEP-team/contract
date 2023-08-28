pub mod application;
pub mod models;

use models::contract::{ELearningContract, ELearningContractExt, ELearningContractMetadata, ELearningStorageKey};
use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId};

#[near_bindgen]
impl ELearningContract {
  #[init]
  pub fn init() -> Self {
    let owner_id = env::signer_account_id();
    Self::new(
      owner_id,
      ELearningContractMetadata {
        spec: "elearning-1.0.0".to_string(),
        name: "elearing".to_string(),
        symbol: "EganTeam".to_string(),
        icon: None,
        base_uri: None,
        reference: None,
        reference_hash: None,
      },
    )
  }

  #[init]
  pub fn new(owner_id: AccountId, metadata: ELearningContractMetadata) -> Self {
    Self {
      owner_id,
      pool_address: env::signer_account_id(),
      metadata_contract: LazyOption::new(ELearningStorageKey::ContractMetadata.try_to_vec().unwrap(), Some(&metadata)),
      subscriber_users: UnorderedSet::new(ELearningStorageKey::SubscriberUsers.try_to_vec().unwrap()),
      intructor_users: UnorderedSet::new(ELearningStorageKey::IntructorUsers.try_to_vec().unwrap()),
      all_course_id: UnorderedSet::new(ELearningStorageKey::AllCourseId.try_to_vec().unwrap()),
      mentor_users: UnorderedMap::new(ELearningStorageKey::MentorUsers.try_to_vec().unwrap()),
      user_metadata_by_id: LookupMap::new(ELearningStorageKey::UserMetadataById.try_to_vec().unwrap()),
      courses_per_user: LookupMap::new(ELearningStorageKey::CoursesPerUser.try_to_vec().unwrap()),
      courses_per_instructor: LookupMap::new(ELearningStorageKey::CoursesPerInstructor.try_to_vec().unwrap()),
      course_metadata_by_id: LookupMap::new(ELearningStorageKey::CourseMetadataById.try_to_vec().unwrap()),
      certificate_per_user: LookupMap::new(ELearningStorageKey::CertificatesPerUser.try_to_vec().unwrap()),
      certificate_metadata_by_id: LookupMap::new(ELearningStorageKey::CertificateMetadataById.try_to_vec().unwrap()),
      skill_metadata_by_skill_id: LookupMap::new(ELearningStorageKey::SkillMetadataPerSkillId.try_to_vec().unwrap()),
      all_pool_id: UnorderedSet::new(ELearningStorageKey::AllPoolId.try_to_vec().unwrap()),
      pool_metadata_by_pool_id: LookupMap::new(ELearningStorageKey::PoolMetadataByPoolId.try_to_vec().unwrap()),
      all_combo_id: UnorderedSet::new(ELearningStorageKey::AllComboId.try_to_vec().unwrap()),
      combo_metadata_by_combo_id: LookupMap::new(ELearningStorageKey::ComboMetadataByComboId.try_to_vec().unwrap()),
      all_mentoring_id: UnorderedSet::new(ELearningStorageKey::AllMentoringId.try_to_vec().unwrap()),
      mentoring_metadata_by_mentoring_id: LookupMap::new(
        ELearningStorageKey::MentoringMetadataByMentoringId.try_to_vec().unwrap(),
      ),
    }
  }
}
