use serde::Serialize;

use crate::tmdb::models::{
    movie::{MovieCreditCast, MovieCreditCrew, MovieCredits},
    person::{PersonCreditCast, PersonCreditCrew, PersonCredits},
};

use super::common::get_image_url;

/// Container for movie credits
#[derive(Debug, Serialize)]
pub struct FrontendMovieCredits {
    /// A list of cast members in a movie's credits
    cast: Vec<FrontendMovieCastMember>,
    /// A list of crew members in a movie's credits
    crew: Vec<FrontendMovieCrewMember>,
}

/// Container for person credits (filmography)
#[derive(Debug, Serialize)]
pub struct FrontendPersonCredits {
    /// A list of cast credits a person has.
    cast: Vec<FrontendPersonCastCredit>,
    /// A list of crew credits a person has.
    crew: Vec<FrontendPersonCrewCredit>,
}

/// Common fields shared between all crew member types
#[derive(Debug, Serialize)]
pub struct CrewBase {
    /// Unique identifier for the crew member
    pub id: u64,
    /// Department in which the crew member worked
    pub department: String,
}

/// Common fields shared between all cast member types
#[derive(Debug, Serialize)]
pub struct CastBase {
    /// Unique identifier for the cast member
    pub id: u64,
    /// Character played by the cast member
    pub character: String,
}

/// Represents a crew member in the context of a movie's credits
#[derive(Debug, Serialize)]
pub struct FrontendMovieCrewMember {
    /// Basic crew member details.
    #[serde(flatten)]
    pub base: CrewBase,
    /// Name of the crew member
    pub name: String,
    /// URL to the crew member's profile picture
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
}

/// Represents a cast member in the context of a movie's credits
#[derive(Debug, Serialize)]
pub struct FrontendMovieCastMember {
    /// Basic cast member details.
    #[serde(flatten)]
    pub base: CastBase,
    /// Name of the cast member
    pub name: String,
    /// URL to the cast member's profile picture
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
}

/// Represents a movie credit in a person's filmography (as crew)
#[derive(Debug, Serialize)]
pub struct FrontendPersonCrewCredit {
    /// Basic crew member details.
    #[serde(flatten)]
    pub base: CrewBase,
    /// Title of the movie
    pub title: String,
    /// URL to the movie's poster
    #[serde(rename = "posterUrl")]
    pub poster_url: String,
}

/// Represents a movie credit in a person's filmography (as cast)
#[derive(Debug, Serialize)]
pub struct FrontendPersonCastCredit {
    /// Basic cast member details.
    #[serde(flatten)]
    pub base: CastBase,
    /// Title of the movie
    pub title: String,
    /// URL to the movie's poster
    #[serde(rename = "posterUrl")]
    pub poster_url: String,
}

impl From<MovieCreditCrew> for FrontendMovieCrewMember {
    /// Converts a [`MovieCreditCrew`] into a [`FrontendMovieCrewMember`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`MovieCreditCrew`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendMovieCrewMember`] instance with all fields mapped from the source.
    fn from(value: MovieCreditCrew) -> Self {
        Self {
            base: CrewBase {
                id: value.base.id,
                department: value.base.known_for_department.unwrap_or("N/A".to_owned()),
            },
            name: value.base.name,
            icon_url: get_image_url(value.base.profile_path),
        }
    }
}

impl From<MovieCreditCast> for FrontendMovieCastMember {
    /// Converts a [`MovieCreditCast`] into a [`FrontendMovieCastMember`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`MovieCreditCast`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendMovieCastMember`] instance with all fields mapped from the source.
    fn from(value: MovieCreditCast) -> Self {
        Self {
            base: CastBase {
                id: value.base.id,
                character: value.character,
            },
            name: value.base.name,
            icon_url: get_image_url(value.base.profile_path),
        }
    }
}

impl From<PersonCreditCrew> for FrontendPersonCrewCredit {
    /// Converts a [`PersonCreditCrew`] into a [`FrontendPersonCrewCredit`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`PersonCreditCrew`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendPersonCrewCredit`] instance with all fields mapped from the source.
    fn from(value: PersonCreditCrew) -> Self {
        Self {
            base: CrewBase {
                id: value.base.id,
                department: value.department,
            },
            title: value.base.title,
            poster_url: get_image_url(value.base.poster_path),
        }
    }
}

impl From<PersonCreditCast> for FrontendPersonCastCredit {
    /// Converts a [`PersonCreditCast`] into a [`FrontendPersonCastCredit`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`PersonCreditCast`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendPersonCastCredit`] instance with all fields mapped from the source.
    fn from(value: PersonCreditCast) -> Self {
        Self {
            base: CastBase {
                id: value.base.id,
                character: value.character,
            },
            title: value.base.title,
            poster_url: get_image_url(value.base.poster_path),
        }
    }
}

/// Converts [`MovieCredits`] into [`FrontendMovieCredits`] for frontend representation.
impl From<MovieCredits> for FrontendMovieCredits {
    /// Converts a [`MovieCredits`] into a [`FrontendMovieCredits`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`MovieCredits`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendMovieCredits`] instance with all fields mapped from the source.
    fn from(value: MovieCredits) -> Self {
        Self {
            cast: value
                .cast
                .into_iter()
                .map(FrontendMovieCastMember::from)
                .collect(),
            crew: value
                .crew
                .into_iter()
                .map(FrontendMovieCrewMember::from)
                .collect(),
        }
    }
}

impl From<PersonCredits> for FrontendPersonCredits {
    /// Converts a [`PersonCredits`] into a [`FrontendPersonCredits`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`PersonCredits`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendPersonCredits`] instance with all fields mapped from the source.
    fn from(value: PersonCredits) -> Self {
        Self {
            cast: value
                .cast
                .into_iter()
                .map(FrontendPersonCastCredit::from)
                .collect(),
            crew: value
                .crew
                .into_iter()
                .map(FrontendPersonCrewCredit::from)
                .collect(),
        }
    }
}
