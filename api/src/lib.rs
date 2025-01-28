use axum::{routing::get, Router};
use discover::fetch_trending;

mod discover;
mod frontend_models;
mod tmdb_models;

pub const IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p/w780/";

pub fn build_router() -> Router {
    Router::new().route("/api/v1/discover/trending", get(fetch_trending))
}
