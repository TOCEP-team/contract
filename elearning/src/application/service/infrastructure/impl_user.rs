#![allow(clippy::too_many_arguments)]
use std::collections::HashMap;

use crate::models::contract::{ELearningContract, ELearningContractExt};
use crate::models::user::{ImplUser, JsonUser, Roles, UserMetadata};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
/// Implement function for user
impl ImplUser for ELearningContract {
  /// Create a user
  fn create_user(
    &mut self,
    nickname: Option<String>,
    avatar: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
  ) {
    // Check User has exist
    let user_id = env::signer_account_id();
    assert!(!self.check_registration(&user_id), "User has already exists");

    let new_nickname = if let Some(value) = nickname {
      value
    } else {
      env::signer_account_id().to_string().replace(".testnet", "").replace(".near", "")
    };
    // Create the basic information of user
    let user_metadata = UserMetadata {
      user_id: user_id.clone(),
      role: Roles::Subscriber,
      nickname: new_nickname,
      total_credit: 0,
      first_name,
      last_name,
      bio,
      avatar,
      resume: None,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
      courses_owned: 0,
      students: 0,
    };

    // Create a Json of user
    let json_user = JsonUser {
      user_id: user_id.clone(),
      metadata: user_metadata,
      skill: HashMap::new(),
      certificate: Vec::new(),
      courses: Vec::new(),
    };

    // Storage json user in system contract
    self.user_metadata_by_id.insert(&user_id, &json_user);

    // Storage user_id in system contract
    self.subscriber_users.insert(&user_id);
  }

  /// Update the role
  fn update_role(&mut self) -> JsonUser {
    // Only Owned has access
    let user_id = env::signer_account_id();
    assert!(self.check_registration(&user_id), "You don't have access!");

    // Check user had the resume
    let mut user = self.user_metadata_by_id.get(&user_id).unwrap();
    assert!(user.metadata.role == Roles::Subscriber, "You already is Instructor");
    assert!(user.metadata.resume.is_some(), "You must upload your resume!");

    // Change user's role then storage
    user.metadata.role = Roles::Instructor;
    self.user_metadata_by_id.insert(&user_id, &user);
    self.intructor_users.insert(&user_id);

    // return
    user
  }

  /// Update user information
  fn update_user_information(
    &mut self,
    nickname: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
    avatar: Option<String>,
    resume: Option<String>,
  ) -> JsonUser {
    // Check access
    assert!(self.check_registration(&env::signer_account_id()), "You don't have access");

    let mut user = self.user_metadata_by_id.get(&env::signer_account_id()).unwrap();

    // Check attribute. If it have some -> update
    if let Some(n) = nickname {
      user.metadata.nickname = n
    };
    if let Some(f) = first_name {
      user.metadata.first_name = Some(f)
    }

    if let Some(l) = last_name {
      user.metadata.last_name = Some(l)
    }

    if let Some(b) = bio {
      user.metadata.bio = Some(b)
    }

    if let Some(a) = avatar {
      user.metadata.avatar = Some(a)
    }

    if let Some(r) = resume {
      user.metadata.resume = Some(r)
    }

    // Storage time information when user update
    user.metadata.updated_at = env::block_timestamp_ms();

    // Storage the change
    self.user_metadata_by_id.insert(&env::signer_account_id(), &user);

    // Return
    user
  }
}
