mod config;
mod db;

use axum::http::header::CONTENT_TYPE;
use axum::{Json, Router, extract::State, http::Method, http::StatusCode, routing::get};
use serde_json::{Value, json};
use sqlx::SqlitePool;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = config::Config::from_env();

    let pool = db::create_pool(&config.database_url).await;
    db::run_migrations(&pool).await;

    let state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact(config.frontend_url.parse().unwrap()))
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", config.host, config.port);
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    Ok(Json(json!({ "status": "ok" })))
}
