use dotenv::dotenv;
use sqlx::{pool, postgres::PgPoolOptions};
use std::env;

mod model;
use model::{create_user, get_user, get_users};

use axum::{Router, extract::State, http::StatusCode, response::Json, routing::get};
use serde_json::{Value, json};

use crate::model::User;

#[tokio::main]
async fn main() {
    let db_env = dotenv().ok();

    let datebase_url = env::var("DATABASE_URL").expect("DATEBASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&datebase_url)
        .await
        .expect("Failed to create pool");

    let app = Router::new().route("/users", get(users)).with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // create_user(&pool, "shaxzod", "shaxzod@gmail.com")
    //     .await
    //     .unwrap();
    // let user = get_user(&pool, 2).await.unwrap();
    // println!("User : {:#?}", users);
    // println!("User operations completed.");
}

async fn users(State(pool): State<sqlx::PgPool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    get_users(&pool)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
