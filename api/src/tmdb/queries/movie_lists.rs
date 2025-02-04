use crate::{
    frontend_models::movies::FrontendMovieList,
    generate_request_struct,
    tmdb::{
        client::{ApiFetchError, TMDBClient},
        models::{common::PaginatedSearchResult, movie::SearchMovie},
    },
};

use super::traits::{
    IncludeAdultQueryParam, IncludeVideoQueryParam, LanguageQueryParam, PageQueryParam,
    PrimaryReleaseDateQueryParam, PrimaryReleaseYearQueryParam, Query, RegionQueryParam,
    SortByQueryParam, YearQueryParam,
};

generate_request_struct!(
    MovieListTrendingRequest,
    "Request struct for fetching today's trending movies."
);

generate_request_struct!(
    MovieListNowPlayingRequest,
    "Request struct for fetching movies that are now playing in theatures."
);

generate_request_struct!(
    DiscoverMoviesRequest,
    "Request struct for fetching movies with a set of filters."
);

impl Query<FrontendMovieList> for MovieListTrendingRequest {
    /// Asynchronously fetches the trending movies list from the TMDB API.
    ///
    /// # Arguments
    ///
    /// * `client` - The `TMDBClient` instance used to make the API request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `FrontendMovieList` on success or a `reqwest::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return a `reqwest::Error` if the request fails or if deserialization fails.
    async fn fetch(self, client: &TMDBClient) -> Result<FrontendMovieList, ApiFetchError> {
        log::debug!("Fetching daily trending movies from TMDB API");

        let response = client
            .get::<PaginatedSearchResult<SearchMovie>>("/trending/movie/day", self.params)
            .await?;

        Ok(FrontendMovieList::from(response))
    }
}

impl Query<FrontendMovieList> for MovieListNowPlayingRequest {
    /// Asynchronously fetches the now playing movies list from the TMDB API.
    ///
    /// # Arguments
    ///
    /// * `client` - The `TMDBClient` instance used to make the API request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `FrontendMovieList` on success or a `reqwest::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return a `reqwest::Error` if the request fails or if deserialization fails.
    async fn fetch(self, client: &TMDBClient) -> Result<FrontendMovieList, ApiFetchError> {
        log::debug!("Fetching now playing movies from TMDB API");

        let response = client
            .get::<PaginatedSearchResult<SearchMovie>>("movie/now_playing", self.params)
            .await?;

        Ok(FrontendMovieList::from(response))
    }
}

impl Query<FrontendMovieList> for DiscoverMoviesRequest {
    /// Asynchronously fetches movies with a set of filters from the TMDB API.
    ///
    /// # Arguments
    ///
    /// * `client` - The `TMDBClient` instance used to make the API request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `FrontendMovieList` on success or a `reqwest::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return a `reqwest::Error` if the request fails or if deserialization fails.
    async fn fetch(self, client: &TMDBClient) -> Result<FrontendMovieList, ApiFetchError> {
        log::debug!("Fetching discover movies from TMDB API");

        let response = client
            .get::<PaginatedSearchResult<SearchMovie>>("discover/movie", self.params)
            .await?;

        Ok(FrontendMovieList::from(response))
    }
}

impl LanguageQueryParam for MovieListNowPlayingRequest {}
impl PageQueryParam for MovieListNowPlayingRequest {}
impl RegionQueryParam for MovieListNowPlayingRequest {}

impl LanguageQueryParam for MovieListTrendingRequest {}

// TODO: im missing most of these but dont require them yet
impl IncludeAdultQueryParam for DiscoverMoviesRequest {}
impl LanguageQueryParam for DiscoverMoviesRequest {}
impl RegionQueryParam for DiscoverMoviesRequest {}
impl PageQueryParam for DiscoverMoviesRequest {}
impl PrimaryReleaseYearQueryParam for DiscoverMoviesRequest {}
impl YearQueryParam for DiscoverMoviesRequest {}
impl PrimaryReleaseDateQueryParam for DiscoverMoviesRequest {}
impl IncludeVideoQueryParam for DiscoverMoviesRequest {}
impl SortByQueryParam for DiscoverMoviesRequest {}
