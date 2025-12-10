use std::fmt::Debug;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{
    routing::{get, post},
    Router,
};
use axum_login::{AuthUser, AuthnBackend};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

use crate::auth::routes::{login_handler, logout_handler, me_handler, signup_handler};

mod routes;

pub fn build_router(pool: PgPool) -> Router {
    Router::new()
        .route("/signup", post(signup_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/me", get(me_handler))
        .with_state(pool)
}

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    username: String,
    email: String,
    password_hash: String,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("email", &self.email)
            .field("password_hash", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}

#[derive(Clone)]
pub struct Backend {
    pool: PgPool,
}

impl Backend {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

impl AuthnBackend for Backend {
    type User = User;

    type Credentials = Credentials;

    type Error = sqlx::Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        log::debug!("Authenticating user");

        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(&creds.email)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(user) = user {
            let parsed_hash = match PasswordHash::new(&user.password_hash) {
                Ok(h) => h,
                Err(e) => {
                    log::error!("Invalid password hash for creds {:?}: {}", creds, e);
                    return Ok(None);
                }
            };

            if Argon2::default()
                .verify_password(creds.password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                log::debug!("Authentication successful");
                return Ok(Some(user));
            }
        }

        Ok(None)
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
    }
}

pub fn hash_password<S: AsRef<str>>(password: S) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    Ok(Argon2::default()
        .hash_password(password.as_ref().as_bytes(), &salt)?
        .to_string())
}
