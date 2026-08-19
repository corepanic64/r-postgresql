use crate::{
    auth::{login::api::login, register::api::register},
    layers::auth::auth_layer,
    users::api::{delete_user, get_user_by_id, get_users, get_users_count, update_user},
};
use axum::{
    Router,
    middleware::{self},
    routing::{get, post},
};
use dotenv::dotenv;
use std::env;

use crate::users::db::DbPool;

pub async fn init_routes(pool: DbPool) {
    dotenv().ok();
    let url = env::var("URL").expect("URL must be stet");

    let users_routes = Router::new()
        .route("/users", get(get_users))
        .route("/users/count", get(get_users_count))
        .route(
            "/users/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        );

    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));

    let app = Router::new()
        .merge(users_routes)
        .layer(middleware::from_fn(auth_layer))
        .merge(auth_routes)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
