#![allow(dead_code)]
use serde::Deserialize;

use super::{common::Gender, movie::BaseMovie, socials::Socials};

#[derive(Debug, Deserialize)]
pub struct PersonDetails {
    pub adult: bool,
    pub also_known_as: Vec<String>,
    pub biography: String,
    pub birthday: String,
    pub deathday: Option<String>,
    pub gender: Gender,
    pub homepage: Option<String>,
    pub id: u64,
    pub imdb_id: String,
    pub known_for_department: String,
    pub name: String,
    pub place_of_birth: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub credits: Option<PersonCredits>,
    pub external_ids: Option<Socials>,
}

#[derive(Debug, Deserialize)]
pub struct SearchPerson {
    pub adult: bool,
    pub gender: Gender,
    pub id: u64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub media_type: Option<String>,
    pub popularity: f64,
    pub profile_path: Option<String>,
    // This won't be useful so im not going to bother deserializing it
    pub known_for: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct PersonCredits {
    pub cast: Vec<PersonCreditCast>,
    pub crew: Vec<PersonCreditCrew>,
}

/// Movie credits that a given person has with the cast role
#[derive(Debug, Deserialize)]
pub struct PersonCreditCast {
    #[serde(flatten)]
    pub base: BaseMovie,
    pub genre_ids: Vec<u64>,
    pub character: String,
    pub credit_id: String,
    pub order: u64,
}

/// Movie credits that a given person has with the crew role
#[derive(Debug, Deserialize)]
pub struct PersonCreditCrew {
    #[serde(flatten)]
    pub base: BaseMovie,
    pub genre_ids: Vec<u64>,
    pub credit_id: String,
    pub department: String,
    pub job: String,
}
