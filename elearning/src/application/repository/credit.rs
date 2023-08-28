use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  env, near_bindgen,
  serde::{Deserialize, Serialize},
};

use crate::models::{
  contract::{self, ELearningContract, ELearningContractExt},
  user::UserId,
};

#[derive(Deserialize, BorshDeserialize, BorshSerialize, Serialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum CreditType {
  Easy = 1,
  Intermediate = 2,
  Challenging = 3,
  Advanced = 5,
  Complex = 8,
  Formidable = 13,
  Insurmountable = 21,
}

#[near_bindgen]
impl ELearningContract {
  #[private]
  pub(crate) fn add_credit(&mut self, user_id: UserId, credit_type: CreditType) {
    let mut user_data = self.user_metadata_by_id.get(&user_id).unwrap();
    match credit_type {
      CreditType::Easy => {
        user_data.metadata.total_credit += CreditType::Easy as u32;
        self.user_metadata_by_id.insert(&user_id, &user_data);
      },

      CreditType::Intermediate => {
        user_data.metadata.total_credit += CreditType::Intermediate as u32;
        self.user_metadata_by_id.insert(&user_id, &user_data);
      },
      CreditType::Challenging => {
        user_data.metadata.total_credit += CreditType::Challenging as u32;
        self.user_metadata_by_id.insert(&user_id, &user_data);
      },
      CreditType::Advanced => {
        user_data.metadata.total_credit += CreditType::Advanced as u32;
        self.user_metadata_by_id.insert(&user_id, &user_data);
      },
      CreditType::Complex => {
        user_data.metadata.total_credit += CreditType::Complex as u32;
        self.user_metadata_by_id.insert(&user_id, &user_data);
      },
      CreditType::Formidable => {
        user_data.metadata.total_credit += CreditType::Formidable as u32;
        self.user_metadata_by_id.insert(&user_id, &user_data);
      },
      CreditType::Insurmountable => {
        user_data.metadata.total_credit += CreditType::Insurmountable as u32;
        self.user_metadata_by_id.insert(&user_id, &user_data);
      },
    }
  }

  #[private]
  pub(crate) fn add_credit_by_skill_reward(&mut self, skill_reward: u32) {
    let user_id = env::signer_account_id();
    let mut user_info = self.user_metadata_by_id.get(&user_id).unwrap();
    user_info.metadata.total_credit += skill_reward;
    self.user_metadata_by_id.insert(&user_id, &user_info);
  }
}
