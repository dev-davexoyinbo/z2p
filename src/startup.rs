use std::{net::TcpListener, io};
use actix_web::{dev::Server, HttpServer, App, web};

use crate::routes;


pub fn run(listener: TcpListener) -> Result<Server, io::Error> {
    let server = HttpServer::new(|| {
        App::new().route("/health_check", web::get().to(routes::health_check::health_check))
        .route("subscriptions", web::post().to(routes::subscriptions::subscribe))
    })
        .listen(listener)?
        .run();

    Ok(server)
}
