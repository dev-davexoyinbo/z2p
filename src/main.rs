use actix_web::{web, HttpRequest, HttpServer, Responder, App, HttpResponse};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}", &name)
} //end function greet

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}//end method health_check

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
