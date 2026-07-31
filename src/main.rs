use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod model;
use model::{create_user, get_users, update_user};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
};

use crate::model::{CreateUser, UpdateUser, User};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let datebase_url = env::var("DATABASE_URL").expect("DATEBASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&datebase_url)
        .await
        .expect("Failed to create pool");

    let app = Router::new()
        .route("/users", get(users))
        .route("/user", post(user_create))
        .route("/user/{id}", put(user_update))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:1234").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn users(State(pool): State<sqlx::PgPool>) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    get_users(&pool)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn user_create(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<StatusCode, (StatusCode, String)> {
    create_user(
        &pool,
        CreateUser {
            name: payload.name,
            email: payload.email,
        },
    )
    .await
    .map(|_| StatusCode::CREATED)
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn user_update(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<StatusCode, (StatusCode, String)> {
    update_user(&pool, id, &payload)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
