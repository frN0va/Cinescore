use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, post},
    Json, Router,
};
use axum_login::AuthSession;
use reqwest::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

use crate::auth::Backend;

#[derive(Deserialize)]
pub struct RateRequest {
    rating: i32,
}

#[derive(Deserialize)]
pub struct TopFiveRequest {
    rank: i32,
}

pub fn build_router(pool: PgPool) -> Router {
    Router::new()
        .route("/movies/:id/like", post(like_movie))
        .route("/movies/:id/like", delete(unlike_movie))
        .route("/movies/:id/watchlist", post(add_to_watchlist))
        .route("/movies/:id/watchlist", delete(remove_from_watchlist))
        .route("/movies/:id/rate", post(rate_movie))
        .route("/movies/:id/top5", post(set_top_five))
        .with_state(pool)
}

async fn like_movie(
    auth_session: AuthSession<Backend>,
    State(pool): State<PgPool>,
    Path(movie_id): Path<i64>,
) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => user,
        None => return StatusCode::UNAUTHORIZED,
    };

    match sqlx::query(
        "INSERT INTO movie_likes (user_id, movie_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(user.id)
    .bind(movie_id)
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn unlike_movie(
    auth_session: AuthSession<Backend>,
    State(pool): State<PgPool>,
    Path(movie_id): Path<i64>,
) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => user,
        None => return StatusCode::UNAUTHORIZED,
    };

    match sqlx::query("DELETE FROM movie_likes WHERE user_id = $1 AND movie_id = $2")
        .bind(user.id)
        .bind(movie_id)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn add_to_watchlist(
    auth_session: AuthSession<Backend>,
    State(pool): State<PgPool>,
    Path(movie_id): Path<i64>,
) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => user,
        None => return StatusCode::UNAUTHORIZED,
    };

    match sqlx::query(
        "INSERT INTO movie_watchlist (user_id, movie_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
    )
    .bind(user.id)
    .bind(movie_id)
    .execute(&pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn remove_from_watchlist(
    auth_session: AuthSession<Backend>,
    State(pool): State<PgPool>,
    Path(movie_id): Path<i64>,
) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => user,
        None => return StatusCode::UNAUTHORIZED,
    };

    match sqlx::query("DELETE FROM movie_watchlist WHERE user_id = $1 AND movie_id = $2")
        .bind(user.id)
        .bind(movie_id)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn rate_movie(
    auth_session: AuthSession<Backend>,
    State(pool): State<PgPool>,
    Path(movie_id): Path<i64>,
    Json(payload): Json<RateRequest>,
) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => user,
        None => return StatusCode::UNAUTHORIZED,
    };

    if payload.rating < 1 || payload.rating > 5 {
        return StatusCode::BAD_REQUEST;
    }

    match sqlx::query("INSERT INTO movie_ratings (user_id, movie_id, rating) VALUES ($1, $2, $3) ON CONFLICT (user_id, movie_id) DO UPDATE SET rating = $3, updated_at = NOW()")
        .bind(user.id)
        .bind(movie_id)
        .bind(payload.rating)
        .execute(&pool)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Database error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn set_top_five(
    auth_session: AuthSession<Backend>,
    State(pool): State<PgPool>,
    Path(movie_id): Path<i64>,
    Json(payload): Json<TopFiveRequest>,
) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => user,
        None => return StatusCode::UNAUTHORIZED,
    };

    if payload.rank < 1 || payload.rank > 5 {
        return StatusCode::BAD_REQUEST;
    }

    // remove the movie from any other rank if it exists, then insert/update new rank
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    if let Err(e) = sqlx::query("DELETE FROM user_top_five WHERE user_id = $1 AND movie_id = $2")
        .bind(user.id)
        .bind(movie_id)
        .execute(&mut *tx)
        .await
    {
        tracing::error!("Database error: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    if let Err(e) = sqlx::query("INSERT INTO user_top_five (user_id, rank, movie_id) VALUES ($1, $2, $3) ON CONFLICT (user_id, rank) DO UPDATE SET movie_id = $3, updated_at = NOW()")
        .bind(user.id)
        .bind(movie_id)
        .execute(&mut *tx)
        .await
    {
        tracing::error!("Database error: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    match tx.commit().await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            tracing::error!("Transaction error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
