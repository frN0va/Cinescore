use serde::{Deserialize, Serialize};

/// Represents a genre
#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
    /// Unique identifier for the genre.
    pub id: u64,
    /// Name of the genre.
    pub name: String,
}

/// Represents a production company involved in a movie.
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize)]
pub struct ProductionCountry {
    /// ISO 3166-1 country code.
    pub iso_3166_1: String,
    /// Name of the country.
    pub name: String,
}

/// Represents a language spoken in a movie.
#[derive(Debug, Deserialize, Serialize)]
pub struct Language {
    /// English name of the language.
    pub english_name: String,
    /// ISO 639-1 language code.
    pub iso_639_1: String,
    /// Name of the language.
    pub name: String,
}
