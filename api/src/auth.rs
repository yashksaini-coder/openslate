use axum::{Json, extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

#[derive(Deserialize)]
pub struct LoginBody {
    pub password: String,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub async fn login(
    jar: CookieJar,
    Json(body): Json<LoginBody>,
) -> Result<(CookieJar, Json<serde_json::Value>), StatusCode> {
    let hash = std::env::var("ADMIN_PASSWORD_HASH").expect("ADMIN_PASSWORD_HASH must be set");

    let valid =
        bcrypt::verify(&body.password, &hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let now = OffsetDateTime::now_utc();
    let exp = now + Duration::days(30);

    let claims = Claims {
        sub: "admin".into(),
        exp: exp.unix_timestamp() as usize,
        iat: now.unix_timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = Cookie::build(("token", token))
        .path("/")
        .http_only(true)
        .secure(false)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .max_age(time::Duration::days(30))
        .build();

    Ok((jar.add(cookie), Json(json!({ "success": true }))))
}

pub async fn logout(jar: CookieJar) -> (CookieJar, Json<serde_json::Value>) {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(0))
        .build();

    (jar.add(cookie), Json(json!({ "success": true })))
}

pub async fn auth_middleware(
    cookie_jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = cookie_jar
        .get("token")
        .map(|c| c.value().to_string())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let secret = jwt_secret();

    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(next.run(request).await)
}

pub async fn me() -> Json<serde_json::Value> {
    Json(json!({ "authenticated": true }))
}
