use serde::{Deserialize, Serialize};

/// Represents a genre
#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
    /// Unique identifier for the genre.
    pub id: u64,
    /// Name of the genre.
    pub name: String,
}

/// Represents a list of movies retrieved from a search query.
#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedSearchResult {
    /// Current page number of results.
    pub page: u64,
    /// List of movies matching the search criteria.
    pub results: Vec<SearchMovie>,
    /// Total number of pages available.
    pub total_pages: u64,
    /// Total number of results found.
    pub total_results: u64,
}

/// Represents a movie in a search result.
#[derive(Debug, Deserialize, Serialize)]
pub struct SearchMovie {
    /// Indicates whether the movie is for adults.
    pub adult: bool,
    /// Path to the movie's backdrop image.
    pub backdrop_path: Option<String>,
    /// List of genre IDs associated with the movie.
    pub genre_ids: Vec<u64>,
    /// Unique identifier for the movie.
    pub id: u64,
    /// Original language of the movie.
    pub original_language: String,
    /// Original title of the movie.
    pub original_title: String,
    /// Brief overview of the movie.
    pub overview: Option<String>,
    /// Popularity score of the movie.
    pub popularity: f64,
    /// Path to the movie's poster image.
    pub poster_path: Option<String>,
    /// Release date of the movie.
    pub release_date: String,
    /// Title of the movie.
    pub title: String,
    /// Indicates whether the movie is a video.
    pub video: bool,
    /// Average vote score.
    pub vote_average: f64,
    /// Total number of votes received.
    pub vote_count: u64,
}

/// Represents credits information for a movie, including cast and crew.
#[derive(Debug, Deserialize, Serialize)]
pub struct MovieCredits {
    /// Unique identifier for the movie.
    pub id: Option<u64>,
    /// List of cast members.
    pub cast: Vec<Cast>,
    /// List of crew members.
    pub crew: Vec<Crew>,
}

/// Represents a cast member in a movie.
#[derive(Debug, Deserialize, Serialize)]
pub struct Cast {
    /// Indicates whether the cast member is an adult.
    pub adult: bool,
    /// Gender of the cast member (if available).
    pub gender: Option<u8>,
    /// Unique identifier for the cast member.
    pub id: u64,
    /// Department the cast member is known for.
    pub known_for_department: String,
    /// Name of the cast member.
    pub name: String,
    /// Original name of the cast member.
    pub original_name: String,
    /// Popularity score of the cast member.
    pub popularity: f32,
    /// Path to the cast member's profile image.
    pub profile_path: Option<String>,
    /// Unique cast ID.
    pub cast_id: u64,
    /// Character played by the cast member.
    pub character: String,
    /// Unique credit ID.
    pub credit_id: String,
    /// Order in which the cast member appears in credits.
    pub order: u64,
}

/// Represents a crew member in a movie.
#[derive(Debug, Deserialize, Serialize)]
pub struct Crew {
    /// Indicates whether the crew member is an adult.
    pub adult: bool,
    /// Gender of the crew member (if available).
    pub gender: Option<u8>,
    /// Unique identifier for the crew member.
    pub id: u64,
    /// Department the crew member is known for.
    pub known_for_department: String,
    /// Name of the crew member.
    pub name: String,
    /// Original name of the crew member.
    pub original_name: String,
    /// Popularity score of the crew member.
    pub popularity: f32,
    /// Path to the crew member's profile image.
    pub profile_path: Option<String>,
    /// Unique credit ID.
    pub credit_id: String,
    /// Department the crew member worked in.
    pub department: String,
    /// Job title of the crew member.
    pub job: String,
}

/// Represents a production company involved in a movie.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProductionCompany {
    /// Unique identifier for the company.
    pub id: u64,
    /// Path to the company's logo.
    pub logo_path: Option<String>,
    /// Name of the company.
    pub name: String,
    /// Country of origin.
    pub origin_country: String,
}

/// Represents a country involved in movie production.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProductionCountry {
    /// ISO 3166-1 country code.
    pub iso_3166_1: String,
    /// Name of the country.
    pub name: String,
}

/// Represents a language spoken in a movie.
#[derive(Debug, Deserialize, Serialize)]
pub struct Language {
    /// English name of the language.
    pub english_name: String,
    /// ISO 639-1 language code.
    pub iso_639_1: String,
    /// Name of the language.
    pub name: String,
}

/// Represents a collection of movies
#[derive(Debug, Deserialize, Serialize)]
pub struct Collection {
    /// The collection ID
    pub id: u64,
    /// The collection name
    pub name: String,
    /// The collection's poster image path
    pub poster_path: String,
    /// The collection's backdrop image path
    pub backdrop_path: String,
}

/// Represents detailed information about a movie.
#[derive(Debug, Deserialize, Serialize)]
pub struct MovieDetails {
    /// Indicates whether the movie is for adults.
    pub adult: bool,
    /// Path to the movie's backdrop image.
    pub backdrop_path: Option<String>,
    /// Collection the movie belongs to, if any.
    pub belongs_to_collection: Option<Collection>,
    /// Budget of the movie in dollars.
    pub budget: u64,
    /// List of genres associated with the movie.
    pub genres: Vec<Genre>,
    /// Homepage URL of the movie.
    pub homepage: String,
    /// Unique identifier for the movie.
    pub id: u64,
    /// IMDb identifier for the movie.
    pub imdb_id: String,
    /// Original language of the movie.
    pub original_language: String,
    /// Original title of the movie.
    pub original_title: String,
    /// Overview of the movie.
    pub overview: String,
    /// Popularity score of the movie.
    pub popularity: f32,
    /// Path to the movie's poster image.
    pub poster_path: String,
    /// List of production companies involved.
    pub production_companies: Vec<ProductionCompany>,
    /// List of countries where the movie was produced.
    pub production_countries: Vec<ProductionCountry>,
    /// Release date of the movie.
    pub release_date: String,
    /// Revenue generated by the movie in dollars.
    pub revenue: u64,
    /// Runtime of the movie in minutes.
    pub runtime: u64,
    /// List of spoken languages in the movie.
    pub spoken_languages: Vec<Language>,
    /// Status of the movie (e.g., Released, Post Production).
    pub status: String,
    /// Tagline of the movie.
    pub tagline: String,
    /// Title of the movie.
    pub title: String,
    /// Indicates whether the movie is a video.
    pub video: bool,
    /// Average vote score.
    pub vote_average: f64,
    /// Total number of votes received.
    pub vote_count: u64,
    /// Optional credits information if requested.
    pub credits: Option<MovieCredits>,
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonDetails {
    pub adult: bool,
    pub also_known_as: Vec<String>,
    pub biography: String,
    pub birthday: String,
    pub deathday: String,
    pub gender: u8,
    pub homepage: String,
    pub id: u64,
    pub imdb_id: String,
    pub known_for_department: String,
    pub name: String,
    pub place_of_birth: String,
    pub popularity: f64,
    pub profile_path: Option<String>,
    pub credits: Option<MovieCredits>,
    pub external_ids: Option<Socials>,
}
