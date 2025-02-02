use axum::{extract::Path, response::IntoResponse, Json};

use crate::{
    frontend_models::{
        movies::{FrontendMovieDetails, FrontendMovieList},
        people::{FrontendPeopleList, FrontendPersonDetails},
    },
    tmdb::{
        client::TMDBClient,
        queries::{
            common::DetailsQuery,
            movie_details::MovieDetailsRequest,
            movie_lists::{MovieListNowPlayingRequest, MovieListQuery, MovieListTrendingRequest},
            people_details::{PersonDetailsRequest, TrendingPeopleQuery, TrendingPeopleRequest},
        },
    },
};

pub struct ApiFetchError {
    status_code: String,
}

impl IntoResponse for ApiFetchError {
    fn into_response(self) -> axum::response::Response {
        let error = format!(
            "Error while fetching data from TMDB. Got status: {}",
            self.status_code
        );

        log::error!("{}", error);

        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, error).into_response()
    }
}
impl From<reqwest::Error> for ApiFetchError {
    /// Converts a [`reqwest::Error`] into a [`ApiFetchError`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`reqwest::Error`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`ApiFetchError`] instance with all fields mapped from the source.
    fn from(value: reqwest::Error) -> Self {
        let status = match value.status() {
            Some(v) => v.to_string(),
            None => "N/A".to_owned(),
        };
        log::error!("Reqwest error occurred: {:?}, status: {}", value, status);

        Self {
            status_code: status,
        }
    }
}

fn get_tmdb_client() -> TMDBClient {
    TMDBClient::new(std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set"))
}

pub async fn fetch_trending() -> Result<Json<FrontendMovieList>, ApiFetchError> {
    let client = get_tmdb_client();
    log::info!("Fetching trending movies");
    Ok(Json(MovieListTrendingRequest::new().fetch(&client).await?))
}

pub async fn fetch_trending_people() -> Result<Json<FrontendPeopleList>, ApiFetchError> {
    let client = get_tmdb_client();
    log::info!("Fetching trending people");
    Ok(Json(TrendingPeopleRequest::new().fetch(&client).await?))
}

pub async fn fetch_now_playing() -> Result<Json<FrontendMovieList>, ApiFetchError> {
    let client = get_tmdb_client();
    log::info!("Fetching now playing movies");
    Ok(Json(
        MovieListNowPlayingRequest::new().fetch(&client).await?,
    ))
}

pub async fn fetch_movie_details(
    Path(movie_id): Path<u64>,
) -> Result<Json<FrontendMovieDetails>, ApiFetchError> {
    let client = get_tmdb_client();
    log::info!("Fetching details for movie ID: {}", movie_id);
    Ok(Json(
        MovieDetailsRequest::new()
            .append_to_response("credits")
            .fetch(&client, movie_id)
            .await?,
    ))
}

pub async fn fetch_person_details(
    Path(person_id): Path<u64>,
) -> Result<Json<FrontendPersonDetails>, ApiFetchError> {
    let client = get_tmdb_client();
    log::info!("Fetching details for person ID: {}", person_id);
    Ok(Json(
        PersonDetailsRequest::new()
            .append_to_response("credits,external_ids")
            .fetch(&client, person_id)
            .await?,
    ))
}
