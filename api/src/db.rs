use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

pub async fn create_pool(database_url: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect(database_url)
        .await
        .expect("Failed to create database pool")
}

pub async fn run_migrations(pool: &SqlitePool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("Failed to run migrations");
}
