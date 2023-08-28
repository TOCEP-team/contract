#![allow(clippy::too_many_arguments)]
use crate::models::{
  certificate::{CertificateId, CertificateMetadata, EnumCertificate},
  contract::{ELearningContract, ELearningContractExt},
  user::UserId,
};
use near_sdk::near_bindgen;

#[near_bindgen]
/// Implement function for certificate
impl EnumCertificate for ELearningContract {
  /// Get all certicicate by user id
  fn get_all_certificate_by_user_id(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CertificateMetadata> {
    assert!(self.check_registration(&user_id), "This user is not exist");
    // Take user's certificate id
    let certificate_id = self.user_metadata_by_id.get(&user_id).unwrap().certificate;
    certificate_id
      .iter()
      .skip(start.unwrap_or(0) as usize)
      .take(limit.unwrap_or(20) as usize)
      .map(|x| self.certificate_metadata_by_id.get(x).unwrap())
      .collect()
  }

  /// Get certificate metadata by certificate id
  fn get_certificate_metadata_by_certificate_id(&self, certificate_id: CertificateId) -> Option<CertificateMetadata> {
    /* uncomment this code when use event */
    //assert!(self.certificate_metadata_by_id.contains_key(&certificate_id), "Certificate is not exsist");
    if let Some(data) = self.certificate_metadata_by_id.get(&certificate_id) {
      Some(data)
    } else {
      None
    }
  }
}
