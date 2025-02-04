#![allow(dead_code)]
use chrono::NaiveDate;
use serde::Deserialize;

use super::{common::Gender, movie::BaseMovie, socials::Socials};

/// Represents detailed information about a person, such as an actor or director.
#[derive(Debug, Deserialize)]
pub struct PersonDetails {
    /// Indicates whether the person is an adult.
    pub adult: bool,
    /// A list of alternative names the person is known by.
    pub also_known_as: Vec<String>,
    /// A biography of the person.
    pub biography: String,
    /// The person's birth date, if available.
    pub birthday: Option<NaiveDate>,
    /// The person's death date, if applicable.
    pub deathday: Option<NaiveDate>,
    /// The person's gender.
    pub gender: Gender,
    /// The URL of the person's homepage, if available.
    pub homepage: Option<String>,
    /// A unique identifier for the person.
    pub id: u64,
    /// The person's IMDb ID, if available.
    #[allow(clippy::doc_markdown)]
    pub imdb_id: Option<String>,
    /// The department the person is best known for (e.g., Acting, Directing).
    pub known_for_department: String,
    /// The person's full name.
    pub name: String,
    /// The place where the person was born, if available.
    pub place_of_birth: Option<String>,
    /// The person's popularity score.
    pub popularity: f64,
    /// The path to the person's profile image, if available.
    pub profile_path: Option<String>,
    /// The person's movie credits, if available.
    pub credits: Option<PersonCredits>,
    /// External IDs for social media and other platforms.
    pub external_ids: Option<Socials>,
}

/// Represents a person in search results with limited details.
#[derive(Debug, Deserialize)]
pub struct SearchPerson {
    /// Indicates whether the person is an adult.
    pub adult: bool,
    /// The person's gender.
    pub gender: Gender,
    /// A unique identifier for the person.
    pub id: u64,
    /// The department the person is best known for, if available.
    pub known_for_department: Option<String>,
    /// The person's full name.
    pub name: String,
    /// The person's original name.
    pub original_name: String,
    /// The type of media associated with the person, if applicable.
    pub media_type: Option<String>,
    /// The person's popularity score.
    pub popularity: f64,
    /// The path to the person's profile image, if available.
    pub profile_path: Option<String>,
    /// This won't be useful so im not going to bother deserializing it
    pub known_for: Option<serde_json::Value>,
}

/// Represents the movie credits of a person.
#[derive(Debug, Deserialize)]
pub struct PersonCredits {
    /// List of movies where the person appeared as a cast member.
    pub cast: Vec<PersonCreditCast>,
    /// List of movies where the person was part of the crew.
    pub crew: Vec<PersonCreditCrew>,
}

/// Represents a cast credit for a person in a movie.
#[derive(Debug, Deserialize)]
pub struct PersonCreditCast {
    /// Basic movie details.
    #[serde(flatten)]
    pub base: BaseMovie,
    /// Genre IDs associated with the movie.
    pub genre_ids: Vec<u64>,
    /// The character played by the person.
    pub character: String,
    /// A unique credit identifier.
    pub credit_id: String,
    /// The order in which the person is credited.
    pub order: u64,
}

/// Represents a crew credit for a person in a movie.
#[derive(Debug, Deserialize)]
pub struct PersonCreditCrew {
    /// Basic movie details.
    #[serde(flatten)]
    pub base: BaseMovie,
    /// Genre IDs associated with the movie.
    pub genre_ids: Vec<u64>,
    /// A unique credit identifier.
    pub credit_id: String,
    /// The department the person worked in (e.g., Directing, Writing).
    pub department: String,
    /// The specific job title the person held (e.g., Director, Producer).
    pub job: String,
}
