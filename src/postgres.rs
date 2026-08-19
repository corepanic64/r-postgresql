use crate::users::db::{DbPool, establish_connection_pool};

pub async fn init_postgres() -> DbPool {
    establish_connection_pool().expect("msg")
}
