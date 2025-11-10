use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SecurityError {
    AesGcm(aes_gcm::Error),
    Argon2(argon2::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    Hash,
    PasswordHash(argon2::password_hash::Error),
}

impl From<aes_gcm::Error> for SecurityError {
    fn from(err: aes_gcm::Error) -> Self {
        SecurityError::AesGcm(err)
    }
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

impl From<std::string::FromUtf8Error> for SecurityError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        SecurityError::FromUtf8Error(err)
    }
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::AesGcm(err) => write!(f, "AES-GCM error: {}", err),
            SecurityError::Argon2(err) => write!(f, "Argon2 error: {}", err),
            SecurityError::FromUtf8Error(err) => write!(f, "UTF-8 conversion error: {}", err),
            SecurityError::Hash => write!(f, "Hash error"), // This is due to a distinct error case from argon2's `hash`
            SecurityError::PasswordHash(err) => write!(f, "Password hash error: {}", err),
        }
    }
}

impl std::error::Error for SecurityError {}