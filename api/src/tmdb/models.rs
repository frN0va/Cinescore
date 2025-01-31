use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct MovieCredits {
    pub id: Option<u64>,
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Cast {
    pub adult: bool,
    pub gender: Option<u8>,
    pub id: u64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f32,
    pub profile_path: Option<String>,
    pub cast_id: u64,
    pub character: String,
    pub credit_id: String,
    pub order: u64,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Crew {
    pub adult: bool,
    pub gender: Option<u8>,
    pub id: u64,
    pub known_for_department: String,
    pub name: String,
    pub original_name: String,
    pub popularity: f32,
    pub profile_path: Option<String>,
    pub credit_id: String,
    pub department: String,
    pub job: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProductionCompany {
    pub id: u64,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Language {
    pub english_name: String,
    pub iso_639_1: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct MovieDetails {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub belongs_to_collection: Option<String>,
    pub budget: u64,
    pub genres: Vec<Genre>,
    pub homepage: String,
    pub id: u64,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f32,
    pub poster_path: String,
    pub production_companies: Vec<ProductionCompany>,
    pub production_countries: Vec<ProductionCountry>,
    pub release_date: String,
    pub revenue: u64,
    pub runtime: u64,
    pub spoken_languages: Vec<Language>,
    pub status: String,
    pub tagline: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: u64,
    // if append_to_response is used
    pub credits: Option<MovieCredits>,
}
