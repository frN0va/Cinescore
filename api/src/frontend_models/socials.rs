use serde::Serialize;

use crate::tmdb::models::socials::Socials;

#[derive(Debug, Serialize)]
pub struct FrontendSocials {
    imdb: Option<String>,
    facebook: Option<String>,
    instagram: Option<String>,
    tiktok: Option<String>,
    twitter: Option<String>,
}

impl From<Socials> for FrontendSocials {
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
