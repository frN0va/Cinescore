use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Socials {
    pub freebase_mid: Option<String>,
    pub freebase_id: Option<String>,
    pub imdb_id: Option<String>,
    pub tvrage_id: Option<u64>,
    pub wikidata_id: Option<String>,
    pub facebook_id: Option<String>,
    pub instagram_id: Option<String>,
    pub tiktok_id: Option<String>,
    pub twitter_id: Option<String>,
    pub youtube_id: Option<String>,
}
