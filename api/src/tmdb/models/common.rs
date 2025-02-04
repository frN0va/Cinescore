#![allow(dead_code)]

use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};
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
#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Gender {
    /// 0: Not specified
    NotSpecified = 0,
    /// 1: Male
    Male = 1,
    /// 2: Female
    Female = 2,
    /// 3: Non-binary
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

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: Option<String> = Option::deserialize(deserializer)?;

    match date_str {
        Some(str) => {
            if str.is_empty() {
                Ok(None)
            } else {
                NaiveDate::parse_from_str(&str, "%Y-%m-%d")
                    .map(Some)
                    .map_err(serde::de::Error::custom)
            }
        }
        None => Ok(None),
    }
}
