use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet};

use crate::models::{
  certificate::CertificateId,
  contract::{ELearningContract, ELearningStorageKey},
  course::CourseId,
  user::UserId,
};

use super::hash_account_id;

impl ELearningContract {
  pub(crate) fn internal_add_certificate_to_user(&mut self, student_id: &UserId, certificate_id: &CertificateId) {
    let mut certificate_set = self.certificate_per_user.get(student_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ELearningStorageKey::CertificatePerUserInner { account_id_hash: hash_account_id(student_id) }
          .try_to_vec()
          .unwrap(),
      )
    });

    // Insert the certificate into the set
    certificate_set.insert(certificate_id);

    // Storage certificate in system
    self.certificate_per_user.insert(student_id, &certificate_set);

    // storage certificate in user data
    let mut new_user_data = self.user_metadata_by_id.get(student_id).unwrap();
    new_user_data.certificate.push(certificate_id.to_string());
    self.user_metadata_by_id.insert(student_id, &new_user_data);
  }
}
