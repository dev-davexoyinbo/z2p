use actix_web::{web, HttpServer, Responder, App, HttpResponse};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}//end method health_check

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
