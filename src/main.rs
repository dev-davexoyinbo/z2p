use std::{net::TcpListener, time::Duration};

use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, PgPool};
use z2p::{configuration::get_configuration, get_subscriber, init_subscriber, startup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(2))
        .connect_lazy(&connection_string.expose_secret())
        .expect("Failed to create Postgres connection pool.");
    let listener = TcpListener::bind(format!(
        "{}:{}",
        &configuration.application.host, &configuration.application.port
    ))
    .expect("Unable to bind address");
    startup::run(listener, connection_pool)?.await
} //end main method
