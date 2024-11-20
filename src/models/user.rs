// models/user.rs 
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub google_id: String,
    pub stripe_customer_id: String,
}
