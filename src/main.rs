use std::net::TcpListener;

use z2p::startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Unable to bind address");
    startup::run(listener)?.await
}//end main method