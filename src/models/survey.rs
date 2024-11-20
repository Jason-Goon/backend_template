use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Survey {
    #[serde(skip_deserializing)]  
    pub id: Option<i32>,        
    pub user_id: i32,
    pub result: String,
}
