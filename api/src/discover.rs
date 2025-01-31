use axum::{extract::Path, Json};

use crate::{
    frontend_models::{FrontendMovieDetails, FrontendMovieList},
    tmdb::{
        client::TMDBClient,
        queries::{
            movie_details::{MovieDetailsQuery, MovieDetailsRequest},
            movie_lists::{MovieListNowPlayingRequest, MovieListQuery, MovieListTrendingRequest},
        },
    },
};

/// Fetches a list of trending movies
///
/// # Returns
/// A JSON-wrapped `FrontendMovieList` containing trending movies.
pub async fn fetch_trending() -> Json<FrontendMovieList> {
    let client = TMDBClient::new(std::env::var("TMDB_API_KEY").unwrap());
    Json(
        MovieListTrendingRequest::new()
            .fetch(&client)
            .await
            .expect("handle errors later"),
    )
}

pub async fn fetch_now_playing() -> Json<FrontendMovieList> {
    let client = TMDBClient::new(std::env::var("TMDB_API_KEY").unwrap());
    Json(
        MovieListNowPlayingRequest::new()
            .fetch(&client)
            .await
            .expect("handle errors later"),
    )
}

pub async fn fetch_movie_details(Path(movie_id): Path<u64>) -> Json<FrontendMovieDetails> {
    let client = TMDBClient::new(std::env::var("TMDB_API_KEY").unwrap());
    Json(
        MovieDetailsRequest::new()
            .append_to_response("credits")
            .fetch(&client, movie_id)
            .await
            .expect("handle errors later"),
    )
}
