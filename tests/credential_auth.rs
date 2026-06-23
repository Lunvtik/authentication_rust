use auth_rust::common::db::get_connection_pool;
use auth_rust::features::authentication::credentials::Credentials;
use auth_rust::features::authentication::error::AuthError;
use auth_rust::features::authentication::hashing::compute_password_hash;
use auth_rust::features::authentication::validate::validate_credentials;
use secrecy::SecretString;
use sqlx::PgPool;
use uuid::Uuid;

mod common;

async fn insert_user(pool: &PgPool, username: &str, password: &str) -> Uuid {
    let user_id = Uuid::new_v4();
    let phc = compute_password_hash(password).unwrap();
    sqlx::query!(
        r#"INSERT INTO users (user_id, username, email, password_hash) VALUES ($1, $2, $3, $4)"#,
        user_id,
        username,
        format!("{username}@example.com"),
        phc,
    )
    .execute(pool)
    .await
    .expect("Failed to insert test user.");
    user_id
}

#[tokio::test]
async fn valid_credentials_are_accepted() {
    let pool = get_connection_pool(&common::test_db_settings());
    let username = format!("user-{}", Uuid::new_v4());
    let user_id = insert_user(&pool, &username, "correct horse battery staple").await;

    let result = validate_credentials(
        Credentials {
            username,
            password: SecretString::from("correct horse battery staple"),
        },
        &pool,
    )
    .await;

    assert_eq!(result.unwrap(), user_id);
}

#[tokio::test]
async fn wrong_password_is_rejected() {
    let pool = get_connection_pool(&common::test_db_settings());
    let username = format!("user-{}", Uuid::new_v4());
    insert_user(&pool, &username, "correct horse battery staple").await;

    let result = validate_credentials(
        Credentials {
            username,
            password: SecretString::from("wrong-password"),
        },
        &pool,
    )
    .await;

    assert!(matches!(result, Err(AuthError::InvalidCredentials(_))));
}

#[tokio::test]
async fn unknown_user_is_rejected() {
    let pool = get_connection_pool(&common::test_db_settings());

    let result = validate_credentials(
        Credentials {
            username: format!("ghost-{}", Uuid::new_v4()),
            password: SecretString::from("whatever"),
        },
        &pool,
    )
    .await;

    assert!(matches!(result, Err(AuthError::InvalidCredentials(_))));
}
