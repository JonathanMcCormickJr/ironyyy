//! Time-based One-Time Password (TOTP) utilities

use super::*;
use easy_totp::{EasyTotp, QRColorMode, TerminalQRSize};

/// Generate a TOTP instance for a given username
/// 
/// # Arguments
/// * `username` - The username for which to generate the TOTP
/// # Returns
/// * `Result<EasyTotp, SecurityError>` - The generated TOTP instance or an error
/// # Errors
/// * `SecurityError::TryRngCore` - If there was an error generating the TOTP
/// # Examples
/// ```rust
/// use ironyyy::security::totp::generate_totp;
/// let totp = generate_totp("example_user").unwrap();
/// ```
pub fn generate_totp(username: &str) -> Result<EasyTotp, SecurityError> {
    let totp = EasyTotp::new(Some("Ironyyy".to_string()), username.to_string()).map_err(|_| SecurityError::TryRngCore)?;
    Ok(totp)
}

/// Generate the onboarding QR code for a given TOTP instance
/// # Arguments
/// * `et` - The TOTP instance for which to generate the QR code
/// # Returns
/// * `Result<Vec<String>, SecurityError>` - The generated QR code lines or an error
/// # Errors
/// * `SecurityError::Totp` - If there was an error generating the QR code
/// # Examples
/// ```rust
/// use ironyyy::security::totp::{generate_totp, onboard_totp};
/// let totp = generate_totp("example_user").unwrap();
/// let qr_code = onboard_totp(&totp).unwrap();
/// for line in qr_code {
///     println!("{}", line);
/// }
/// ```
pub fn onboard_totp(et: &EasyTotp) -> Result<Vec<String>, SecurityError> {
    Ok(et.qr_text(TerminalQRSize::Full, QRColorMode::Inverted).map_err(|_| SecurityError::Totp)?)

}

/// Verify a TOTP code against a given TOTP instance
/// # Arguments
/// * `et` - The TOTP instance to verify against
/// * `code` - The TOTP code to verify
/// # Returns
/// * `Result<bool, SecurityError>` - Whether the code is valid or an error
/// # Errors
/// * `SecurityError::Totp` - If there was an error generating the token
/// # Examples
/// ```rust
/// use ironyyy::security::totp::{generate_totp, verify_totp};
/// let totp = generate_totp("example_user").unwrap();
/// let code = totp.generate_token().unwrap();
/// let is_valid = verify_totp(&totp, &code).unwrap();
/// assert!(is_valid);
/// ```
pub fn verify_totp(et: &EasyTotp, code: &str) -> Result<bool, SecurityError> {
    Ok(et.generate_token().map_err(|_| SecurityError::Totp)? == code)
}