use axum::{ extract::{ State, Json }, http::StatusCode, response::IntoResponse };
use axum_extra::extract::cookie::{ Cookie, CookieJar };
use bcrypt::{ hash, verify, DEFAULT_COST };
use chrono::{ Duration, Utc };
use jsonwebtoken::{ encode, EncodingKey, Header };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use sqlx::{ MySqlPool, Row };

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn register_handler(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegisterRequest>
) -> impl IntoResponse {
    // hash password
    let hashed = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "status": "error", "message": "Hash failed" })),
            );
        }
    };

    let res = sqlx
        ::query("INSERT INTO users (username, email, password, role_id) VALUES (?, ?, ?, 2)")
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(&hashed)
        .execute(&pool).await;

    match res {
        Ok(_) =>
            (
                StatusCode::CREATED,
                Json(
                    json!({
                "status": "success",
                "message": "✅ User registered successfully"
            })
                ),
            ),
        Err(e) =>
            (
                StatusCode::BAD_REQUEST,
                Json(
                    json!({
                "status": "error",
                "message": format!("❌ Registration failed: {}", e)
            })
                ),
            ),
    }
}

pub async fn login_handler(
    State(pool): State<MySqlPool>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let row = sqlx::query(
        r#"
        SELECT users.id, users.email, users.password, roles.name AS role
        FROM users
        LEFT JOIN roles ON users.role_id = roles.id
        WHERE users.email = ?
        "#,
    )
    .bind(&payload.email)
    .fetch_optional(&pool)
    .await
    .unwrap();

    if let Some(user) = row {
        let password: String = user.get("password");
        let email: String = user.get("email");
        let id: i64 = user.get("id");
        let role: Option<String> = user.try_get("role").ok();

        if verify(&payload.password, &password).unwrap_or(false) {
            let exp = Utc::now() + Duration::hours(24);
            let claims = Claims {
                sub: email.clone(),
                exp: exp.timestamp() as usize,
            };

            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
            let token =
                encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
                    .unwrap();

            let cookie = Cookie::build(("jwt", token.clone()))
                .http_only(true)
                .path("/")
                .max_age(time::Duration::hours(24))
                .build();

            let updated_jar = jar.add(cookie);

            return (
                StatusCode::OK,
                updated_jar,
                Json(json!({
                    "status": "success",
                    "token": token,
                    "user": {
                        "id": id,
                        "email": email,
                        "role": role.unwrap_or("user".into())
                    }
                })),
            );
        }
    }

    // ⬇️ bagian ini sudah diperbaiki
    (
        StatusCode::UNAUTHORIZED,
        jar,
        Json(json!({
            "status": "error",
            "message": "❌ Invalid credentials"
        })),
    )
}
pub async fn logout_handler(
    jar: CookieJar,
) -> impl IntoResponse {
    // Buat cookie kosong dengan nama sama dan umur 0 supaya dianggap "terhapus"
    let remove_cookie = Cookie::build(("jwt", ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .http_only(true)
        .build();

    let updated_jar = jar.add(remove_cookie);

    (
        StatusCode::OK,
        updated_jar,
        axum::Json(json!({
            "status": "success",
            "message": "✅ Logged out successfully"
        })),
    )
}
