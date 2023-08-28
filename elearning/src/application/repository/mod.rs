#![allow(unused)]

use crate::models::{course::CourseId, mentor::MentoringId, skill::SkillId, user::UserId};
use near_sdk::{env, AccountId, CryptoHash};
use unidecode::unidecode;

pub mod credit;
pub mod internal;
pub mod internal_certificate;
pub mod internal_combo;
pub mod internal_course;
pub mod internal_mentor;
pub mod internal_skill;
pub mod internal_user;
/*

Function for course

*/

pub(crate) fn convert_coure_title_to_cousrse_id(title: &str, account_id: String) -> String {
  let account = account_id.replace(".testnet", "").replace(".near", "");
  let unaccented = unidecode(title);
  let lowercased = unaccented.to_ascii_lowercase();
  let result = lowercased + " " + &account;
  result.replace(' ', "_")
}

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
  //get the default hash
  let mut hash = CryptoHash::default();
  //we hash the account ID and return it
  hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
  hash
}

//used to make sure the user attached exactly 1 yoctoNEAR
pub(crate) fn assert_one_yocto() {
  assert_eq!(env::attached_deposit(), 1, "Requires attached deposit of exactly 1 yoctoNEAR",)
}

//Assert that the user has attached at least 1 yoctoNEAR (for security reasons and to pay for storage)
pub(crate) fn assert_at_least_one_yocto() {
  assert!(env::attached_deposit() >= 1, "Requires attached deposit of at least 1 yoctoNEAR",)
}

/*

Function for skill

*/

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_skill_id(skill_id: &SkillId) -> CryptoHash {
  //get the default hash
  let mut hash = CryptoHash::default();
  //we hash the account ID and return it
  hash.copy_from_slice(&env::sha256(skill_id.as_bytes()));
  hash
}

/*

Function for certificate

*/
pub(crate) fn convert_to_certificate_id(course_id: &CourseId, student: &UserId) -> String {
  let cert = "cert ".to_ascii_lowercase();
  let student_convert = student.to_string().replace(".testnet", "").replace(".near", "").to_ascii_lowercase();
  let unaccented = unidecode(course_id);
  let lowercased = unaccented.to_ascii_lowercase();
  let result = cert + &lowercased + " " + &student_convert;
  result.replace(' ', "_")
}

/*

Function for mentoring

*/

pub(crate) fn convert_mentoring_title_to_mentoring_id(title: &str, account_id: String) -> String {
  let mentoring = "mentoring ".to_ascii_lowercase();
  let account = account_id.replace(".testnet", "").replace(".near", "");
  let unaccented = unidecode(title);
  let lowercased = unaccented.to_ascii_lowercase();
  let result = mentoring + &lowercased + " " + &account;
  result.replace(' ', "_")
}

pub(crate) fn convert_to_study_process_id(mentoring_id: &MentoringId, student: &UserId) -> String {
  let for_student = " for ".to_ascii_lowercase();
  let student_convert = student.to_string().replace(".testnet", " ").replace(".near", " ").to_ascii_lowercase();
  let unaccented = unidecode(mentoring_id);
  let lowercased = unaccented.to_ascii_lowercase();
  let time = env::block_timestamp_ms().to_string().to_ascii_lowercase();
  let result = lowercased + &for_student + &student_convert + &time;
  result.replace(' ', "_")
}

/*
Function for mentor
*/

pub(crate) fn convert_to_yocto(amount: u128) -> u128 {
  amount * (10u128.pow(24))
}

/* pool */

pub(crate) fn convert_pool_title_to_pool_id(title: &str) -> String {
  let unaccented = unidecode(title);
  let lowercased = unaccented.to_ascii_lowercase();
  lowercased.replace(' ', "_")
}

/* combo */

pub(crate) fn convert_combo_title_to_combo_id(title: &str) -> String {
  let combo = "combo ".to_ascii_lowercase();
  let unaccented = unidecode(title);
  let lowercased = unaccented.to_ascii_lowercase();
  let result = combo + &lowercased;
  result.replace(' ', "_")
}

pub(crate) fn check_encode(hash_collection: String, encode_check: String) -> bool {
  let secret = String::from("rTR3/4d4RCqJdS59ffoMb+N2uZYR8nwe");
  let combined = format!("{}{}", hash_collection, secret);
  // write input message

  let a = env::sha256(combined.as_bytes());
  let b = hex::encode(a);

  if b == encode_check {
    return true;
  }
  false
}
