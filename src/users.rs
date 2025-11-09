//! # Users
//! Module for managing user accounts. 

use easy_totp::EasyTotp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # User struct
/// Represents a user in the system.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct User {
    /// The username of the user.
    pub username: String,
    /// The unique identifier of the user.
    pub user_uuid: Uuid,
    /// The hashed password of the user.
    pub password_hash: String,
    /// Optional two-factor authentication instance
    pub totp_instance: Option<EasyTotp>,    
}

impl User {
    /// Creates a new user with the given username and password hash.
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            username,
            user_uuid: Uuid::new_v4(),
            password_hash,
            totp_instance: None,
        }
    }
}