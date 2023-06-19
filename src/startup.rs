use std::{net::TcpListener, io};
use actix_web::{dev::Server, HttpServer, App, web};
use sqlx::PgConnection;

use crate::routes;


pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, io::Error> {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new().route("/health_check", web::get().to(routes::health_check::health_check))
        .route("subscriptions", web::post().to(routes::subscriptions::subscribe))
        .app_data(connection.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}
