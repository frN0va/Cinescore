use std::collections::HashMap;

use axum::response::IntoResponse;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, USER_AGENT},
    Client,
};
use serde::de::DeserializeOwned;

/// Base URL for image resources from TMDB.
pub const IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p/original/";

/// Base URL for the TMDB API.
pub const API_BASE_URL: &str = "https://api.themoviedb.org/3";

/// Represents various errors that can be encountered while querying the TMDB API
#[derive(Debug, thiserror::Error)]
pub enum ApiFetchError {
    /// An error that was encountered while actively fetching the data. This may be something like
    /// a network error (404, 500) on the part of TMDB or it may be a malformed response body
    #[error("Error while fetching data from TMDB. Got status: {0}")]
    Request(#[from] reqwest::Error),
    /// An error that was triggered due to an endpoint expecting a query parameter to be present
    /// when it was excluded from the request
    #[error("Missing query parameter `{param}` in request `{request_name}`")]
    MissingQueryParam {
        /// The query param which is missing
        param: &'static str,
        /// The name of the request that was expecting the param
        request_name: &'static str,
    },
    /// An error that occured during the deserialization of the JSON returned from the TMDB API
    #[error("Deserialization error at {path}: {source}")]
    Deserialization {
        /// The path in the JSON where the error occured
        path: String,
        /// The source error
        #[source]
        source: serde_json::Error,
    },
}

impl IntoResponse for ApiFetchError {
    /// Converts an [`ApiFetchError`] into an [`axum::response::Response`]
    fn into_response(self) -> axum::response::Response {
        log::error!("{}", self);

        if let Self::Request(e) = self {
            if let Some(status_code) = e.status() {
                log::error!("Errored with status code: {}", status_code)
            }
        }

        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Error encountered while fetching data from the TMDB API",
        )
            .into_response()
    }
}

/// A struct representing a client for interacting with the TMDB API.
#[derive(Debug, Clone)]
pub struct TMDBClient {
    /// The [`reqwest::Client`] to use for sending requests
    client: Client,
    /// Your TMDB API key
    api_key: String,
    /// The API's base URL
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
        log::debug!("Creating TMDBClient with provided API key");
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
    ) -> Result<T, ApiFetchError> {
        let url = format!("{}/{}", self.base_url, endpoint);
        log::info!("Making GET request to URL: {}", url);

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
            .await?;

        if response.status().is_success() {
            log::debug!("Successful response from TMDB API");
        } else {
            log::warn!("Non-success status code received: {}", response.status());
        }

        let response_as_value = response.json::<serde_json::Value>().await?;

        let json_string = response_as_value.to_string();

        let mut deserializer = serde_json::Deserializer::from_str(&json_string);

        match serde_path_to_error::deserialize::<&mut serde_json::Deserializer<_>, T>(
            &mut deserializer,
        ) {
            Ok(v) => Ok(v),
            Err(e) => Err(ApiFetchError::Deserialization {
                path: e.path().to_string(),
                source: e.into_inner(),
            }),
        }
    }
}
