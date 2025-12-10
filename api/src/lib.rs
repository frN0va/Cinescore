//! Cinescore is an open source movie rating platform. This crate contains the API
use axum::{routing::get, Router};
use axum_login::AuthManagerLayerBuilder;
use discover::{
    fetch_movie_details, fetch_now_playing, fetch_person_details, fetch_trending,
    fetch_trending_people, fetch_upcoming_movies, search_movies, search_people,
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tower_sessions::{cookie::time::Duration, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;

use crate::auth::Backend;

mod auth;
mod discover;
mod frontend_models;
mod tmdb;

pub async fn build_router(pool: PgPool) -> Router {
    tracing::info!("Creating session manager...");

    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("Failed to run session migrations");

    // with_secure is false in debug and true in release
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(!cfg!(debug_assertions))
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let backend = Backend::new(pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    tracing::info!("Initializing API routes...");

    Router::new()
        .route("/api/v1/discover/trending", get(fetch_trending))
        .route(
            "/api/v1/discover/trending_people",
            get(fetch_trending_people),
        )
        .route("/api/v1/discover/now_playing", get(fetch_now_playing))
        .route("/api/v1/discover/upcoming", get(fetch_upcoming_movies))
        .route("/api/v1/movies/{id}", get(fetch_movie_details))
        .route("/api/v1/people/{id}", get(fetch_person_details))
        .route("/api/v1/search/movies", get(search_movies))
        .route("/api/v1/search/people", get(search_people))
        .nest("/api/v1/auth", auth::build_router(pool.clone()))
        .layer(auth_layer)
        .layer(TraceLayer::new_for_http())
}
