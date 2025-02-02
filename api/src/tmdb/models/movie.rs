#![allow(dead_code)]
use serde::Deserialize;

use super::{
    person::SearchPerson,
    shared::{Genre, Language, ProductionCompany, ProductionCountry},
};

/// Represents a collection of movies
#[derive(Debug, Deserialize)]
pub struct Collection {
    /// The collection ID
    pub id: u64,
    /// The collection name
    pub name: String,
    /// The collection's poster image path
    pub poster_path: String,
    /// The collection's backdrop image path
    pub backdrop_path: String,
}

#[derive(Debug, Deserialize)]
pub struct BaseMovie {
    /// Indicates whether the movie is for adults.
    pub adult: bool,
    /// Path to the movie's backdrop image.
    pub backdrop_path: Option<String>,
    /// Unique identifier for the movie.
    pub id: u64,
    /// Original language of the movie.
    pub original_language: String,
    /// Original title of the movie.
    pub original_title: String,
    /// Brief overview of the movie.
    pub overview: Option<String>,
    /// Popularity score of the movie.
    pub popularity: f64,
    /// Path to the movie's poster image.
    pub poster_path: Option<String>,
    /// Release date of the movie.
    pub release_date: String,
    /// Title of the movie.
    pub title: String,
    /// Indicates whether the movie has videos
    pub video: bool,
    /// Average vote score.
    pub vote_average: f64,
    /// Total number of votes received.
    pub vote_count: u64,
}

#[derive(Debug, Deserialize)]
pub struct SearchMovie {
    #[serde(flatten)]
    pub base: BaseMovie,
    /// List of genre IDs associated with the movie.
    pub genre_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct MovieDetails {
    #[serde(flatten)]
    pub base: BaseMovie,
    /// Collection the movie belongs to, if any.
    pub belongs_to_collection: Option<Collection>,
    /// Budget of the movie in dollars.
    pub budget: u64,
    /// List of genres associated with the movie.
    pub genres: Vec<Genre>,
    /// Homepage URL of the movie.
    pub homepage: String,
    /// IMDb identifier for the movie.
    pub imdb_id: String,
    /// List of production companies involved.
    pub production_companies: Vec<ProductionCompany>,
    /// List of countries where the movie was produced.
    pub production_countries: Vec<ProductionCountry>,
    /// Revenue generated by the movie in dollars.
    pub revenue: u64,
    /// Runtime of the movie in minutes.
    pub runtime: u64,
    /// List of spoken languages in the movie.
    pub spoken_languages: Vec<Language>,
    /// Status of the movie (e.g., Released, Post Production).
    pub status: String,
    /// Tagline of the movie.
    pub tagline: String,
    /// Optional credits information if requested.
    pub credits: Option<MovieCredits>,
}

/// Represents credits information for a movie, including cast and crew.
#[derive(Debug, Deserialize)]
pub struct MovieCredits {
    /// Unique identifier for the movie.
    pub id: Option<u64>,
    /// List of cast members.
    pub cast: Vec<MovieCreditCast>,
    /// List of crew members.
    pub crew: Vec<MovieCreditCrew>,
}

/// Represents a cast member in a movie.
#[derive(Debug, Deserialize)]
pub struct MovieCreditCast {
    #[serde(flatten)]
    pub base: SearchPerson,
    /// Unique cast ID.
    pub cast_id: u64,
    /// Character played by the cast member.
    pub character: String,
    /// Unique credit ID.
    pub credit_id: String,
    /// Order in which the cast member appears in credits.
    pub order: u64,
}

/// Represents a crew member in a movie.
#[derive(Debug, Deserialize)]
pub struct MovieCreditCrew {
    #[serde(flatten)]
    pub base: SearchPerson,
    /// Unique credit ID.
    pub credit_id: String,
    /// Department the crew member worked in.
    pub department: String,
    /// Job title of the crew member.
    pub job: String,
}
