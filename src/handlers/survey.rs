use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::survey::Survey;  

pub async fn get_surveys(
    pool: web::Data<SqlitePool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let surveys = sqlx::query_as::<_, Survey>("SELECT id, user_id, result FROM surveys WHERE user_id = ?")
        .bind(user_id.into_inner())
        .fetch_all(pool.get_ref())
        .await;

    match surveys {
        Ok(surveys) => HttpResponse::Ok().json(surveys), 
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn add_survey(
    pool: web::Data<SqlitePool>,
    survey: web::Json<Survey>,
) -> impl Responder {
    println!("Adding survey: user_id={}, result={}", survey.user_id, survey.result);
    let result = sqlx::query(
        "INSERT INTO surveys (user_id, result) VALUES (?, ?)"
    )
    .bind(survey.user_id)
    .bind(&survey.result)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            println!("Survey added successfully");
            HttpResponse::Ok().body("Survey added successfully")
        }
        Err(e) => {
            println!("Error adding survey: {:?}", e);  
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_surveys(
    pool: web::Data<SqlitePool>,
    user_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query("DELETE FROM surveys WHERE user_id = ?")
        .bind(user_id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Surveys deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
