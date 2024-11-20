use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::SqlitePool;
use std::env;
mod db;
mod models;
mod auth;
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database at: {}", database_url);

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    db::init_db(&pool).await.expect("Failed to initialize the database");
    println!("Database initialized successfully.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
