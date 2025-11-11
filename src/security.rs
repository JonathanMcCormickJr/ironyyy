//! # Security Module
//! This module handles security features such as password hashing and encryption

mod errors;
mod helpers;
pub mod totp;

use crate::security::errors::SecurityError;
use self::helpers::{ argon2_instance };

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};
use argon2::{
    Params,
    password_hash::{
        PasswordHasher, SaltString
    },
    Argon2
};
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

/// # Argon2 Hash
/// 
/// Represents a hashed password using the Argon2id algorithm.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Argon2Hash(pub Vec<u8>);

impl Argon2Hash {
    /// Creates a new Argon2Hash from a plaintext password and a salt.
    /// 
    /// # Arguments
    /// * `password` - The plaintext password to hash.
    /// * `salt` - A UUID used as the salt for hashing.
    /// # Returns
    /// * `Result<Argon2Hash, SecurityError>` - The resulting Argon2Hash or an error.
    /// 
    /// # Examples
    /// ```rust
    /// use ironyyy::security::Argon2Hash;
    /// use uuid::Uuid;
    /// let salt = Uuid::new_v4();
    /// let hash = Argon2Hash::new("my_secure_password", salt).unwrap();
    /// ```
    pub fn new(password: &str, salt: Uuid) -> Result<Self, SecurityError> {
        let argon2 = argon2_instance()?;

        let salt_string = SaltString::encode_b64(salt.as_bytes())?;
        let password_hash = argon2.hash_password(password.as_bytes(), &salt_string)?;

        Ok(Argon2Hash(password_hash.hash.ok_or(SecurityError::Hash)?.as_bytes().to_vec()))
    }

    /// Verifies a plaintext password against the stored hash.
    /// 
    /// # Arguments
    /// * `password` - The plaintext password to verify.
    /// * `salt` - The UUID salt used during hashing.
    /// # Returns
    /// * `Result<bool, SecurityError>` - True if the password matches, false otherwise.
    /// # Examples
    /// ```rust
    /// use ironyyy::security::Argon2Hash;
    /// use uuid::Uuid;
    /// let salt = Uuid::new_v4();
    /// let hash = Argon2Hash::new("my_secure_password", salt).unwrap();
    /// let is_valid = hash.verify_password("my_secure_password", salt).unwrap();
    /// assert!(is_valid);
    pub fn verify_password(&self, password: &str, salt: Uuid) -> Result<bool, SecurityError> {
        let reference_hash = self.0.clone();
        let argon2 = argon2_instance()?;
        let salt_string = SaltString::encode_b64(salt.as_bytes()).unwrap();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt_string)?.hash.ok_or(SecurityError::Hash)?.as_bytes().to_vec();

        Ok(reference_hash == password_hash)
    }
}

/// # Argon2 password-derived encryption key
/// Represents an encryption key derived from a password using Argon2id.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Argon2EncryptionKey(pub [u8; 32]);

impl Argon2EncryptionKey {
    /// Creates a new Argon2EncryptionKey from a plaintext password and a salt.
    /// 
    /// # Arguments
    /// * `password` - The plaintext password to derive the key from.
    /// * `salt` - A UUID used as the salt for key derivation.
    /// # Returns
    /// * `Result<Argon2EncryptionKey, SecurityError>` - The resulting Argon2EncryptionKey or an error.
    /// # Examples
    /// ```rust
    /// use ironyyy::security::Argon2EncryptionKey;
    /// use uuid::Uuid;
    /// let salt = Uuid::new_v4();
    /// let key = Argon2EncryptionKey::new("my_secure_password", salt).unwrap();
    /// assert_eq!(key.0.len(), 32);
    /// assert_eq!(key, Argon2EncryptionKey::new("my_secure_password", salt).unwrap());
    /// ```
    pub fn new(password: &str, salt: Uuid) -> Result<Self, SecurityError> {
        let mut output_key_material = [0u8; 32];
        argon2_instance()?.hash_password_into(password.as_bytes(), salt.as_bytes(), &mut output_key_material)?;
        Ok(Argon2EncryptionKey(output_key_material))
    }
}

/// # Ciphertext
/// Symmetrically encrypted ciphertext, represented as a vector of bytes.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Ciphertext(pub Vec<u8>);

impl Ciphertext {
    /// Encrypts plaintext using the provided Argon2EncryptionKey and nonce.
    /// 
    /// # Arguments
    /// * `plaintext` - The plaintext string to encrypt.
    /// * `encryption_key` - The Argon2EncryptionKey used for encryption.
    /// * `nonce` - A 12-byte nonce for AES-GCM encryption. CHANGE THIS FOR EVERY CALL TO ENCRYPT, but also store it alongside the ciphertext for decryption.
    /// # Returns
    /// * `Result<Ciphertext, SecurityError>` - The resulting Ciphertext or an error.
    /// # Examples
    /// ```rust
    /// use ironyyy::security::{ Argon2EncryptionKey, Ciphertext };
    /// use rand_core::{ OsRng, TryRngCore };
    /// use uuid::Uuid;
    /// let salt = Uuid::new_v4();
    /// let key = Argon2EncryptionKey::new("my_secure_password", salt).unwrap();
    /// let mut nonce = [0u8; 12];
    /// OsRng.try_fill_bytes(&mut nonce).unwrap();
    /// let ciphertext = Ciphertext::encrypt("Sensitive data", &key, &nonce).unwrap();
    /// assert_eq!(ciphertext, Ciphertext::encrypt("Sensitive data", &key, &nonce).unwrap());
    /// ```
    pub fn encrypt(plaintext: &str, encryption_key: &Argon2EncryptionKey, nonce: &[u8; 12]) -> Result<Self, SecurityError> {
        let key: &Key<Aes256Gcm> = &encryption_key.0.into();

        let cipher = Aes256Gcm::new(&key);
        let ciphertext = cipher.encrypt(
            &(*nonce).into(),
            plaintext.as_bytes(),
        )?;
        Ok(Ciphertext(ciphertext))
    }

    /// Decrypts the ciphertext using the provided Argon2EncryptionKey and nonce.
    /// 
    /// # Arguments
    /// * `encryption_key` - The Argon2EncryptionKey used for decryption.
    /// * `nonce` - The 12-byte nonce used during encryption.
    /// # Returns
    /// * `Result<String, SecurityError>` - The resulting plaintext string or an error.
    /// # Examples
    /// ```rust
    /// use ironyyy::security::{ Argon2EncryptionKey, Ciphertext };
    /// use rand_core::{ OsRng, TryRngCore };
    /// use uuid::Uuid;
    /// let salt = Uuid::new_v4();
    /// let key = Argon2EncryptionKey::new("my_secure_password", salt).unwrap();
    /// let mut nonce = [0u8; 12];
    /// OsRng.try_fill_bytes(&mut nonce).unwrap();
    /// let ciphertext = Ciphertext::encrypt("Sensitive data", &key, &nonce).unwrap();
    /// let plaintext = ciphertext.decrypt(&key, &nonce).unwrap();
    /// assert_eq!(plaintext, "Sensitive data");
    /// ```
    pub fn decrypt(&self, encryption_key: &Argon2EncryptionKey, nonce: &[u8; 12]) -> Result<String, SecurityError> {
        let key: &Key<Aes256Gcm> = &encryption_key.0.into();
        let cipher = Aes256Gcm::new(&key);
        let plaintext_bytes = cipher.decrypt(
            &(*nonce).into(),
            self.0.as_ref(),
        )?;
        let plaintext = String::from_utf8(plaintext_bytes)?;
        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::{ OsRng, TryRngCore };
    use uuid::Uuid;

    #[test]
    fn test_argon2_hash_and_verify() {
        let salt = Uuid::new_v4();
        let password = "my_secure_password";
        let hash = Argon2Hash::new(password, salt).unwrap();
        let is_valid = hash.verify_password(password, salt).unwrap();
        assert!(is_valid);
        let is_invalid = hash.verify_password("wrong_password", salt).unwrap();
        assert!(!is_invalid);
    }

    #[test]
    fn test_argon2_encryption_key_determinism() {
        let salt = Uuid::new_v4();
        let password = "my_secure_password";
        let key1 = Argon2EncryptionKey::new(password, salt).unwrap();
        let key2 = Argon2EncryptionKey::new(password, salt).unwrap();
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_ciphertext_encrypt_decrypt() {
        let salt = Uuid::new_v4();
        let password = "my_secure_password";
        let key = Argon2EncryptionKey::new(password, salt).unwrap();

        let mut nonce = [0u8; 12];
        OsRng.try_fill_bytes(&mut nonce).unwrap();

        let plaintext = "Sensitive data";
        let ciphertext = Ciphertext::encrypt(plaintext, &key, &nonce).unwrap();
        let decrypted_plaintext = ciphertext.decrypt(&key, &nonce).unwrap();

        assert_eq!(plaintext, decrypted_plaintext);
    }
}