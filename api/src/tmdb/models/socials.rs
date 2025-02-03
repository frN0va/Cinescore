#![allow(dead_code)]
use serde::Deserialize;

/// Represents a collection of external social media and database IDs associated with a person.
#[allow(clippy::doc_markdown)]
#[derive(Debug, Deserialize)]
pub struct Socials {
    /// The person's Freebase MID identifier, if available.
    pub freebase_mid: Option<String>,
    /// The person's Freebase ID, if available.
    pub freebase_id: Option<String>,
    /// The person's IMDb ID, if available.
    pub imdb_id: Option<String>,
    /// The person's TVRage ID, if available.
    pub tvrage_id: Option<u64>,
    /// The person's Wikidata ID, if available.
    pub wikidata_id: Option<String>,
    /// The person's Facebook username or ID, if available.
    pub facebook_id: Option<String>,
    /// The person's Instagram username, if available.
    pub instagram_id: Option<String>,
    /// The person's TikTok username, if available.
    pub tiktok_id: Option<String>,
    /// The person's Twitter username, if available.
    pub twitter_id: Option<String>,
    /// The person's YouTube channel ID, if available.
    pub youtube_id: Option<String>,
}
