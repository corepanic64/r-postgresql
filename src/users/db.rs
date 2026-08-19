use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use dotenv::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Debug)]
pub enum DbError {
    ConnectionError(String),
    PoolError(PoolError),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            DbError::PoolError(e) => write!(f, "Pool error: {}", e),
        }
    }
}

impl std::error::Error for DbError {}

pub fn establish_connection_pool() -> Result<DbPool, DbError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").map_err(|_| {
        DbError::ConnectionError("DATABASE_URL must be set in .env file".to_string())
    })?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .min_idle(Some(2))
        .test_on_check_out(true)
        .build(manager)
        .map_err(DbError::PoolError)
}

pub fn get_connection(pool: &DbPool) -> Result<DbConnection, DbError> {
    pool.get().map_err(DbError::PoolError)
}

use chrono::Utc;
use diesel::prelude::*;

use crate::users::model::{UpdateUser, User, UserModified};
use crate::users::schema::users;

pub type DbResult<T> = Result<T, diesel::result::Error>;

pub fn get_user_by_id(conn: &mut DbConnection, user_id: i32) -> DbResult<UserModified> {
    users::table
        .find(user_id)
        .select(UserModified::as_select())
        .first(conn)
}

pub fn get_user_by_email(conn: &mut DbConnection, email: String) -> DbResult<User> {
    users::table
        .select(User::as_select())
        .filter(users::email.eq(email))
        .get_result(conn)
}

pub fn list_users(
    conn: &mut DbConnection,
    page: i64,
    per_page: i64,
) -> DbResult<Vec<UserModified>> {
    let offset = (page - 1) * per_page;

    users::table
        .select(UserModified::as_select())
        .order(users::created_at.desc())
        .limit(per_page)
        .offset(offset)
        .load(conn)
}

pub fn update_user(
    conn: &mut DbConnection,
    user_id: i32,
    mut chages: UpdateUser,
) -> DbResult<User> {
    chages.updated_at = Some(Utc::now().naive_utc());
    diesel::update(users::table.find(user_id))
        .set(&chages)
        .returning(User::as_returning())
        .get_result(conn)
}

pub fn delete_user(conn: &mut DbConnection, user_id: i32) -> DbResult<User> {
    diesel::delete(users::table.find(user_id))
        .returning(User::as_returning())
        .get_result(conn)
}

pub fn count_users(conn: &mut DbConnection) -> DbResult<i64> {
    users::table.count().get_result(conn)
}
