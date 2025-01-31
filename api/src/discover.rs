use axum::{extract::Path, response::IntoResponse, Json};

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

pub struct ApiFetchError {
    status_code: String,
}

impl IntoResponse for ApiFetchError {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!(
                "Error while fetching data from TMDB. Got status: {}",
                self.status_code
            ),
        )
            .into_response()
    }
}
impl From<reqwest::Error> for ApiFetchError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            status_code: match value.status() {
                Some(v) => v.to_string(),
                None => "N/A".to_string(),
            },
        }
    }
}

pub async fn fetch_trending() -> Result<Json<FrontendMovieList>, ApiFetchError> {
    let client = TMDBClient::new(std::env::var("TMDB_API_KEY").unwrap());
    Ok(Json(MovieListTrendingRequest::new().fetch(&client).await?))
}

pub async fn fetch_now_playing() -> Result<Json<FrontendMovieList>, ApiFetchError> {
    let client = TMDBClient::new(std::env::var("TMDB_API_KEY").unwrap());
    Ok(Json(
        MovieListNowPlayingRequest::new().fetch(&client).await?,
    ))
}

pub async fn fetch_movie_details(
    Path(movie_id): Path<u64>,
) -> Result<Json<FrontendMovieDetails>, ApiFetchError> {
    let client = TMDBClient::new(std::env::var("TMDB_API_KEY").unwrap());
    Ok(Json(
        MovieDetailsRequest::new()
            .append_to_response("credits")
            .fetch(&client, movie_id)
            .await?,
    ))
}
