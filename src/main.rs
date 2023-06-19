use std::net::TcpListener;

use sqlx::{Connection, PgConnection};
use z2p::{configuration::get_configuration, startup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Unable to connect to postgres");
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Unable to bind address");
    startup::run(listener, connection)?.await
} //end main method
