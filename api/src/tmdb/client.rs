use std::collections::HashMap;

use reqwest::{
    header::{ACCEPT, AUTHORIZATION, USER_AGENT},
    Client,
};
use serde::de::DeserializeOwned;

/// Base URL for image resources from TMDB.
pub const IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p/original/";

/// Base URL for the TMDB API.
pub const API_BASE_URL: &str = "https://api.themoviedb.org/3";

/// A struct representing a client for interacting with the TMDB API.
#[derive(Debug, Clone)]
pub struct TMDBClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl TMDBClient {
    /// Creates a new `TMDBClient` with the provided API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key used for authentication with the TMDB API.
    ///
    /// # Returns
    ///
    /// A new `TMDBClient` instance with the provided API key and default base URL.
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: API_BASE_URL.to_owned(),
        }
    }

    /// Makes an asynchronous GET request to the TMDB API.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The specific endpoint to hit on the TMDB API.
    /// * `params` - A `HashMap` of query parameters to append to the request.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type that the response will be deserialized into. It must implement `DeserializeOwned`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response of type `T` on success, or a `reqwest::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return a `reqwest::Error` if the request fails or if deserialization fails.
    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: HashMap<&str, String>,
    ) -> Result<T, reqwest::Error> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = self
            .client
            .get(url)
            .query(&params)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(ACCEPT, "application/json")
            .header(
                USER_AGENT,
                format!("Cinescore {}", env!("CARGO_PKG_VERSION")),
            )
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(response)
    }
}
