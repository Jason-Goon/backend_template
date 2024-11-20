use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Prompt {
    pub id: i32,
    pub user_id: i32,
    pub prompt: String,
    pub created_at: String,  
}
