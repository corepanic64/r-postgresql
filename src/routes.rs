use crate::api_requsts::{create_user, delete_user, get_user_by_id, get_users, update_user};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::env;

pub async fn init_routes(pool: Pool<Postgres>) {
    dotenv().ok();
    let url = env::var("URL").expect("URL must be stet");

    let app = Router::new()
        .route("/users", get(get_users))
        .route("/user", post(create_user))
        .route("/user/{id}", get(get_user_by_id))
        .route("/user/{id}", put(update_user))
        .route("/user/{id}", delete(delete_user))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
