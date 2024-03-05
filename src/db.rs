use actix_web::Result;
use sqlx::PgPool;
use std::env;

use crate::dtypes::structs::ErrorMessage; 

pub async fn connect() -> Result<PgPool, ErrorMessage> {
    if let Err(e) = dotenv::dotenv() {
        return Err(ErrorMessage::new(&format!("Failed to read .env file: {}", e)));
    }

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| ErrorMessage::new("DATABASE_URL is not set"))?;

    let pool_result = PgPool::connect(&database_url)
        .await
        .map_err(|e| ErrorMessage::new(&format!("Database connection error: {}", e)))?;

    Ok(pool_result)
}
