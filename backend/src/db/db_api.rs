use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn create_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .connect("postgres://postgres:password@localhost:5432/logs")
        .await
        .unwrap()
}
