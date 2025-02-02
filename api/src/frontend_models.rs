use serde::Serialize;

use crate::tmdb::{
    client::IMAGE_BASE_URL,
    models::{
        movie::{MovieCreditCast, MovieCreditCrew, MovieCredits, MovieDetails, SearchMovie},
        pagination::PaginatedSearchResult,
        person::{PersonCreditCast, PersonCreditCrew, PersonCredits, PersonDetails, SearchPerson},
        shared::Language,
        socials::Socials,
    },
};

const DEFAULT_PLACEHOLDER_IMAGE: &str =
    "https://www.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png";

fn get_image_url(path: Option<String>) -> String {
    match path {
        Some(v) => format!("{}{}", IMAGE_BASE_URL, v),
        None => DEFAULT_PLACEHOLDER_IMAGE.to_owned(),
    }
}

/// Represents a list of movies formatted for the frontend.
#[derive(Debug, Serialize)]
pub struct FrontendMovieList {
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
}

/// Converts a [`SearchMovie`] into a [`MovieListing`] for frontend representation.
impl From<SearchMovie> for MovieListing {
    fn from(value: SearchMovie) -> Self {
        Self {
            id: value.base.id,
            title: value.base.title,
            poster: get_image_url(value.base.poster_path),
            description: value
                .base
                .overview
                .unwrap_or("No overview provided".to_owned()),
            // TODO: these 3
            overall_score: 0.0,
            is_liked: false,
            in_watchlist: false,
        }
    }
}

/// Converts a [`PaginatedSearchResult<SearchMovie>`] into a [`FrontendMovieList`].
impl From<PaginatedSearchResult<SearchMovie>> for FrontendMovieList {
    fn from(value: PaginatedSearchResult<SearchMovie>) -> Self {
        Self {
            movies: value.results.into_iter().map(MovieListing::from).collect(),
        }
    }
}

/// Represents movie credits formatted for the frontend.
#[derive(Debug, Serialize)]
pub struct FrontendCredits {
    /// List of cast members.
    cast: Vec<FrontendCast>,
    /// List of crew members.
    crew: Vec<FrontendCrew>,
}

/// Represents a cast member for the frontend.
#[derive(Debug, Serialize)]
pub struct FrontendCast {
    /// Name of the cast member.
    name: Option<String>,
    /// URL to the cast member's profile picture.
    #[serde(rename = "iconUrl")]
    icon_url: Option<String>,
    /// The character played by the cast member.
    character: String,
    #[serde(rename = "posterUrl")]
    poster_url: Option<String>,
    title: Option<String>,
    /// Unique identifier for the cast member.
    id: u64,
}

/// Represents a crew member for the frontend.
#[derive(Debug, Serialize)]
pub struct FrontendCrew {
    /// Name of the crew member.
    name: Option<String>,
    /// URL to the crew member's profile picture.
    #[serde(rename = "iconUrl")]
    icon_url: Option<String>,
    /// Department in which the crew member worked.
    department: String,
    #[serde(rename = "posterUrl")]
    poster_url: Option<String>,
    title: Option<String>,
    /// Unique identifier for the crew member.
    id: u64,
}

/// Converts a [`Crew`] struct into a [`FrontendCrew`] for frontend representation.
impl From<MovieCreditCrew> for FrontendCrew {
    fn from(value: MovieCreditCrew) -> Self {
        Self {
            name: Some(value.base.name),
            icon_url: Some(get_image_url(value.base.profile_path)),
            department: value.base.known_for_department,
            id: value.base.id,
            poster_url: None,
            title: None,
        }
    }
}

/// Converts a [`Cast`] struct into a [`FrontendCast`] for frontend representation.
impl From<MovieCreditCast> for FrontendCast {
    fn from(value: MovieCreditCast) -> Self {
        Self {
            name: Some(value.base.name),
            icon_url: Some(get_image_url(value.base.profile_path)),
            character: value.character,
            id: value.base.id,
            poster_url: None,
            title: None,
        }
    }
}

impl From<PersonCreditCrew> for FrontendCrew {
    fn from(value: PersonCreditCrew) -> Self {
        Self {
            name: None,
            icon_url: None,
            department: value.department,
            poster_url: Some(get_image_url(value.base.poster_path)),
            title: Some(value.base.title),
            id: value.base.id,
        }
    }
}

impl From<PersonCreditCast> for FrontendCast {
    fn from(value: PersonCreditCast) -> Self {
        Self {
            name: None,
            icon_url: None,
            poster_url: Some(get_image_url(value.base.poster_path)),
            title: Some(value.base.title),
            id: value.base.id,
            character: value.character,
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

impl From<PersonCredits> for FrontendCredits {
    fn from(value: PersonCredits) -> Self {
        Self {
            cast: value.cast.into_iter().map(FrontendCast::from).collect(),
            crew: value.crew.into_iter().map(FrontendCrew::from).collect(),
        }
    }
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

/// Converts a [`MovieDetails`] struct into a [`FrontendMovieDetails`] for frontend representation.
impl From<MovieDetails> for FrontendMovieDetails {
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
            credits: value.credits.map(FrontendCredits::from),
            // TODO: these 3
            overall_score: 0.0,
            is_liked: false,
            in_watchlist: false,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FrontendSocials {
    imdb: Option<String>,
    facebook: Option<String>,
    instagram: Option<String>,
    tiktok: Option<String>,
    twitter: Option<String>,
}

impl From<Socials> for FrontendSocials {
    fn from(value: Socials) -> Self {
        Self {
            imdb: value.imdb_id,
            facebook: value.facebook_id,
            instagram: value.instagram_id,
            tiktok: value.tiktok_id,
            twitter: value.twitter_id,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FrontendPersonDetails {
    biography: String,
    birthday: String,
    deathday: Option<String>,
    gender: u8,
    id: u64,
    #[serde(rename = "knownForDepartment")]
    known_for_department: String,
    name: String,
    #[serde(rename = "placeOfBirth")]
    place_of_birth: String,
    #[serde(rename = "iconUrl")]
    icon_url: String,
    credits: Option<FrontendCredits>,
    socials: Option<FrontendSocials>,
}

impl From<PersonDetails> for FrontendPersonDetails {
    fn from(value: PersonDetails) -> Self {
        Self {
            biography: value.biography,
            birthday: value.birthday,
            deathday: value.deathday,
            gender: value.gender,
            id: value.id,
            known_for_department: value.known_for_department,
            name: value.name,
            place_of_birth: value.place_of_birth,
            icon_url: get_image_url(value.profile_path),
            credits: value.credits.map(FrontendCredits::from),
            socials: value.external_ids.map(FrontendSocials::from),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FrontendPeopleList {
    people: Vec<FrontendPersonListing>,
}

#[derive(Debug, Serialize)]
pub struct FrontendPersonListing {
    id: u64,
    name: String,
    gender: u8,
    department: String,
    #[serde(rename = "iconUrl")]
    icon_url: String,
}

impl From<SearchPerson> for FrontendPersonListing {
    fn from(value: SearchPerson) -> Self {
        Self {
            id: value.id,
            name: value.name,
            gender: value.gender,
            department: value.known_for_department,
            icon_url: get_image_url(value.profile_path),
        }
    }
}

impl From<PaginatedSearchResult<SearchPerson>> for FrontendPeopleList {
    fn from(value: PaginatedSearchResult<SearchPerson>) -> Self {
        Self {
            people: value
                .results
                .into_iter()
                .map(FrontendPersonListing::from)
                .collect(),
        }
    }
}
