use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn connect_database() -> Result<PgPool, sqlx::Error> {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is missing");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to PostgreSQL");

    Ok(pool)
}