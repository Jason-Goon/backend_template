use actix_web::web;
use crate::auth::{google_auth::authenticate_with_google, stripe_auth::create_stripe_customer};
use crate::handlers::{add_survey, get_surveys, get_prompts, add_prompt, delete_surveys, delete_prompts, delete_user_account};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/auth/google", web::post().to(authenticate_with_google))
            .route("/auth/stripe", web::post().to(create_stripe_customer))
            .route("/surveys/{user_id}", web::get().to(get_surveys)) // Now includes user_id
            .route("/add-survey", web::post().to(add_survey))
            .route("/delete-surveys/{user_id}", web::delete().to(delete_surveys))
            .route("/prompts/{user_id}", web::get().to(get_prompts))
            .route("/add-prompt", web::post().to(add_prompt))
            .route("/delete-prompts/{user_id}", web::delete().to(delete_prompts))
            .route("/delete-user/{user_id}", web::delete().to(delete_user_account))
    );
}
