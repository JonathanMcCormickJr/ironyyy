use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityError {
    Argon2(argon2::Error),
    Hash,
    PasswordHash(argon2::password_hash::Error),
}

impl From<argon2::Error> for SecurityError {
    fn from(err: argon2::Error) -> Self {
        SecurityError::Argon2(err)
    }
}

impl From<argon2::password_hash::Error> for SecurityError {
    fn from(err: argon2::password_hash::Error) -> Self {
        SecurityError::PasswordHash(err)
    }
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::Argon2(err) => write!(f, "Argon2 error: {}", err),
            SecurityError::Hash => write!(f, "Hash error"), // This is due to a distinct error case from argon2's `hash`
            SecurityError::PasswordHash(err) => write!(f, "Password hash error: {}", err),
        }
    }
}

impl std::error::Error for SecurityError {}