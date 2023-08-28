#![allow(clippy::too_many_arguments)]
use crate::{
  application::repository::convert_to_certificate_id,
  models::{
    certificate::{CertificateFeatures, CertificateMetadata},
    contract::{ELearningContract, ELearningContractExt},
    course::CourseId,
    skill::WrapSkill,
    user::UserId,
  },
};
use near_sdk::near_bindgen;

#[near_bindgen]
/// Implement function for certificate
impl CertificateFeatures for ELearningContract {
  #[private]
  fn mint_certificate(&mut self, course_id: CourseId, student_id: UserId, skills: Vec<WrapSkill>, media: String) {
    // this function only for course owner
    //let check_owner = env::signer_account_id();
    let course = self.course_metadata_by_id.get(&course_id).unwrap();

    //assert!(course.instructor_id.contains_key(&check_owner), "You are not the course owner");
    assert!(course.students_studying_map.contains_key(&student_id), "This user is not a student in course");
    //assert!(course.students_completed.contains_key(&student_id), "Student are not completed the course");

    let certificate_id = convert_to_certificate_id(&course_id, &student_id);
    assert!(
      !self.user_metadata_by_id.get(&student_id).unwrap().certificate.contains(&certificate_id),
      "This certificate already exist"
    );

    // New certificate data
    let certificate_metadata =
      CertificateMetadata { certificate_id: certificate_id.clone(), student: student_id.clone(), media, skills };

    // Storage certificate in system contract
    self.certificate_metadata_by_id.insert(&certificate_id, &certificate_metadata);

    // Storage certificate in student's data
    self.internal_add_certificate_to_user(&student_id, &certificate_id);
  }
}
