use serde::{Deserialize, Serialize};

use crate::tmdb::{
    client::IMAGE_BASE_URL,
    models::{Cast, Crew, Language, MovieCredits, MovieDetails, MovieListSearch, SearchMovie},
};

/// Represents a list of movies formatted for the frontend.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontendMovieList {
    movies: Vec<MovieListing>,
}

/// Represents an individual movie entry in the frontend.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
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
}

/// Converts a [`SearchMovie`] into a [`MovieListing`] for frontend representation.
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

/// Converts a [`MovieListSearch`] into a [`FrontendMovieList`].
impl From<MovieListSearch> for FrontendMovieList {
    fn from(value: MovieListSearch) -> Self {
        Self {
            movies: value.results.into_iter().map(MovieListing::from).collect(),
        }
    }
}

/// Represents movie credits formatted for the frontend.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontendCredits {
    /// List of cast members.
    cast: Vec<FrontendCast>,
    /// List of crew members.
    crew: Vec<FrontendCrew>,
}

/// Represents a cast member for the frontend.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontendCast {
    /// Name of the cast member.
    name: String,
    /// URL to the cast member's profile picture.
    #[serde(rename = "iconUrl")]
    icon_url: String,
    /// The character played by the cast member.
    character: String,
    /// Unique identifier for the cast member.
    id: u64,
}

/// Represents a crew member for the frontend.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontendCrew {
    /// Name of the crew member.
    name: String,
    /// URL to the crew member's profile picture.
    #[serde(rename = "iconUrl")]
    icon_url: String,
    /// Department in which the crew member worked.
    department: String,
    /// Unique identifier for the crew member.
    id: u64,
}

/// Converts a [`Crew`] struct into a [`FrontendCrew`] for frontend representation.
impl From<Crew> for FrontendCrew {
    fn from(value: Crew) -> Self {
        Self {
            name: value.name,
            icon_url: match value.profile_path {
                Some(v) => format!("{}{}", IMAGE_BASE_URL, v),
                // TODO: real image
                None => "https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png".to_owned(),
            },
            department: value.known_for_department,
            id: value.id,
        }
    }
}

/// Converts a [`Cast`] struct into a [`FrontendCast`] for frontend representation.
impl From<Cast> for FrontendCast {
    fn from(value: Cast) -> Self {
        Self {
            name: value.name,
            icon_url: match value.profile_path {
                Some(v) => format!("{}{}", IMAGE_BASE_URL, v),
                // TODO: real image
                None => "https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png".to_owned(),
            },
            character: value.character,
            id: value.id,
        }
    }
}

/// Converts [`MovieCredits`] into [`FrontendCredits`] for frontend representation.
impl From<MovieCredits> for FrontendCredits {
    fn from(value: MovieCredits) -> Self {
        Self {
            cast: value.cast.into_iter().map(FrontendCast::from).collect(),
            crew: value.crew.into_iter().map(FrontendCrew::from).collect(),
        }
    }
}

/// Represents detailed movie information formatted for the frontend.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FrontendMovieDetails {
    /// URL to the movie's backdrop image.
    #[serde(rename = "backdropUrl")]
    backdrop_url: String,
    /// The budget of the movie in dollars.
    budget: u64,
    /// Unique identifier for the movie.
    id: u64,
    /// The IMDb identifier of the movie.
    #[serde(rename = "imdbId")]
    imdb_id: String,
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
    release_date: String,
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
    credits: Option<FrontendCredits>,
}

/// Converts a [`MovieDetails`] struct into a [`FrontendMovieDetails`] for frontend representation.
impl From<MovieDetails> for FrontendMovieDetails {
    fn from(value: MovieDetails) -> Self {
        Self {
            backdrop_url: match value.backdrop_path {
                Some(v) => format!("{}{}", IMAGE_BASE_URL, v),
                // TODO: real image
                None => "https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png".to_owned(),
            },
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
            credits: match value.credits {
                Some(v) => Some(FrontendCredits::from(v)),
                None => None,
            }
        }
    }
}
