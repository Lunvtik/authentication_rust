use auth_rust::common::configuration::DatabaseSettings;
use secrecy::SecretString;

pub fn test_db_settings() -> DatabaseSettings {
    let password =
        std::env::var("APP_DATABASE__PASSWORD").unwrap_or_else(|_| "test".to_string());
    DatabaseSettings {
        host: "127.0.0.1".into(),
        port: 5432,
        name: "auth_shop".into(),
        username: "auth_app".into(),
        password: SecretString::from(password),
        require_ssl: false,
    }
}
