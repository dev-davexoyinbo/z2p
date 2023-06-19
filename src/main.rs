use std::net::TcpListener;

use z2p::{startup, configuration::get_configuration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    println!("Configuration: {:#?}", configuration);
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Unable to bind address");
    startup::run(listener)?.await
}//end main method