use anyhow::Context;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{Error, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use secrecy::{ExposeSecret, SecretString};

use crate::features::authentication::error::AuthError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_accepts_correct_password() {
        let phc = compute_password_hash("correct horse battery staple").unwrap();
        let result = verify_password_hash(
            SecretString::from(phc),
            SecretString::from("correct horse battery staple"),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn verify_rejects_wrong_password() {
        let phc = compute_password_hash("correct horse battery staple").unwrap();
        let result = verify_password_hash(
            SecretString::from(phc),
            SecretString::from("wrong-password"),
        );
        assert!(matches!(result, Err(AuthError::InvalidCredentials(_))));
    }

    #[test]
    fn verify_rejects_malformed_phc() {
        let result = verify_password_hash(
            SecretString::from("not-a-phc-string"),
            SecretString::from("whatever"),
        );
        assert!(matches!(result, Err(AuthError::UnexpectedError(_))));
    }

    #[test]
    fn same_password_yields_different_hashes() {
        let a = compute_password_hash("same-password").unwrap();
        let b = compute_password_hash("same-password").unwrap();
        assert_ne!(a, b);
    }
}

pub fn compute_password_hash(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hasher = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15_000, 2, 1, None).expect("Argon2-Parameter sind fest und gültig"),
    );
    let phc = hasher.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(phc)
}

pub fn verify_password_hash(
    expected_password_hash: SecretString,
    password_candidate: SecretString,
) -> Result<(), AuthError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")
        .map_err(AuthError::UnexpectedError)?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}
