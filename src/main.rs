mod api_requsts;
mod db_requests;
mod model;
mod postgres;
mod routes;

use crate::{postgres::init_postgres, routes::init_routes};

#[tokio::main]
async fn main() {
    let pool = init_postgres().await;
    init_routes(pool).await;
}
