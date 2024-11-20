use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;  
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleAuthRequest {
    pub id_token: String,
}

pub async fn authenticate_with_google(
    pool: web::Data<SqlitePool>, 
    _auth_request: web::Json<GoogleAuthRequest>
) -> impl Responder {
    
    let google_id = "mock_google_user_id_123";  
    let user = sqlx::query!(
        "INSERT INTO users (google_id, stripe_customer_id) VALUES (?, ?) RETURNING id",
        google_id,
        "mock_stripe_customer_id"
    )
    .fetch_one(pool.get_ref()) 
    .await;

    match user {
        Ok(user) => HttpResponse::Ok().json(format!("Authenticated Google user ID: {}", user.id)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
