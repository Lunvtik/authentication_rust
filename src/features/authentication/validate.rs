use anyhow::Context;
use secrecy::SecretString;
use sqlx::PgPool;
use uuid::Uuid;

use crate::common::telemetry::spawn_blocking_with_tracing;
use crate::features::authentication::credentials::Credentials;
use crate::features::authentication::error::AuthError;
use crate::features::authentication::hashing::verify_password_hash;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::features::authentication::hashing::compute_password_hash;

    #[tokio::test]
    async fn valid_credentials_return_user_id() {
        let user_id = Uuid::from_u128(42);
        let phc = compute_password_hash("correct horse battery staple").unwrap();
        let stored = Some((user_id, SecretString::from(phc)));
        let result =
            verify_credentials_logic(stored, SecretString::from("correct horse battery staple"))
                .await;
        assert_eq!(result.unwrap(), user_id);
    }

    #[tokio::test]
    async fn wrong_password_is_rejected() {
        let phc = compute_password_hash("correct horse battery staple").unwrap();
        let stored = Some((Uuid::from_u128(42), SecretString::from(phc)));
        let result = verify_credentials_logic(stored, SecretString::from("wrong-password")).await;
        assert!(matches!(result, Err(AuthError::InvalidCredentials(_))));
    }

    #[tokio::test]
    async fn unknown_user_is_rejected() {
        let result = verify_credentials_logic(None, SecretString::from("any-password")).await;
        assert!(matches!(result, Err(AuthError::InvalidCredentials(_))));
    }
}

pub async fn verify_credentials_logic(
    stored: Option<(Uuid, SecretString)>,
    password_candidate: SecretString,
) -> Result<Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = SecretString::from(
        "$argon2id$v=19$m=15000,t=2,p=1$gZiV/M1gPc22ElAH/Jh1Hw$CWOrkoo7oJBQ/iyh7uJOLO2aLEfrHwTWl1SAxTOzRno",
    );

    if let Some((stored_user_id, stored_password_hash)) = stored {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(expected_password_hash, password_candidate)
    })
    .await
    .context("Failed to spawn blocking task.")
    .map_err(AuthError::UnexpectedError)??;

    user_id.ok_or_else(|| AuthError::InvalidCredentials(anyhow::anyhow!("Unknown username.")))
}

#[tracing::instrument(name = "Get stored credentials", skip(pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<(Uuid, SecretString)>, anyhow::Error> {
    let row = sqlx::query!(
        r#"SELECT user_id, password_hash FROM users WHERE username = $1"#,
        username,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to query stored credentials.")?
    .map(|row| (row.user_id, SecretString::from(row.password_hash)));
    Ok(row)
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<Uuid, AuthError> {
    let stored = get_stored_credentials(&credentials.username, pool)
        .await
        .map_err(AuthError::UnexpectedError)?;
    verify_credentials_logic(stored, credentials.password).await
}
