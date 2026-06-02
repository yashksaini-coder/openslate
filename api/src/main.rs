mod auth;
mod config;
mod db;

use axum::http::Method;
use axum::{Router, middleware, routing::get};
use tower_http::cors::AllowOrigin;

#[derive(Clone)]
struct AppState {
    db: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = config::Config::from_env();

    let pool = db::create_pool(&config.database_url).await;
    db::run_migrations(&pool).await;

    let state = AppState { db: pool };

    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(AllowOrigin::exact(config.frontend_url.parse().unwrap()))
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let public = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/auth/login", axum::routing::post(auth::login))
        .route("/api/auth/logout", axum::routing::post(auth::logout));

    let protected = Router::new()
        .route("/api/auth/me", get(auth::me))
        .route_layer(middleware::from_fn(auth::auth_middleware));

    let app = Router::new()
        .merge(public)
        .merge(protected)
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", config.host, config.port);
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<axum::Json<serde_json::Value>, axum::http::StatusCode> {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map_err(|_| axum::http::StatusCode::SERVICE_UNAVAILABLE)?;

    Ok(axum::Json(serde_json::json!({ "status": "ok" })))
}
