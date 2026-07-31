use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod model;
use model::{create_user, get_user, get_users};

#[tokio::main]
async fn main() {
    let db_env = dotenv().ok();

    let datebase_url = env::var("DATABASE_URL").expect("DATEBASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&datebase_url)
        .await
        .expect("Failed to create pool");

    // create_user(&pool, "shaxzod", "shaxzod@gmail.com")
    //     .await
    //     .unwrap();
    // let user = get_user(&pool, 2).await.unwrap();
    let users = get_users(&pool).await.unwrap();
    println!("User : {:#?}", users);
    println!("User operations completed.");
}
