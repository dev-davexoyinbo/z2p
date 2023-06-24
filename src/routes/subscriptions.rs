use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, db_pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name= %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    // let request_id = Uuid::new_v4();
    // // Spans, like logs, have an associated level
    // // `info_span` creates a span at the info-level
    // let request_span = tracing::info_span!(
    // "Adding a new subscriber.",
    // %request_id,
    // subscriber_email = %form.email,
    // subscriber_name= %form.name
    // );
    // // Using `enter` in an async function is a recipe for disaster!
    // // Bear with me for now, but don't do this at home.
    // // See the following section on `Instrumenting Futures`
    // let _request_span_guard = request_span.enter();

    // // We do not call `.enter` on query_span!
    // // `.instrument` takes care of it at the right moments
    // // in the query future lifetime
    // let query_span = tracing::info_span!("Saving new subscriber details in the database");

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool.get_ref())
    // .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("A new subscriber details have been saved",);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query {:?}.", e);
            HttpResponse::InternalServerError().finish()
        }
    }
} //end function subscript
