#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents a genre
#[derive(Debug, Deserialize)]
pub struct Genre {
    /// Unique identifier for the genre.
    pub id: u64,
    /// Name of the genre.
    pub name: String,
}

/// Represents a production company involved in a movie.
#[derive(Debug, Deserialize)]
pub struct ProductionCompany {
    /// Unique identifier for the company.
    pub id: u64,
    /// Path to the company's logo.
    pub logo_path: Option<String>,
    /// Name of the company.
    pub name: String,
    /// Country of origin.
    pub origin_country: String,
}

/// Represents a country involved in movie production.
#[derive(Debug, Deserialize)]
pub struct ProductionCountry {
    /// ISO 3166-1 country code.
    pub iso_3166_1: String,
    /// Name of the country.
    pub name: String,
}

/// Represents a language spoken in a movie.
#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    /// English name of the language.
    pub english_name: String,
    /// ISO 639-1 language code.
    pub iso_639_1: String,
    /// Name of the language.
    pub name: String,
}

/// The person's gender represented as a number:
/// - 0: Not specified
/// - 1: Female
/// - 2: Male
/// - 3: Non-binary
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Gender {
    NotSpecified = 0,
    Male = 1,
    Female = 2,
    NonBinary = 3,
}

/// Represents a paginated list retrieved from a search query.
#[derive(Debug, Deserialize)]
pub struct PaginatedSearchResult<T> {
    /// Current page number of results.
    pub page: u64,
    /// List of movies matching the search criteria.
    pub results: Vec<T>,
    /// Total number of pages available.
    pub total_pages: u64,
    /// Total number of results found.
    pub total_results: u64,
}
