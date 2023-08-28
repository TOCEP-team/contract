use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  env, near_bindgen,
  serde::{Deserialize, Serialize},
  Balance, Promise,
};

use crate::{
  application::repository::{convert_to_yocto, credit::CreditType},
  models::{
    combo::{ComboId, ComboState},
    contract::{ELearningContract, ELearningContractExt},
    course::{CourseId, EnumCourse},
    user::EnumUser,
  },
};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WrapComboHash {
  /// Course in combo
  pub course_id: CourseId,

  pub encode_check: String,
}

pub trait Payment {
  fn payment_course(&mut self, course_id: CourseId, encode_check: String);

  fn payment_combo(&mut self, combo_id: ComboId, combo_hash: Vec<WrapComboHash>);

  fn internal_tranfer_course(&mut self, course_id: CourseId, price: Balance, encode_check: String);
}

#[near_bindgen]
impl Payment for ELearningContract {
  #[payable]
  fn payment_course(&mut self, course_id: CourseId, encode_check: String) {
    // Check course has exists
    let user_id = env::signer_account_id();
    assert!(self.internal_check_course_exits(&course_id), "The course doesn't exists");
    assert!(self.internal_check_user_has_register(&user_id), "You need registration to use platform!");
    let course = self.get_course_metadata_by_course_id(course_id.clone()).unwrap();
    assert!(self.internal_check_subscriber_has_course(&user_id, &course_id), "You already have this course!");
    let amount_deposit = env::attached_deposit();
    assert!(amount_deposit >= convert_to_yocto(course.price), "You do not deposit enough money");
    self.internal_tranfer_course(course_id, course.price, encode_check);
  }

  #[payable]
  fn payment_combo(&mut self, combo_id: ComboId, combo_hash: Vec<WrapComboHash>) {
    assert!(self.check_combo_existence(&combo_id), "Please check combo id");
    assert!(self.check_combo_state(&combo_id) == ComboState::ACTIVE, "This combo is deactive");
    for input_course in combo_hash.clone() {
      assert!(self.check_course_in_combo(&combo_id, &input_course.course_id), "Please check course id is not in combo");
    }

    let price_set = self.combo_metadata_by_combo_id.get(&combo_id).unwrap().courses;
    let mut combo_price: u128 = 0;
    for price_per_course in price_set.iter() {
      combo_price += price_per_course.price;
    }
    let amount_deposit = env::attached_deposit();
    assert!(amount_deposit >= convert_to_yocto(combo_price), "You do not deposit enough money");
    let mut unique_course: Vec<WrapComboHash> = Vec::new();
    let mut unique_course_id: Vec<CourseId> = Vec::new();

    // Ensure that the courses are not duplicated
    for course_info in combo_hash {
      if !unique_course_id.contains(&course_info.course_id) {
        unique_course.push(course_info.clone());
        unique_course_id.push(course_info.course_id);
      }
    }
    assert!(
      self.combo_metadata_by_combo_id.get(&combo_id).unwrap().enable_course.len() == unique_course.len(),
      "the courses are duplicated"
    );
    //self.tranfer_combo(combo_id);
    for per_course in unique_course {
      let mut price: Balance = 0;
      let course = self.combo_metadata_by_combo_id.get(&combo_id).unwrap().courses;
      for get_price in course.iter() {
        if get_price.course_id == per_course.course_id {
          price = get_price.price;
        }
      }
      self.internal_tranfer_course(per_course.course_id, price, per_course.encode_check)
    }
  }

  #[private]
  fn internal_tranfer_course(&mut self, course_id: CourseId, price: Balance, encode_check: String) {
    let user_id = env::signer_account_id();
    let price = convert_to_yocto(price);
    let mut course = self.get_course_metadata_by_course_id(course_id.clone()).unwrap();
    // Plus 1 student to course owner
    for i in course.instructor_id.keys() {
      let mut course_owner = self.get_user_metadata_by_user_id(i).unwrap();

      let tranfer_percent = course.instructor_id.get(i).unwrap();
      let convert_to_subtract = (*tranfer_percent) as u128;
      Promise::new(i.clone()).transfer(price / 10_000 * convert_to_subtract);
      course_owner.metadata.students += 1;
      if (course.students_studying_map.len() + 1) % 5 == 0 {
        self.add_credit(i.clone(), CreditType::Easy);
      }
      self.user_metadata_by_id.insert(&course_owner.user_id, &course_owner);
    }

    course.students_studying_map.insert(user_id.clone(), encode_check.to_string());
    let mut user = self.get_user_metadata_by_user_id(&user_id).unwrap();
    user.courses.push(course_id.clone());
    self.user_metadata_by_id.insert(&user_id, &user);
    self.course_metadata_by_id.insert(&course_id, &course);
  }
}
