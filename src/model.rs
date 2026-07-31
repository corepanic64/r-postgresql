#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub async fn create_user(pool: &sqlx::PgPool, name: &str, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(name)
        .bind(email)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_user(pool: &sqlx::PgPool, id: i32) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn get_users(pool: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}
