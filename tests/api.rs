use actix_web::{web, test, App, http::StatusCode};
use serde_json::json;
use sqlx::{SqlitePool, migrate::Migrator};
use quitsocial::configure_routes;

static MIGRATOR: Migrator = sqlx::migrate!();

async fn setup_project_db() -> SqlitePool {
    let db_path = "sqlite://./test_database.db";  
    let pool = SqlitePool::connect(db_path).await.unwrap();
    MIGRATOR.run(&pool).await.unwrap(); 
    pool
}

#[actix_rt::test]
async fn test_authenticate_with_google() {
    let pool = setup_project_db().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/google")
        .set_json(&json!({ "id_token": "mock_token" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let user_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(user_count.0, 1);
}

#[actix_rt::test]
async fn test_create_and_retrieve_survey() {
    let pool = setup_project_db().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/google")
        .set_json(&json!({ "id_token": "mock_token" }))
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::post()
        .uri("/api/add-survey")
        .set_json(&json!({ "user_id": 1, "result": "Survey for User 1" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri("/api/surveys/1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let surveys: Vec<serde_json::Value> = test::read_body_json(resp).await;

    assert_eq!(surveys.len(), 1, "Expected exactly 1 survey but found {}", surveys.len());
    assert_eq!(surveys[0]["result"], "Survey for User 1");
}

#[actix_rt::test]
async fn test_create_and_retrieve_prompt() {
    let pool = setup_project_db().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/google")
        .set_json(&json!({ "id_token": "mock_token" }))
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::post()
        .uri("/api/add-prompt")
        .set_json(&json!({ "user_id": 1, "prompt": "Prompt for User 1" }))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri("/api/prompts/1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let prompts: Vec<serde_json::Value> = test::read_body_json(resp).await;

    assert_eq!(prompts.len(), 1, "Expected exactly 1 prompt but found {}", prompts.len());
    assert_eq!(prompts[0]["prompt"], "Prompt for User 1");
}

#[actix_rt::test]
async fn test_delete_user_and_associated_data() {
    let pool = setup_project_db().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/google")
        .set_json(&json!({ "id_token": "mock_token" }))
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::post()
        .uri("/api/add-survey")
        .set_json(&json!({ "user_id": 1, "result": "Survey for User 1" }))
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::post()
        .uri("/api/add-prompt")
        .set_json(&json!({ "user_id": 1, "prompt": "Prompt for User 1" }))
        .to_request();
    test::call_service(&app, req).await;

    let req = test::TestRequest::delete()
        .uri("/api/delete-user/1")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let survey_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM surveys WHERE user_id = 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(survey_count.0, 0, "Expected no surveys after user deletion");

    let prompt_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM prompts WHERE user_id = 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(prompt_count.0, 0, "Expected no prompts after user deletion");
}
