//! # Security Module
//! This module handles security features such as password hashing and encryption

mod errors;
mod helpers;

use crate::security::errors::SecurityError;

use argon2::{
    Params,
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rand_core::{ OsRng, TryRngCore };
use uuid::Uuid;


fn argon2_params() -> Result<Params, argon2::Error> {
    Ok(Params::new(
        65536, // memory cost in KiB
        8,    // time cost
        1,    // parallelism
        None, // output length (default is 32 bytes)
    )?)
}

pub struct Argon2Hash(Vec<u8>);

impl Argon2Hash {
    pub fn new(password: &str, salt: Uuid) -> Result<Self, SecurityError> {
        let params = argon2_params()?;
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            params,
        );

        let salt_string = SaltString::encode_b64(salt.as_bytes()).unwrap();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt_string)?;

        Ok(Argon2Hash(password_hash.hash.ok_or(SecurityError::Hash)?.as_bytes().to_vec()))
    }
}

pub struct Argon2EncryptionKey([u8; 64]);

fn test_undocumented_function() {}