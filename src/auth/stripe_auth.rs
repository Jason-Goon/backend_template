use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StripeCustomerRequest {
    pub google_id: String,
}

pub async fn create_stripe_customer(customer_request: web::Json<StripeCustomerRequest>) -> impl Responder {
    let mock_stripe_customer_id = "mock_stripe_customer_id_456";
    HttpResponse::Ok().json(format!("Created Stripe customer ID: {}", mock_stripe_customer_id))
}