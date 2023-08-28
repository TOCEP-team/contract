use crate::application::repository::{convert_mentoring_title_to_mentoring_id, convert_to_study_process_id};
use crate::models::contract::{ELearningContract, ELearningContractExt};
use crate::models::mentor::{MentorFeatures, MentoringId, StudyProcessId};
use crate::models::pool_request::external::cross_pool;
use crate::models::pool_request::pool::{GAS_FOR_CHECK_RESULT, GAS_FOR_CROSS_CALL};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, Balance};

#[near_bindgen]
/// Implement function for mentor
impl MentorFeatures for ELearningContract {
  /// Mentor create a mentoring
  fn create_mentoring(&mut self, mentoring_title: String, price_per_lession: U128, description: Option<String>) {
    // Make sure only user can call this function
    assert!(self.check_registration(&env::signer_account_id()), "You are not a user");
    // Comvert mentoring title to mentoring id
    let mentoring_id = convert_mentoring_title_to_mentoring_id(&mentoring_title, env::signer_account_id().to_string());
    assert!(!self.check_mentoring_existence(&mentoring_id), "Mentoring id already exist");

    // Cross call to pool and storage mentoring id, if success -> storage mentoring metadata in ELearning contract
    cross_pool::ext(self.pool_address.to_owned())
      .with_static_gas(GAS_FOR_CROSS_CALL)
      .add_mentoring_id(mentoring_id.clone())
      .then(Self::ext(env::current_account_id()).with_static_gas(GAS_FOR_CHECK_RESULT).storage_mentoring(
        mentoring_title,
        mentoring_id,
        price_per_lession,
        description,
      ));
  }

  /// Create and storage new study process, can only call by pool contract
  fn buy_mentoring_process(&mut self, mentoring_id: MentoringId, amount: U128) -> U128 {
    // Can only pool contract call this function
    assert!(self.pool_address == env::predecessor_account_id(), "You don't have permision");
    let student_id = env::signer_account_id();
    let value: Balance = amount.into();

    if !self.check_registration(&student_id) {
      return U128(2);
    }

    if student_id == self.mentoring_metadata_by_mentoring_id.get(&mentoring_id).unwrap().mentoring_owner {
      return U128(2);
    }

    let study_process_id = convert_to_study_process_id(&mentoring_id, &student_id);
    if self.internal_add_study_process(&mentoring_id, &student_id, study_process_id, value) {
      return U128(1);
    }
    U128(2)
  }

  fn make_lession_completed(&mut self, mentoring_id: MentoringId, study_process_id: StudyProcessId) {
    assert!(self.check_mentoring_existence(&mentoring_id), "Mentoring is not exist");
    let student_id = env::signer_account_id();
    assert!(self.check_student_in_mentoring(&mentoring_id, &student_id), "You are not a student in this mentoring");
    assert!(
      !self.check_study_process_state(&mentoring_id, &student_id, &study_process_id),
      "This study process already has finished"
    );

    if self.check_last_lession(&mentoring_id, &student_id, &study_process_id) {
      self.complete_last_lession(&mentoring_id, &study_process_id)
    } else {
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
        .remaining_amount -= price_per_lession;

      mentoring_info
        .study_process
        .get_mut(&student_id)
        .unwrap()
        .study_process_list
        .get_mut(&study_process_id)
        .unwrap()
        .lession_completed += 1;

      self.mentoring_metadata_by_mentoring_id.insert(&mentoring_id, &mentoring_info);
      self.mentoring_claim(mentoring_id.clone(), student_id.clone(), study_process_id.clone());
    }
    /*let total_lession_completed = mentoring_info
      .study_process
      .get(&student_id)
      .unwrap()
      .study_process_list
      .get(&study_process_id)
      .unwrap()
      .lession_completed;
    let total_lession = mentoring_info
      .study_process
      .get(&student_id)
      .unwrap()
      .study_process_list
      .get(&study_process_id)
      .unwrap()
      .total_lession;
    if total_lession_completed + 1 == total_lession {
      self.mentoring_withdraw(mentoring_id, study_process_id)
    }*/
  }

  /// Update mentoring
  fn update_mentoring(
    &mut self,
    mentoring_id: &MentoringId,
    price_per_lession: Option<U128>,
    description: Option<String>,
  ) {
    assert!(self.mentoring_metadata_by_mentoring_id.contains_key(mentoring_id), "This mentoring is not exsist");
    assert!(self.check_mentoring_owner(mentoring_id, &env::signer_account_id()), "You are not mentoring owner");

    let mut mentoring = self.mentoring_metadata_by_mentoring_id.get(mentoring_id).unwrap();
    // Check attribute. If it have some -> update
    if let Some(f) = description {
      mentoring.description = Some(f)
    }

    if let Some(l) = price_per_lession {
      let l: Balance = l.into();
      mentoring.price_per_lession = l
    }

    // Storage new coure data
    self.mentoring_metadata_by_mentoring_id.insert(mentoring_id, &mentoring);
  }

  fn mentoring_withdraw(&mut self, mentoring_id: MentoringId, study_process_id: StudyProcessId) {
    assert!(self.check_mentoring_withdraw_availability(&mentoring_id, &study_process_id), "You don't have access");
    let withdraw_amount = self
      .mentoring_metadata_by_mentoring_id
      .get(&mentoring_id)
      .unwrap()
      .study_process
      .get(&env::signer_account_id())
      .unwrap()
      .study_process_list
      .get(&study_process_id)
      .unwrap()
      .remaining_amount;
    cross_pool::ext(self.pool_address.to_owned())
      .with_static_gas(GAS_FOR_CROSS_CALL)
      .mentoring_withdraw(withdraw_amount.into())
      .then(
        Self::ext(env::current_account_id())
          .with_static_gas(GAS_FOR_CHECK_RESULT)
          .make_study_process_end(&mentoring_id, study_process_id),
      );
  }
}
