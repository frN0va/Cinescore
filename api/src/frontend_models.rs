use serde::{Deserialize, Serialize};

use crate::tmdb::{
    client::IMAGE_BASE_URL,
    models::{MovieListSearch, SearchMovie},
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
