use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use sqlx::types::Uuid;
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>) -> HttpResponse {

        let request_id = Uuid::new_v4();
        log::info!("request_id {} - Adding '{}' '{}' as a new subscriber", request_id, form.email, form.name);
        log::info!("request_id {} - Saving a new subscriber", request_id);

        match sqlx::query!(
            r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            form.email,
            form.name,
            Utc::now())
            .execute(pool.get_ref())
            .await
            {
                Ok(_) => {
                    log::info!("request_id {} - Saved a new subscriber", request_id);
                    HttpResponse::Ok().finish()
                },
                Err(e) => {
                    log::error!("request_id {} - Failed to execture query: {:?}", request_id, e);
                    HttpResponse::InternalServerError().finish()
                }
            }
}