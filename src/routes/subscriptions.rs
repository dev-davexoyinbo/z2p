use actix_web::{web, Responder, HttpResponse};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<FormData>, _connection: web::Data<PgConnection>) -> impl Responder {
    HttpResponse::Ok().finish()
}
