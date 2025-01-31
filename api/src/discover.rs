use axum::Json;

use crate::{
    frontend_models::FrontendMovieList,
    tmdb::{
        client::TMDBClient,
        queries::movie_lists::{MovieListNowPlaying, MovieListQuery, MovieListTrending},
    },
};

pub async fn fetch_trending() -> Json<FrontendMovieList> {
    let client = TMDBClient::new(std::env::var("TMDB_API_KEY").unwrap());
    Json(
        MovieListTrending::new()
            .fetch(&client)
            .await
            .expect("handle errors later"),
    )
}

pub async fn fetch_now_playing() -> Json<FrontendMovieList> {
    let client = TMDBClient::new(std::env::var("TMDB_API_KEY").unwrap());
    Json(
        MovieListNowPlaying::new()
            .fetch(&client)
            .await
            .expect("handle errors later"),
    )
}
