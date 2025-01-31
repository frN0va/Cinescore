use std::collections::HashMap;

use crate::{
    frontend_models::FrontendMovieDetails,
    generate_request_struct,
    tmdb::{client::TMDBClient, models::MovieDetails},
};

/// A trait for querying movie details from the TMDB API.
#[allow(dead_code)]
pub trait MovieDetailsQuery {
    /// Returns a mutable reference to the query parameters.
    ///
    /// This allows modifying the parameters before making a request.
    ///
    /// # Returns
    ///
    /// A mutable reference to a `HashMap` containing the query parameters.
    fn params(&mut self) -> &mut HashMap<&'static str, String>;

    /// Asynchronously fetches the movie details from the TMDB API.
    ///
    /// # Arguments
    ///
    /// * `client` - The `TMDBClient` instance used to make the API request.
    /// * `id` - The ID of the movie to fetch details for.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `FrontendMovieDetails` on success or a `reqwest::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return a `reqwest::Error` if the request fails or if the response cannot be parsed.
    async fn fetch(
        self,
        client: &TMDBClient,
        id: u64,
    ) -> Result<FrontendMovieDetails, reqwest::Error>;

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
        self.params().insert("language", language);
        self
    }

    /// Sets the `append_to_response` query parameter for the request.
    ///
    /// # Arguments
    ///
    /// * `append` - A `&str` representing the additional fields to include in the response.
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn append_to_response(mut self, append: &str) -> Self
    where
        Self: Sized,
    {
        self.params()
            .insert("append_to_response", append.to_string());
        self
    }
}

generate_request_struct!(
    MovieDetailsRequest,
    "Request struct for fetching movie details."
);

impl MovieDetailsQuery for MovieDetailsRequest {
    /// Returns a mutable reference to the query parameters for the movie details request.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `HashMap` containing query parameters.
    fn params(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.params
    }

    /// Asynchronously fetches the movie details from the TMDB API for the given movie ID.
    ///
    /// # Arguments
    ///
    /// * `client` - The `TMDBClient` instance used to make the API request.
    /// * `id` - The ID of the movie to fetch details for.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `FrontendMovieDetails` on success or a `reqwest::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return a `reqwest::Error` if the request fails or if deserialization fails.
    async fn fetch(
        self,
        client: &TMDBClient,
        id: u64,
    ) -> Result<FrontendMovieDetails, reqwest::Error> {
        let response = client
            .get::<MovieDetails>(&format!("movie/{}", id), self.params)
            .await?;

        Ok(FrontendMovieDetails::from(response))
    }
}
