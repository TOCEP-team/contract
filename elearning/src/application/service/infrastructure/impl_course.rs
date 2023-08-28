use crate::{
  application::repository::{check_encode, convert_coure_title_to_cousrse_id, convert_to_certificate_id},
  models::{
    certificate::CertificateFeatures,
    contract::{ELearningContract, ELearningContractExt},
    course::{CourseFeatures, CourseId, CourseMetadata},
    skill::{SkillFeatures, SkillId, WrapSkill},
    user::{Roles, UserId},
  },
};
use near_sdk::{env, json_types::U128, near_bindgen, Balance};
use std::collections::HashMap;

#[near_bindgen]
impl CourseFeatures for ELearningContract {
  fn create_course(
    &mut self,
    title: String,
    description: Option<String>,
    media: Option<String>,
    price: U128,
    skills: Vec<SkillId>,
  ) -> CourseMetadata {
    let instructor_id = env::signer_account_id();
    assert!(
      self.user_metadata_by_id.get(&instructor_id).unwrap().metadata.role == Roles::Instructor,
      "You aren't an instructor, You need register & upload your resume to become a instructor!"
    );

    let course_id = convert_coure_title_to_cousrse_id(&title, instructor_id.to_string());
    assert!(
      !self.course_metadata_by_id.contains_key(&course_id),
      "Please! Change your title course, it already exists"
    );

    let price: Balance = price.into();
    let mut initial_instructor: HashMap<UserId, u32> = HashMap::new();
    initial_instructor.insert(instructor_id.clone(), 10000);

    // New course data
    let course_metadata = CourseMetadata {
      course_id: course_id.clone(),
      title,
      skills,
      price,
      media,
      description,
      instructor_id: initial_instructor,
      created_at: env::block_timestamp_ms(),
      update_at: env::block_timestamp_ms(),
      students_completed: HashMap::new(),
      students_studying_map: HashMap::new(),
      rating: 0,
      rating_count: 0,
      content: "".to_string(),
      consensus: HashMap::new(),
    };
    // Storage in system contract
    self.course_metadata_by_id.insert(&course_id, &course_metadata);
    self.all_course_id.insert(&course_id);

    // Storage in user data
    let mut user = self.user_metadata_by_id.get(&instructor_id).unwrap();
    user.metadata.courses_owned += 1;
    self.user_metadata_by_id.insert(&instructor_id, &user);
    self.internal_add_course_to_instructor(&instructor_id, &course_id);
    course_metadata
  }

  // TODO: More Requirement to check
  fn make_user_finish_course(&mut self, course_id: CourseId, media: String, rating: u16, hash_collection: String) {
    assert!(rating <= 10, "your rating must be less than or equal to 10");
    let mut course = self.course_metadata_by_id.get(&course_id).unwrap();

    let student_id = env::signer_account_id();
    // Check the courser owner
    //assert!(course.instructor_id.contains_key(&env::signer_account_id()), "You are not the course owner");
    // Check user are student in this course or not
    assert!(course.students_studying_map.contains_key(&student_id), "This user is not a student in this course");
    // Check: has student complete the course yet
    let certificate_id = convert_to_certificate_id(&course_id, &student_id);
    assert!(
      !self.user_metadata_by_id.get(&student_id).unwrap().certificate.contains(&certificate_id),
      "This student already completed the course"
    );
    let encode_check =
      self.course_metadata_by_id.get(&course_id).unwrap().students_studying_map.get(&student_id).unwrap().clone();
    assert!(check_encode(hash_collection, encode_check), "hash collection is incorect");

    let mut skills: Vec<WrapSkill> = Vec::new();
    for skill_id in course.skills.iter() {
      let wrap_skill = WrapSkill { skill_id: skill_id.to_string(), credit: 10 };
      skills.push(wrap_skill);
    }
    // Update new data
    course.students_completed.insert(student_id.clone(), rating);
    self.course_metadata_by_id.insert(&course_id, &course);
    self.mint_certificate(course_id, student_id, skills.clone(), media);
    self.mint_skill_by_certificate(certificate_id, skills.clone());
    for skill_info in skills.iter() {
      self.add_credit_by_skill_reward(skill_info.credit);
    }
  }

  /// Update course
  fn update_course(
    &mut self,
    course_id: CourseId,
    content: Option<String>,
    description: Option<String>,
    media: Option<String>,
    price: Option<U128>,
    title: Option<String>,
  ) {
    assert!(self.course_metadata_by_id.contains_key(&course_id), "This course is not exsist");
    assert!(
      self.course_metadata_by_id.get(&course_id).unwrap().instructor_id.contains_key(&env::signer_account_id()),
      "You are not course owner"
    );

    let mut course = self.course_metadata_by_id.get(&course_id).unwrap();
    // Check attribute. If it have some -> update
    if let Some(n) = content {
      course.content = n
    };
    if let Some(f) = description {
      course.description = Some(f)
    }

    if let Some(l) = media {
      course.media = Some(l)
    }

    if let Some(b) = price {
      let b: Balance = b.into();
      course.price = b
    }

    if let Some(a) = title {
      let new_title = convert_coure_title_to_cousrse_id(&a, env::signer_account_id().to_string());
      course.title = new_title;
    }

    course.update_at = env::block_timestamp_ms();
    // Storage new coure data
    self.course_metadata_by_id.insert(&course_id, &course);
  }
}
