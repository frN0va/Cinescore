use std::collections::HashMap;

use crate::{
    frontend_models::movies::FrontendMovieList,
    generate_request_struct,
    tmdb::{
        client::{ApiFetchError, TMDBClient},
        models::{common::PaginatedSearchResult, movie::SearchMovie},
    },
};

/// A trait for querying a list of movies from the TMDB API.
#[allow(dead_code)]
pub trait MovieListQuery {
    /// Returns a mutable reference to the query parameters.
    ///
    /// This allows modifying the parameters before making a request.
    ///
    /// # Returns
    ///
    /// A mutable reference to a `HashMap` containing the query parameters.
    fn params(&mut self) -> &mut HashMap<&'static str, String>;

    /// Asynchronously fetches a list of movies from the TMDB API.
    ///
    /// # Arguments
    ///
    /// * `client` - The `TMDBClient` instance used to make the API request.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `FrontendMovieList` on success or a `reqwest::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return a `reqwest::Error` if the request fails or if the response cannot be parsed.
    async fn fetch(self, client: &TMDBClient) -> Result<FrontendMovieList, ApiFetchError>;

    /// Sets the `language` query parameter for the request.
    ///
    /// # Arguments
    ///
    /// * `language` - A `String` representing the language to filter results by.
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn language(mut self, language: String) -> Self
    where
        Self: Sized,
    {
        log::debug!("Inserting language `{}` into query parameters", language);

        self.params().insert("language", language);
        self
    }

    /// Sets the `page` query parameter for the request.
    ///
    /// # Arguments
    ///
    /// * `page` - A `u32` representing the page number for paginated results.
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn page(mut self, page: u32) -> Self
    where
        Self: Sized,
    {
        log::debug!("Inserting page `{}` into query parameters", page);

        self.params().insert("page", page.to_string());
        self
    }

    /// Sets the `region` query parameter for the request.
    ///
    /// # Arguments
    ///
    /// * `region` - A `String` representing the region code for region-based filtering.
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn region(mut self, region: String) -> Self
    where
        Self: Sized,
    {
        log::debug!("Inserting region `{}` into query parameters", region);

        self.params().insert("region", region);
        self
    }
}

generate_request_struct!(
    MovieListTrendingRequest,
    "Request struct for fetching today's trending movies."
);

impl MovieListQuery for MovieListTrendingRequest {
    /// Returns a mutable reference to the query parameters for the trending movies request.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `HashMap` containing query parameters.
    fn params(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.params
    }

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
        log::info!("Fetching daily trending movies from TMDB API");

        let response = client
            .get::<PaginatedSearchResult<SearchMovie>>("/trending/movie/day", self.params)
            .await?;

        Ok(FrontendMovieList::from(response))
    }
}

generate_request_struct!(
    MovieListNowPlayingRequest,
    "Request struct for fetching movies that are now playing in theatures."
);

impl MovieListQuery for MovieListNowPlayingRequest {
    /// Returns a mutable reference to the query parameters for the now playing movies request.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `HashMap` containing query parameters.
    fn params(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.params
    }

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
        log::info!("Fetching now playing movies from TMDB API");

        let response = client
            .get::<PaginatedSearchResult<SearchMovie>>("movie/now_playing", self.params)
            .await?;

        Ok(FrontendMovieList::from(response))
    }
}
