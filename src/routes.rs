use axum::{
    Router,
    routing::{get, post},
};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::env;

use crate::auth::{login::api::login, register::api::register};
use crate::users::api::{create_user, delete_user, get_user_by_id, get_users, update_user};

pub async fn init_routes(pool: Pool<Postgres>) {
    dotenv().ok();
    let url = env::var("URL").expect("URL must be stet");

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
