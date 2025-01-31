use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct MovieListSearch {
    pub page: u64,
    pub results: Vec<SearchMovie>,
    pub total_pages: u64,
    pub total_results: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct SearchMovie {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<u64>,
    pub id: u64,
    pub original_language: String,
    pub original_title: String,
    pub overview: Option<String>,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub release_date: String, // ToDo: Date Type
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: u64,
}
