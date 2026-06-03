use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub frontend_url: String,
    pub r2_bucket: String,
    pub r2_account_id: String,
    pub r2_access_key: String,
    pub r2_secret_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3001".into())
                .parse()
                .expect("PORT must be a number"),
            frontend_url: env::var("FRONTEND_URL")
                .unwrap_or_else(|_| "http://localhost:5173".into()),
            r2_bucket: env::var("R2_BUCKET").expect("R2_BUCKET must be set"),
            r2_account_id: env::var("R2_ACCOUNT_ID").expect("R2_ACCOUNT_ID must be set"),
            r2_access_key: env::var("R2_ACCESS_KEY").expect("R2_ACCESS_KEY must be set"),
            r2_secret_key: env::var("R2_SECRET_KEY").expect("R2_SECRET_KEY must be set"),
        }
    }
}
