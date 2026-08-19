mod auth;
mod layers;
mod model;
mod postgres;
mod routes;
mod users;

use crate::{postgres::init_postgres, routes::init_routes};

#[tokio::main]
async fn main() {
    let pool = init_postgres().await;
    init_routes(pool).await;
}
