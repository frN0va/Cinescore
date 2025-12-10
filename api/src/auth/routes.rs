use axum::{extract::State, response::IntoResponse, Json};
use axum_login::AuthSession;
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

use crate::auth::{self, Backend, Credentials};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn signup_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<SignupRequest>,
) -> impl IntoResponse {
    log::info!("Attempting to signup: {}", payload.email);

    let user_check = sqlx::query("SELECT id FROM users WHERE email = $1 OR username = $2")
        .bind(&payload.email)
        .bind(&payload.username)
        .fetch_optional(&pool)
        .await;

    let user_row = match user_check {
        Ok(row) => row,
        Err(e) => {
            log::error!("Database error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response();
        }
    };

    if user_row.is_some() {
        return (StatusCode::CONFLICT, "User already exists").into_response();
    }

    let hash = match auth::hash_password(&payload.password) {
        Ok(h) => h,
        Err(e) => {
            log::error!("Error hasing password: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Password Error").into_response();
        }
    };

    match sqlx::query("INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3)")
        .bind(&payload.username)
        .bind(&payload.email)
        .bind(hash)
        .execute(&pool)
        .await
    {
        Ok(_) => (),
        Err(e) => {
            log::error!("Error inserting user into database: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database Error").into_response();
        }
    }

    log::debug!("Signup successful");
    (StatusCode::CREATED, "User created successfully").into_response()
}

pub async fn login_handler(
    mut auth_session: AuthSession<Backend>,
    Json(creds): Json<Credentials>,
) -> impl IntoResponse {
    log::info!("Attempting login for user {}", creds.email);
    let email = creds.email.clone();

    let user = match auth_session.authenticate(creds).await {
        Ok(Some(user)) => user,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response(),
        Err(e) => {
            log::error!("Authentication error: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Auth error").into_response();
        }
    };

    if auth_session.login(&user).await.is_err() {
        log::info!("Login failure for user {}", email);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Login failed").into_response();
    }

    log::info!("Login success for user {}", email);
    (StatusCode::OK, "Login Success").into_response()
}

pub async fn logout_handler(mut auth_session: AuthSession<Backend>) -> impl IntoResponse {
    log::debug!("Attempting logout");
    match auth_session.logout().await {
        Ok(_) => (StatusCode::OK, "Logged out").into_response(),
        Err(e) => {
            log::error!("Logout error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Logout error").into_response()
        }
    }
}

pub async fn me_handler(auth_session: AuthSession<Backend>) -> impl IntoResponse {
    match auth_session.user {
        Some(user) => Json(user).into_response(),
        None => (StatusCode::UNAUTHORIZED, "Not logged in").into_response(),
    }
}
