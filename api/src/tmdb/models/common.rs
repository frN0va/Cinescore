#![allow(dead_code)]
use std::{fmt::Display, str::FromStr};

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

/// A simple struct to define a date. There is no error checking for the validity of the date
#[derive(Debug)]
pub struct Date {
    /// Year of the [`Date`]
    year: u16,
    /// Month of the [`Date`]
    month: u8,
    /// Day of the [`Date`]
    day: u8,
}

impl Date {
    /// Construct a new [`Date`] object. This does not check the validity of the date.
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err("Invalid date format: expected YYYY-MM-DD".into());
        }

        let year = parts[0].parse::<u16>().map_err(|_| "Invalid year")?;
        let month = parts[1].parse::<u8>().map_err(|_| "Invalid month")?;
        let day = parts[2].parse::<u8>().map_err(|_| "Invalid day")?;

        Ok(Date::new(year, month, day))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}
