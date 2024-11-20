//handlers/user.rs
use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;

pub async fn delete_user_account(
    pool: web::Data<SqlitePool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let user_id = user_id.into_inner(); 
    let transaction = pool.begin().await;

    if let Ok(mut tx) = transaction {
        if sqlx::query("DELETE FROM surveys WHERE user_id = ?")
            .bind(user_id)
            .execute(&mut tx)
            .await
            .is_err()
        {
            return HttpResponse::InternalServerError().body("Failed to delete surveys");
        }

        if sqlx::query("DELETE FROM prompts WHERE user_id = ?")
            .bind(user_id)
            .execute(&mut tx)
            .await
            .is_err()
        {
            return HttpResponse::InternalServerError().body("Failed to delete prompts");
        }

        if sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(user_id)
            .execute(&mut tx)
            .await
            .is_err()
        {
            return HttpResponse::InternalServerError().body("Failed to delete user");
        }

        if tx.commit().await.is_ok() {
            return HttpResponse::Ok().body("User account and data deleted successfully");
        }
    }

    HttpResponse::InternalServerError().finish()
}
