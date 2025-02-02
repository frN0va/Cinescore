use serde::Serialize;

use crate::tmdb::models::{
    movie::{MovieCreditCast, MovieCreditCrew, MovieCredits},
    person::{PersonCreditCast, PersonCreditCrew, PersonCredits},
};

use super::common::get_image_url;

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
    /// The URL to the image of the relavant movie poster if this is a credit
    #[serde(rename = "posterUrl")]
    poster_url: Option<String>,
    /// The title of the relavant movie if this is a credit
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
    /// The URL to the image of the relavant movie poster if this is a credit
    #[serde(rename = "posterUrl")]
    poster_url: Option<String>,
    /// The title of the relavant movie if this is a credit
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
