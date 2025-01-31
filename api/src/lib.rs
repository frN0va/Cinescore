use axum::{routing::get, Router};
use discover::{fetch_now_playing, fetch_trending};

mod discover;
mod frontend_models;
mod tmdb;

pub fn build_router() -> Router {
    Router::new()
        .route("/api/v1/discover/trending", get(fetch_trending))
        .route("/api/v1/discover/now_playing", get(fetch_now_playing))
}
