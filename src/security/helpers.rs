use super::*;

/// # Argon2 Parameters
/// Returns Argon2 parameters configured for secure password hashing and key derivation.
fn argon2_params() -> Result<Params, argon2::Error> {
    Ok(Params::new(
        65536, // memory cost in KiB
        8,    // time cost
        1,    // parallelism
        None, // output length (default is 32 bytes)
    )?)
}

/// # Argon Instance
/// Returns a configured Argon2 instance for password hashing and key derivation.
pub fn argon2_instance<'a>() -> Result<Argon2<'a>, SecurityError> {
    let params = argon2_params()?;
    Ok(Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        params,
    ))
}