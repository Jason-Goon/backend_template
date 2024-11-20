use actix_web::{web, HttpResponse, Responder};
use sqlx::{SqlitePool, sqlite::Sqlite};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PromptInput {
    user_id: i32,
    prompt: String,
}

pub async fn delete_prompts(
    pool: web::Data<SqlitePool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query("DELETE FROM prompts WHERE user_id = ?")
        .bind(user_id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Prompts deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Handler to retrieve all prompts for a user
pub async fn get_prompts(
    pool: web::Data<SqlitePool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let prompts = sqlx::query_as::<Sqlite, (i32, String)>("SELECT id, prompt FROM prompts WHERE user_id = ?")
        .bind(user_id.into_inner())
        .fetch_all(pool.get_ref())
        .await;

    match prompts {
        Ok(prompts) => HttpResponse::Ok().json(prompts),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Handler to add a new prompt for a user
pub async fn add_prompt(
    pool: web::Data<SqlitePool>,
    prompt: web::Json<PromptInput>,
) -> impl Responder {
    let result = sqlx::query(
        "INSERT INTO prompts (user_id, prompt) VALUES (?, ?)"
    )
    .bind(prompt.user_id)
    .bind(&prompt.prompt)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Prompt added successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
