use std::net::TcpListener;

use auth_rust::common::configuration::get_configuration;
use auth_rust::common::db::get_connection_pool;
use auth_rust::common::telemetry::init_telemetry;
use auth_rust::startup::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration()?;
    init_telemetry("auth_rust", &configuration.telemetry.log_level);

    let db_pool = get_connection_pool(&configuration.database);
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, db_pool)?.await?;
    Ok(())
}
