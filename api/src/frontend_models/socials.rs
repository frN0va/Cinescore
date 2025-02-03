use serde::Serialize;

use crate::tmdb::models::socials::Socials;

/// This struct contains information on a specific person's social media IDs
#[allow(clippy::doc_markdown)]
#[derive(Debug, Serialize)]
pub struct FrontendSocials {
    /// The person's IMDb ID, if available.
    imdb: Option<String>,
    /// The person's Facebook username or ID, if available.
    facebook: Option<String>,
    /// The person's Instagram username, if available.
    instagram: Option<String>,
    /// The person's TikTok username, if available.
    tiktok: Option<String>,
    /// The person's Twitter username, if available.
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
