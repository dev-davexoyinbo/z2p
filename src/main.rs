use std::net::TcpListener;

use env_logger::Env;
use sqlx::{PgPool};
use z2p::{configuration::get_configuration, startup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Unable to connect to postgres");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Unable to bind address");
    startup::run(listener, connection_pool)?.await
} //end main method
