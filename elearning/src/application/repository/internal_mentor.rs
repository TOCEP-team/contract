use std::collections::HashMap;

use crate::application::repository::convert_to_yocto;
use crate::models::contract::{ELearningContract, ELearningContractExt};
use crate::models::mentor::{MentoringId, MentoringMetadata, StudyProcessId, StudyProcessList, StudyProcessMetadata};
use crate::models::pool_request::external::cross_pool;
use crate::models::pool_request::pool::{GAS_FOR_CHECK_RESULT, GAS_FOR_CROSS_CALL};
use crate::models::user::UserId;
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, Balance, PromiseResult};

use super::credit::CreditType;

#[near_bindgen]
/// Implement function for mentor
impl ELearningContract {
  /// Check mentoring exist or not
  pub(crate) fn check_mentoring_existence(&self, mentoring_id: &MentoringId) -> bool {
    self.mentoring_metadata_by_mentoring_id.contains_key(mentoring_id)
  }

  pub(crate) fn check_study_process_state(
    &self,
    mentoring_id: &MentoringId,
    student_id: &UserId,
    study_process_id: &StudyProcessId,
  ) -> bool {
    assert!(self.check_mentoring_existence(mentoring_id), "Mentoring is not exist");
    assert!(self.check_student_in_mentoring(mentoring_id, student_id), "Student id is incorect");
    self
      .mentoring_metadata_by_mentoring_id
      .get(mentoring_id)
      .unwrap()
      .study_process
      .get(student_id)
      .unwrap()
      .study_process_list
      .get(study_process_id)
      .unwrap()
      .mentoring_completed
  }

  pub(crate) fn check_student_in_mentoring(&self, mentoring_id: &MentoringId, student_id: &UserId) -> bool {
    assert!(self.check_mentoring_existence(mentoring_id), "This mentoring is not exist");
    if self.mentoring_metadata_by_mentoring_id.get(mentoring_id).unwrap().study_process.contains_key(student_id) {
      return true;
    }
    false
  }

  /// Internal add study process
  #[private]
  pub(crate) fn internal_add_study_process(
    &mut self,
    mentoring_id: &MentoringId,
    student_id: &UserId,
    study_process_id: StudyProcessId,
    value: Balance,
  ) -> bool {
    let mut mentoring_info = self.mentoring_metadata_by_mentoring_id.get(mentoring_id).unwrap();
    let update_price_per_lession = self.mentoring_metadata_by_mentoring_id.get(mentoring_id).unwrap().price_per_lession;
    let study_process_info = StudyProcessMetadata {
      study_process_id: study_process_id.clone(),
      student_id: student_id.clone(),
      start_at: env::block_timestamp_ms(),
      total_lession: (value / update_price_per_lession) as u32,
      lession_completed: 0,
      mentoring_completed: false,
      total_claim: 0,
      total_amount: value,
      remaining_amount: value,
      price_per_lession: update_price_per_lession,
    };

    if !(mentoring_info.study_process.contains_key(student_id)) {
      let mut new_study_process_list = StudyProcessList { study_process_list: HashMap::new() };

      new_study_process_list.study_process_list.insert(study_process_id, study_process_info);
      mentoring_info.study_process.insert(student_id.clone(), new_study_process_list);

      self.mentoring_metadata_by_mentoring_id.insert(mentoring_id, &mentoring_info);
      true
    } else {
      if self.check_duplicate_study_process(mentoring_id, student_id, &study_process_id) {
        return false;
      };
      mentoring_info
        .study_process
        .get_mut(student_id)
        .unwrap()
        .study_process_list
        .insert(study_process_id, study_process_info);
      self.mentoring_metadata_by_mentoring_id.insert(mentoring_id, &mentoring_info);
      true
    }
  }

  pub(crate) fn check_duplicate_study_process(
    &self,
    mentoring_id: &MentoringId,
    student_id: &UserId,
    study_process_id: &StudyProcessId,
  ) -> bool {
    self
      .mentoring_metadata_by_mentoring_id
      .get(mentoring_id)
      .unwrap()
      .study_process
      .get(student_id)
      .unwrap()
      .study_process_list
      .contains_key(study_process_id)
  }

  #[private]
  pub fn storage_mentoring(
    &mut self,
    mentoring_title: String,
    mentoring_id: MentoringId,
    price_per_lession: U128,
    description: Option<String>,
  ) {
    let price_per_lession: Balance = convert_to_yocto(price_per_lession.into());
    let mentoring_info = MentoringMetadata {
      mentoring_title,
      mentoring_id: mentoring_id.clone(),
      mentoring_owner: env::signer_account_id(),
      price_per_lession,
      description,
      study_process: HashMap::new(),
    };
    self.all_mentoring_id.insert(&mentoring_id);
    self.mentoring_metadata_by_mentoring_id.insert(&mentoring_id, &mentoring_info);
  }

  /// Check user exist or not
  pub(crate) fn check_mentoring_owner(&self, mentoring_id: &MentoringId, owner_id: &UserId) -> bool {
    assert!(self.check_mentoring_existence(mentoring_id), "Mentoring is not exist");
    if self.mentoring_metadata_by_mentoring_id.get(mentoring_id).unwrap().mentoring_owner == owner_id.clone() {
      return true;
    }
    false
  }

  /// tranfer money when lession complete
  #[private]
  pub(crate) fn mentoring_claim(
    &mut self,
    mentoring_id: MentoringId,
    student_id: UserId,
    study_process_id: StudyProcessId,
  ) {
    assert!(self.check_mentoring_existence(&mentoring_id), "Mentoring is not exist");
    //assert!(self.check_claim_availability(&mentoring_id), "You are not mentoring owner");
    assert!(self.check_student_in_mentoring(&mentoring_id, &student_id), "Student id is incorect");
    assert!(
      !self.check_study_process_state(&mentoring_id, &student_id, &study_process_id),
      "This study proces has completed"
    );
    let mentoring_owner = self.mentoring_metadata_by_mentoring_id.get(&mentoring_id).unwrap().mentoring_owner;
    let amount = self
      .mentoring_metadata_by_mentoring_id
      .get(&mentoring_id)
      .unwrap()
      .study_process
      .get(&student_id)
      .unwrap()
      .study_process_list
      .get(&study_process_id)
      .unwrap()
      .price_per_lession;
    cross_pool::ext(self.pool_address.to_owned())
      .with_static_gas(GAS_FOR_CROSS_CALL)
      .mentoring_claim(mentoring_owner, amount.into())
      .then(Self::ext(env::current_account_id()).with_static_gas(GAS_FOR_CHECK_RESULT).remove_claim_info(
        mentoring_id,
        student_id,
        study_process_id,
      ));
  }

  //#[private]
  //fn check_claim_availability(&self, mentoring_id: &MentoringId) -> bool {
  //  if env::signer_account_id() == self.mentoring_metadata_by_mentoring_id.get(mentoring_id).unwrap().mentoring_owner {
  //   return true;
  //  }
  //  false
  //}

  #[private]
  pub fn remove_claim_info(&mut self, mentoring_id: MentoringId, student_id: UserId, study_process_id: StudyProcessId) {
    let result = match env::promise_result(0) {
      PromiseResult::NotReady => env::abort(),
      PromiseResult::Successful(value) => {
        if let Ok(refund) = near_sdk::serde_json::from_slice::<U128>(&value) {
          refund.0
          // If we can't properly parse the value, the original amount is returned.
        } else {
          U128(2).into()
        }
      },
      PromiseResult::Failed => U128(2).into(),
    };

    if result == 1 {
      let mut mentoring_info = self.mentoring_metadata_by_mentoring_id.get(&mentoring_id).unwrap();

      let price_per_lession = mentoring_info
        .study_process
        .get(&student_id)
        .unwrap()
        .study_process_list
        .get(&study_process_id)
        .unwrap()
        .price_per_lession;

      mentoring_info
        .study_process
        .get_mut(&student_id)
        .unwrap()
        .study_process_list
        .get_mut(&study_process_id)
        .unwrap()
        .total_claim += price_per_lession;

      self.mentoring_metadata_by_mentoring_id.insert(&mentoring_id, &mentoring_info);
    }
  }

  #[private]
  pub(crate) fn complete_last_lession(&mut self, mentoring_id: &MentoringId, study_process_id: &StudyProcessId) {
    let mentoring_info = self.mentoring_metadata_by_mentoring_id.get(mentoring_id).unwrap();
    let price_per_lession = mentoring_info
      .study_process
      .get(&env::signer_account_id())
      .unwrap()
      .study_process_list
      .get(study_process_id)
      .unwrap()
      .price_per_lession;
    let current_remaining_amout = mentoring_info
      .study_process
      .get(&env::signer_account_id())
      .unwrap()
      .study_process_list
      .get(study_process_id)
      .unwrap()
      .remaining_amount;
    let receiver = mentoring_info.mentoring_owner;

    let remaining_amout = current_remaining_amout - price_per_lession;
    cross_pool::ext(self.pool_address.to_owned())
      .with_static_gas(GAS_FOR_CROSS_CALL)
      .transfer_last_lession(receiver, price_per_lession.into(), remaining_amout.into())
      .then(
        Self::ext(env::current_account_id())
          .with_static_gas(GAS_FOR_CHECK_RESULT)
          .complete_study_process(mentoring_id, study_process_id.to_string()),
      );
  }

  #[private]
  pub(crate) fn check_last_lession(
    &mut self,
    mentoring_id: &MentoringId,
    student_id: &UserId,
    study_process_id: &StudyProcessId,
  ) -> bool {
    let lession_completed = self
      .mentoring_metadata_by_mentoring_id
      .get(mentoring_id)
      .unwrap()
      .study_process
      .get(student_id)
      .unwrap()
      .study_process_list
      .get(study_process_id)
      .unwrap()
      .lession_completed;

    let total_lession = self
      .mentoring_metadata_by_mentoring_id
      .get(mentoring_id)
      .unwrap()
      .study_process
      .get(student_id)
      .unwrap()
      .study_process_list
      .get(study_process_id)
      .unwrap()
      .total_lession;

    if lession_completed == total_lession - 1 {
      return true;
    }
    false
  }

  /// Get all information of users
  //

  /// Make study process end when student with draw money
  #[private]
  pub fn make_study_process_end(&mut self, mentoring_id: &MentoringId, study_process_id: StudyProcessId) {
    let student_id = env::signer_account_id();

    let mut mentoring_info = self.mentoring_metadata_by_mentoring_id.get(mentoring_id).unwrap();

    mentoring_info
      .study_process
      .get_mut(&student_id)
      .unwrap()
      .study_process_list
      .get_mut(&study_process_id)
      .unwrap()
      .mentoring_completed = true;

    mentoring_info
      .study_process
      .get_mut(&student_id)
      .unwrap()
      .study_process_list
      .get_mut(&study_process_id)
      .unwrap()
      .remaining_amount = 0;

    self.mentoring_metadata_by_mentoring_id.insert(mentoring_id, &mentoring_info);
  }

  #[private]
  pub(crate) fn check_mentoring_withdraw_availability(
    &self,
    mentoring_id: &MentoringId,
    study_process_id: &StudyProcessId,
  ) -> bool {
    if !self.check_mentoring_existence(mentoring_id) {
      return false;
    }

    if !self.check_student_in_mentoring(mentoring_id, &env::signer_account_id()) {
      return false;
    }

    if self.check_study_process_state(mentoring_id, &env::signer_account_id(), study_process_id) {
      return false;
    }
    true
  }

  #[private]
  pub fn complete_study_process(&mut self, mentoring_id: &MentoringId, study_process_id: StudyProcessId) {
    let student_id = env::signer_account_id();

    let mut mentoring_info = self.mentoring_metadata_by_mentoring_id.get(mentoring_id).unwrap();

    let owner_id = mentoring_info.mentoring_owner.clone();

    mentoring_info
      .study_process
      .get_mut(&student_id)
      .unwrap()
      .study_process_list
      .get_mut(&study_process_id)
      .unwrap()
      .mentoring_completed = true;

    mentoring_info
      .study_process
      .get_mut(&student_id)
      .unwrap()
      .study_process_list
      .get_mut(&study_process_id)
      .unwrap()
      .remaining_amount = 0;

    let price_per_lession = mentoring_info
      .study_process
      .get(&student_id)
      .unwrap()
      .study_process_list
      .get(&study_process_id)
      .unwrap()
      .price_per_lession;

    mentoring_info
      .study_process
      .get_mut(&student_id)
      .unwrap()
      .study_process_list
      .get_mut(&study_process_id)
      .unwrap()
      .total_claim += price_per_lession;

    mentoring_info
      .study_process
      .get_mut(&student_id)
      .unwrap()
      .study_process_list
      .get_mut(&study_process_id)
      .unwrap()
      .lession_completed += 1;

    self.mentoring_metadata_by_mentoring_id.insert(mentoring_id, &mentoring_info);
    self.add_credit(student_id, CreditType::Easy);
    self.add_credit(owner_id, CreditType::Easy);
  }
}
