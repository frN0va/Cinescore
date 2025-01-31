use serde::{Deserialize, Serialize};

use crate::tmdb::{
    client::IMAGE_BASE_URL,
    models::{
        Cast, Crew, Genre, Language, MovieCredits, MovieDetails, MovieListSearch, SearchMovie,
    },
};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontendMovieList {
    movies: Vec<MovieListing>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct MovieListing {
    id: u64,
    title: String,
    poster: String,
    description: String,
    #[serde(rename = "overallScore")]
    overall_score: f32,
    #[serde(rename = "isLiked")]
    is_liked: bool,
    #[serde(rename = "inWatchlist")]
    in_watchlist: bool,
}

impl From<SearchMovie> for MovieListing {
    fn from(value: SearchMovie) -> Self {
        Self {
            id: value.id,
            title: value.title,
            poster: match value.poster_path {
                Some(v) => format!("{}{}", IMAGE_BASE_URL, v),
                // TODO: real image
                None => "https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png".to_owned(),
            },
            description: value.overview.unwrap_or("No overview provided".to_owned()),
            // TODO: these 3
            overall_score: 0.0,
            is_liked: false,
            in_watchlist: false,
        }
    }
}

impl From<MovieListSearch> for FrontendMovieList {
    fn from(value: MovieListSearch) -> Self {
        Self {
            movies: value.results.into_iter().map(MovieListing::from).collect(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontendCredits {
    cast: Vec<FrontendCast>,
    crew: Vec<FrontendCrew>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontendCast {
    name: String,
    #[serde(rename = "iconUrl")]
    icon_url: String,
    character: String,
    id: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontendCrew {
    name: String,
    #[serde(rename = "iconUrl")]
    icon_url: String,
    department: String,
    id: u64,
}

impl From<Crew> for FrontendCrew {
    fn from(value: Crew) -> Self {
        Self {
            name: value.name,
            icon_url: value.profile_path.unwrap_or("https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png".to_string()),
            department: value.known_for_department,
            id: value.id,
        }
    }
}

impl From<Cast> for FrontendCast {
    fn from(value: Cast) -> Self {
        Self {
            name: value.name,
            icon_url: value.profile_path.unwrap_or("https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png".to_string()),
            character: value.character,
            id: value.id,
        }
    }
}

impl From<MovieCredits> for FrontendCredits {
    fn from(value: MovieCredits) -> Self {
        Self {
            cast: value.cast.into_iter().map(FrontendCast::from).collect(),
            crew: value.crew.into_iter().map(FrontendCrew::from).collect(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontendMovieDetails {
    #[serde(rename = "backdropUrl")]
    backdrop_url: String,
    budget: u64,
    id: u64,
    #[serde(rename = "imdbId")]
    imdb_id: String,
    #[serde(rename = "originalLanguage")]
    original_language: String,
    overview: String,
    #[serde(rename = "posterUrl")]
    poster_url: String,
    title: String,
    #[serde(rename = "releaseDate")]
    release_date: String,
    revenue: u64,
    runtime: u64,
    #[serde(rename = "spokenLanguages")]
    spoken_languages: Vec<Language>,
    tagline: String,
}

impl From<MovieDetails> for FrontendMovieDetails {
    fn from(value: MovieDetails) -> Self {
        Self {
            backdrop_url: value.backdrop_path.unwrap_or("https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png".to_string()),
            budget: value.budget,
            id: value.id,
            imdb_id: value.imdb_id,
            original_language: value.original_language,
            overview: value.overview,
            poster_url: value.poster_path,
            title: value.title,
            release_date: value.release_date,
            revenue: value.revenue,
            runtime: value.runtime,
            spoken_languages: value.spoken_languages,
            tagline: value.tagline,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct FrontendMoviePage {
    details: FrontendMovieDetails,
    credits: FrontendCredits,
}
