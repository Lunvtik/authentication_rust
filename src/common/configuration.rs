use secrecy::SecretString;
use serde::Deserialize;

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, File, FileFormat};

    const FULL: &str = "\
application:
  host: \"127.0.0.1\"
  port: 8000
database:
  host: \"127.0.0.1\"
  port: 5432
  name: \"auth_shop\"
  username: \"auth_app\"
  password: \"db-secret\"
  require_ssl: false
redis:
  host: \"127.0.0.1\"
  port: 6379
session:
  secret: \"session-secret\"
  ttl_seconds: 86400
telemetry:
  log_level: \"info\"
";

    fn settings_from(yaml: &str) -> Result<Settings, config::ConfigError> {
        Config::builder()
            .add_source(File::from_str(yaml, FileFormat::Yaml))
            .build()?
            .try_deserialize()
    }

    #[test]
    fn full_yaml_parses_into_settings() {
        let settings = settings_from(FULL).unwrap();
        assert_eq!(settings.application.port, 8000);
        assert_eq!(settings.database.name, "auth_shop");
        assert!(settings.redis.password.is_none());
    }

    #[test]
    fn missing_required_field_fails() {
        let yaml = FULL.replace("  password: \"db-secret\"\n", "");
        assert!(settings_from(&yaml).is_err());
    }

    #[test]
    fn later_source_overrides_earlier() {
        let settings = Config::builder()
            .add_source(File::from_str(FULL, FileFormat::Yaml))
            .add_source(File::from_str("application:\n  port: 9999\n", FileFormat::Yaml))
            .build()
            .unwrap()
            .try_deserialize::<Settings>()
            .unwrap();
        assert_eq!(settings.application.port, 9999);
    }

    #[test]
    fn environment_parses_known_and_rejects_unknown() {
        assert!(matches!(
            Environment::try_from("local".to_string()),
            Ok(Environment::Local)
        ));
        assert!(matches!(
            Environment::try_from("Production".to_string()),
            Ok(Environment::Production)
        ));
        assert!(Environment::try_from("garbage".to_string()).is_err());
    }
}

#[derive(Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub session: SessionSettings,
    pub telemetry: TelemetrySettings,
}

#[derive(Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: SecretString,
    pub require_ssl: bool,
}

#[derive(Deserialize)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub password: Option<SecretString>,
}

#[derive(Deserialize)]
pub struct SessionSettings {
    pub secret: SecretString,
    pub ttl_seconds: u64,
}

#[derive(Deserialize)]
pub struct TelemetrySettings {
    pub log_level: String,
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!("{other} is not a supported environment.")),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to determine current directory.")
        .join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    config::Config::builder()
        .add_source(config::File::from(base_path.join("base.yaml")))
        .add_source(config::File::from(
            base_path.join(format!("{}.yaml", environment.as_str())),
        ))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__")
                .try_parsing(true),
        )
        .build()?
        .try_deserialize()
}
