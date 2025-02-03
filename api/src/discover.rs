use axum::{extract::Path, Json};
use serde::Deserialize;

use crate::{
    frontend_models::{
        movies::{FrontendMovieDetails, FrontendMovieList},
        people::{FrontendPeopleList, FrontendPersonDetails},
    },
    tmdb::{
        client::{ApiFetchError, TMDBClient},
        queries::{
            movie_details::MovieDetailsRequest,
            movie_lists::{MovieListNowPlayingRequest, MovieListTrendingRequest},
            people_details::{PersonDetailsRequest, TrendingPeopleRequest},
            search::{SearchMoviesRequest, SearchPeopleRequest},
            traits::{AppendToResponseQueryParam, IdQuery, Query, QueryQueryParam},
        },
    },
};

fn get_tmdb_client() -> TMDBClient {
    TMDBClient::new(match std::env::var("TMDB_API_KEY") {
        Ok(v) => v,
        Err(_) => {
            log::error!("`TMDB_API_KEY` environment variable must be set.");
            std::process::exit(1);
        }
    })
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

#[derive(Deserialize)]
pub struct SearchParams {
    query: String,
}

pub async fn search_movies(
    params: axum::extract::Query<SearchParams>,
) -> Result<Json<FrontendMovieList>, ApiFetchError> {
    let client = get_tmdb_client();

    Ok(Json(
        SearchMoviesRequest::new()
            .query(params.query.clone())
            .fetch(&client)
            .await?,
    ))
}

pub async fn search_people(
    params: axum::extract::Query<SearchParams>,
) -> Result<Json<FrontendPeopleList>, ApiFetchError> {
    let client = get_tmdb_client();

    Ok(Json(
        SearchPeopleRequest::new()
            .query(params.query.clone())
            .fetch(&client)
            .await?,
    ))
}
