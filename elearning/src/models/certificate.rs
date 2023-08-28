#![allow(clippy::too_many_arguments)]
use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  serde::{Deserialize, Serialize},
};

use super::{course::CourseId, skill::WrapSkill, user::UserId};

/// `CertificateId` is a type alias for `String`, typically representing a unique identifier for a certificate in the system.
pub type CertificateId = String;

/// The `CertificateMetadata` struct represents metadata for a certificate in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CertificateMetadata {
  /// Unique identifier for the certificate, of type `CertificateId`.
  pub certificate_id: CertificateId,

  /// Student own the certificate. Course mint certificate and send to student
  pub student: UserId,

  /// Skill user own after complete the course
  pub skills: Vec<WrapSkill>,

  /// The certficate link
  pub media: String,
}

pub trait CertificateFeatures {
  /// Mint certificate. Only course owner can call this function. Student must finish the course
  fn mint_certificate(&mut self, course_id: CourseId, student_id: UserId, skills: Vec<WrapSkill>, media: String);
}

pub trait EnumCertificate {
  /// Get certificate metadata by certificate id
  fn get_certificate_metadata_by_certificate_id(&self, certificate_id: CertificateId) -> Option<CertificateMetadata>;

  /// Get all certicicate by user id
  fn get_all_certificate_by_user_id(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CertificateMetadata>;
}
