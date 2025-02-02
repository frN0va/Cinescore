use std::collections::HashMap;

use crate::{
    frontend_models::FrontendMovieDetails,
    generate_request_struct,
    tmdb::{client::TMDBClient, models::movie::MovieDetails},
};

use super::common::DetailsQuery;

generate_request_struct!(
    MovieDetailsRequest,
    "Request struct for fetching movie details."
);

impl DetailsQuery<FrontendMovieDetails> for MovieDetailsRequest {
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
        log::info!("Fetching movie details for movie ID {}", id);

        let response = client
            .get::<MovieDetails>(&format!("movie/{}", id), self.params)
            .await?;

        Ok(FrontendMovieDetails::from(response))
    }
}
