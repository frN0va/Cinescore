use crate::{
    frontend_models::movies::FrontendMovieDetails,
    generate_request_struct,
    tmdb::{
        client::{ApiFetchError, TMDBClient},
        models::movie::MovieDetails,
    },
};

use super::traits::{AppendToResponseQueryParam, IdQuery, LanguageQueryParam};

generate_request_struct!(
    MovieDetailsRequest,
    "Request struct for fetching movie details."
);

impl IdQuery<FrontendMovieDetails> for MovieDetailsRequest {
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
    ) -> Result<FrontendMovieDetails, ApiFetchError> {
        tracing::debug!("Fetching movie details for movie ID {}", id);

        let response = client
            .get::<MovieDetails>(&format!("movie/{}", id), self.params)
            .await?;

        Ok(FrontendMovieDetails::from(response))
    }
}

impl AppendToResponseQueryParam for MovieDetailsRequest {}
impl LanguageQueryParam for MovieDetailsRequest {}
