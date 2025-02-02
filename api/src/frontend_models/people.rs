use serde::Serialize;

use crate::tmdb::models::{
    pagination::PaginatedSearchResult,
    person::{PersonDetails, SearchPerson},
};

use super::{common::get_image_url, credits::FrontendCredits, socials::FrontendSocials};

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
