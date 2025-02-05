use chrono::NaiveDate;
use serde::Serialize;

use crate::tmdb::models::{
    common::{Language, PaginatedSearchResult},
    movie::{MovieDetails, SearchMovie},
};

use super::{common::get_image_url, credits::FrontendMovieCredits};

/// Represents a list of movies formatted for the frontend.
#[derive(Debug, Serialize)]
pub struct FrontendMovieList {
    /// A list of movies
    movies: Vec<MovieListing>,
}

/// Represents an individual movie entry in the frontend.
#[derive(Debug, Serialize)]
pub struct MovieListing {
    /// Unique identifier for the movie.
    id: u64,
    /// The title of the movie.
    title: String,
    /// URL to the movie's poster image.
    poster: String,
    /// A brief description or overview of the movie.
    description: String,
    /// The overall score or rating of the movie.
    #[serde(rename = "overallScore")]
    overall_score: f32,
    /// Whether the user has liked the movie.
    #[serde(rename = "isLiked")]
    is_liked: bool,
    /// Whether the movie is in the user's watchlist.
    #[serde(rename = "inWatchlist")]
    in_watchlist: bool,
    /// Release date of the movie.
    #[serde(rename = "releaseDate")]
    release_date: Option<NaiveDate>,
}

/// Represents detailed movie information formatted for the frontend.
#[derive(Debug, Serialize)]
pub struct FrontendMovieDetails {
    /// URL to the movie's backdrop image.
    #[serde(rename = "backdropUrl")]
    backdrop_url: String,
    /// The budget of the movie in dollars.
    budget: u64,
    /// Unique identifier for the movie.
    id: u64,
    /// The IMDb identifier of the movie.
    #[allow(clippy::doc_markdown)]
    #[serde(rename = "imdbId")]
    imdb_id: Option<String>,
    /// The original language of the movie.
    #[serde(rename = "originalLanguage")]
    original_language: String,
    /// A brief description or overview of the movie.
    overview: String,
    /// URL to the movie's poster image.
    #[serde(rename = "posterUrl")]
    poster_url: String,
    /// The title of the movie.
    title: String,
    /// The release date of the movie.
    #[serde(rename = "releaseDate")]
    release_date: Option<NaiveDate>,
    /// The revenue of the movie in dollars.
    revenue: u64,
    /// The runtime of the movie in minutes.
    runtime: u64,
    /// A list of languages spoken in the movie.
    #[serde(rename = "spokenLanguages")]
    spoken_languages: Vec<Language>,
    /// The tagline of the movie.
    tagline: String,
    /// Optional movie credits (cast and crew).
    credits: Option<FrontendMovieCredits>,
    /// The overall score or rating of the movie.
    #[serde(rename = "overallScore")]
    overall_score: f32,
    /// Whether the user has liked the movie.
    #[serde(rename = "isLiked")]
    is_liked: bool,
    /// Whether the movie is in the user's watchlist.
    #[serde(rename = "inWatchlist")]
    in_watchlist: bool,
}

/// Converts a [`SearchMovie`] into a [`MovieListing`] for frontend representation.
impl From<SearchMovie> for MovieListing {
    /// Converts a [`SearchMovie`] into a [`MovieListing`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`SearchMovie`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`MovieListing`] instance with all fields mapped from the source.
    fn from(value: SearchMovie) -> Self {
        Self {
            id: value.base.id,
            title: value.base.title,
            poster: get_image_url(value.base.poster_path),
            description: value
                .base
                .overview
                .unwrap_or("No overview provided".to_owned()),
            release_date: value.base.release_date,
            // TODO: these 3
            overall_score: 0.0,
            is_liked: false,
            in_watchlist: false,
        }
    }
}

/// Converts a [`PaginatedSearchResult<SearchMovie>`] into a [`FrontendMovieList`].
impl From<PaginatedSearchResult<SearchMovie>> for FrontendMovieList {
    /// Converts a [`PaginatedSearchResult<SearchMovie>`] into a [`FrontendMovieList`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`PaginatedSearchResult<SearchMovie>`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendMovieList`] instance with all fields mapped from the source.
    fn from(value: PaginatedSearchResult<SearchMovie>) -> Self {
        Self {
            movies: value.results.into_iter().map(MovieListing::from).collect(),
        }
    }
}

/// Converts a [`MovieDetails`] struct into a [`FrontendMovieDetails`] for frontend representation.
impl From<MovieDetails> for FrontendMovieDetails {
    /// Converts a [`MovieDetails`] into a [`FrontendMovieDetails`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`MovieDetails`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendMovieDetails`] instance with all fields mapped from the source.
    fn from(value: MovieDetails) -> Self {
        Self {
            backdrop_url: get_image_url(value.base.backdrop_path),
            budget: value.budget,
            id: value.base.id,
            imdb_id: value.imdb_id,
            original_language: value.base.original_language,
            overview: value.base.overview.unwrap_or("N/A".to_owned()),
            poster_url: get_image_url(value.base.poster_path),
            title: value.base.title,
            release_date: value.base.release_date,
            revenue: value.revenue,
            runtime: value.runtime,
            spoken_languages: value.spoken_languages,
            tagline: value.tagline,
            credits: value.credits.map(FrontendMovieCredits::from),
            // TODO: these 3
            overall_score: 0.0,
            is_liked: false,
            in_watchlist: false,
        }
    }
}
