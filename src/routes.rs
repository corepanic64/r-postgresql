use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::env;
use tower::{Layer, ServiceBuilder};

use crate::users::api::{create_user, delete_user, get_user_by_id, get_users, update_user};
use crate::{
    auth::{login::api::login, register::api::register},
    layers::auth::auth_layer,
};

pub async fn init_routes(pool: Pool<Postgres>) {
    dotenv().ok();
    let url = env::var("URL").expect("URL must be stet");

    let users_routes = Router::new()
        .route("/users", get(get_users).post(create_user))
        .route_layer(middleware::from_fn(my_user_layer));

    let alohida = Router::new().route(
        "/users/{id}",
        get(get_user_by_id).put(update_user).delete(delete_user),
    );
    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login));
    let app = Router::new()
        .merge(users_routes)
        .merge(auth_routes)
        .merge(alohida)
        .layer(middleware::from_fn(auth_layer))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(&url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn my_layer(request: Request, next: Next) -> Response {
    println!("OBSHIY LAYERDAN OTDIM");
    next.run(request).await
}
async fn my_user_layer(request: Request, next: Next) -> Response {
    println!("USERS LAYERDAN OTDIM");
    next.run(request).await
}
