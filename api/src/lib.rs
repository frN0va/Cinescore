use axum::{routing::get, Router};
use discover::{
    fetch_movie_details, fetch_now_playing, fetch_person_details, fetch_trending,
    fetch_trending_people,
};

mod discover;
mod frontend_models;
mod tmdb;

pub fn build_router() -> Router {
    Router::new()
        .route("/api/v1/discover/trending", get(fetch_trending))
        .route(
            "/api/v1/discover/trending_people",
            get(fetch_trending_people),
        )
        .route("/api/v1/discover/now_playing", get(fetch_now_playing))
        .route("/api/v1/movies/{id}", get(fetch_movie_details))
        .route("/api/v1/people/{id}", get(fetch_person_details))
}
