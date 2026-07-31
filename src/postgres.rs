use dotenv::dotenv;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub async fn init_postgres() -> Pool<Postgres> {
    dotenv().ok();
    let datebase_url = env::var("DATABASE_URL").expect("DATEBASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&datebase_url)
        .await
        .expect("Failed to create pool")
}
