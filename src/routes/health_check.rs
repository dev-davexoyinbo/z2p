use actix_web::{Responder, HttpResponse};

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
} //end method health_check