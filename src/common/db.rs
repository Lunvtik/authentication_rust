use secrecy::ExposeSecret;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::PgPool;

use crate::common::configuration::DatabaseSettings;

pub fn connect_options(settings: &DatabaseSettings) -> PgConnectOptions {
    let ssl_mode = if settings.require_ssl {
        PgSslMode::Require
    } else {
        PgSslMode::Prefer
    };
    PgConnectOptions::new()
        .host(&settings.host)
        .port(settings.port)
        .username(&settings.username)
        .password(settings.password.expose_secret())
        .database(&settings.name)
        .ssl_mode(ssl_mode)
}

pub fn get_connection_pool(settings: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(connect_options(settings))
}
