use serde::Serialize;

use crate::tmdb::models::socials::Socials;

/// This struct contains information on a specific person's social media IDs
#[derive(Debug, Serialize)]
pub struct FrontendSocials {
    imdb: Option<String>,
    facebook: Option<String>,
    instagram: Option<String>,
    tiktok: Option<String>,
    twitter: Option<String>,
}

impl From<Socials> for FrontendSocials {
    /// Converts a [`Socials`] into a [`FrontendSocials`].
    ///
    /// # Arguments
    ///
    /// * `value` - The source [`Socials`] to convert from
    ///
    /// # Returns
    ///
    /// A new [`FrontendSocials`] instance with all fields mapped from the source.
    fn from(value: Socials) -> Self {
        Self {
            imdb: value.imdb_id,
            facebook: value.facebook_id,
            instagram: value.instagram_id,
            tiktok: value.tiktok_id,
            twitter: value.twitter_id,
        }
    }
}
