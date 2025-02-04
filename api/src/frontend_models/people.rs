use serde::Serialize;

use crate::tmdb::models::{
    common::{Date, Gender, PaginatedSearchResult},
    person::{PersonDetails, SearchPerson},
};

use super::{common::get_image_url, credits::FrontendPersonCredits, socials::FrontendSocials};

/// Represents detailed information about a person in the entertainment industry.
/// This struct contains comprehensive biographical and professional information,
/// including their work history and social media presence.
#[derive(Debug, Serialize)]
pub struct FrontendPersonDetails {
    /// A biographical description of the person.
    pub biography: String,
    /// The person's date of birth in ISO 8601 format (YYYY-MM-DD).
    pub birthday: Option<Date>,
    /// The person's date of death in ISO 8601 format (YYYY-MM-DD), if applicable.
    pub deathday: Option<Date>,
    /// The person's gender represented as a number:
    pub gender: Gender,
    /// Unique identifier for the person.
    pub id: u64,
    /// The department or field they are primarily known for working in
    /// (e.g., "Acting", "Directing", "Production").
    #[serde(rename = "knownForDepartment")]
    pub known_for_department: String,
    /// The person's full name.
    pub name: String,
    /// The location where the person was born.
    #[serde(rename = "placeOfBirth")]
    pub place_of_birth: String,
    /// URL to the person's profile image.
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    /// The person's movie credits, both as cast and crew member.
    /// May be None if credit information is not requested or available.
    pub credits: Option<FrontendPersonCredits>,
    /// The person's social media profiles and external identifiers.
    /// May be None if social media information is not requested or available.
    pub socials: Option<FrontendSocials>,
}

/// A collection of people returned from a search or listing operation.
/// This struct is typically used when displaying search results or browsing
/// through lists of people in the entertainment industry.
#[derive(Debug, Serialize)]
pub struct FrontendPeopleList {
    /// Vector of brief person listings.
    pub people: Vec<FrontendPersonListing>,
}

/// A brief overview of a person, typically used in search results or lists.
/// Contains essential identifying information without full biographical details.
#[derive(Debug, Serialize)]
pub struct FrontendPersonListing {
    /// Unique identifier for the person.
    pub id: u64,
    /// The person's full name.
    pub name: String,
    /// The person's gender represented as a number:
    pub gender: Gender,
    /// The department or field they are primarily known for working in
    /// (e.g., "Acting", "Directing", "Production").
    pub department: String,
    /// URL to the person's profile image.
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
}

/// Conversion implementation to create frontend-ready person details
/// from the internal person details model.
impl From<PersonDetails> for FrontendPersonDetails {
    /// Converts a [`PersonDetails`] into a [`FrontendPersonDetails`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`PersonDetails`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendPersonDetails`] instance with all fields mapped from the source.
    fn from(value: PersonDetails) -> Self {
        Self {
            biography: value.biography,
            birthday: value.birthday,
            deathday: value.deathday,
            gender: value.gender,
            id: value.id,
            known_for_department: value.known_for_department,
            name: value.name,
            place_of_birth: value.place_of_birth.unwrap_or("N/A".to_owned()),
            icon_url: get_image_url(value.profile_path),
            credits: value.credits.map(FrontendPersonCredits::from),
            socials: value.external_ids.map(FrontendSocials::from),
        }
    }
}

/// Conversion implementation to create a frontend-ready person listing
/// from a search result.
impl From<SearchPerson> for FrontendPersonListing {
    /// Converts a [`SearchPerson`] into a [`FrontendPersonListing`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`SearchPerson`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendPersonListing`] instance with all fields mapped from the source.
    fn from(value: SearchPerson) -> Self {
        Self {
            id: value.id,
            name: value.name,
            gender: value.gender,
            department: value.known_for_department.unwrap_or("N/A".to_owned()),
            icon_url: get_image_url(value.profile_path),
        }
    }
}

/// Conversion implementation to create a frontend-ready people list
/// from paginated search results.
impl From<PaginatedSearchResult<SearchPerson>> for FrontendPeopleList {
    /// Converts a [`PaginatedSearchResult<SearchPerson>`] into a [`FrontendPeopleList`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`PaginatedSearchResult<SearchPerson>`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendPeopleList`] instance containing converted listings for each person
    /// in the search results.
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
