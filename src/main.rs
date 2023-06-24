use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;
use z2p::{configuration::get_configuration, startup, get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string.expose_secret())
        .await
        .expect("Unable to connect to postgres");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Unable to bind address");
    startup::run(listener, connection_pool)?.await
} //end main method
