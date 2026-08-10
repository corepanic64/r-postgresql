use crate::auth::register::model::Register;

pub async fn register_db_user(pool: &sqlx::PgPool, payload: Register) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email, password) VALUES ($1, $2, $3)")
        .bind(&payload.name)
        .bind(&payload.email)
        .bind(&payload.password)
        .execute(pool)
        .await?;
    Ok(())
}
