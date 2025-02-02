#![allow(dead_code)]
use std::collections::HashMap;

use serde::Serialize;

use crate::tmdb::client::{ApiFetchError, TMDBClient};

/// Defines a route that supports query parameters.
///
/// This trait provides a method to access and modify query parameters,
/// allowing structs to customize their API requests dynamically.
pub trait HasParams {
    /// Returns a mutable reference to the query parameters.
    ///
    /// This allows modifying the parameters before making a request.
    ///
    /// # Returns
    ///
    /// A mutable reference to a `HashMap` containing the query parameters.
    fn params(&mut self) -> &mut HashMap<&'static str, String>;
}

/// A trait for making API queries that return a structured response.
///
/// Implementers of this trait can define an API request that fetches data
/// from TMDB and deserializes it into the specified type.
///
/// # Type Parameters
///
/// * `T` - The response type that the query should return. Must implement `Serialize`.
pub trait Query<T>: HasParams
where
    T: Serialize,
{
    /// Asynchronously fetches the movie details from the TMDB API.
    ///
    /// # Arguments
    ///
    /// * `client` - The `TMDBClient` instance used to make the API request.
    ///
    /// # Returns
    ///
    /// A `Result` containing a generic details struct on success or a `reqwest::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return a `reqwest::Error` if the request fails or if the response cannot be parsed.
    async fn fetch(self, client: &TMDBClient) -> Result<T, ApiFetchError>;
}

/// A trait for making API queries that require an ID parameter.
///
/// This is used for requests that fetch specific details based on a given ID.
///
/// # Type Parameters
///
/// * `T` - The response type that the query should return. Must implement `Serialize`.
pub trait IdQuery<T>: HasParams
where
    T: Serialize,
{
    /// Asynchronously fetches the movie details from the TMDB API.
    ///
    /// # Arguments
    ///
    /// * `client` - The `TMDBClient` instance used to make the API request.
    /// * `id` - The ID of the movie to fetch details for.
    ///
    /// # Returns
    ///
    /// A `Result` containing a generic details struct on success or a `reqwest::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return a `reqwest::Error` if the request fails or if the response cannot be parsed.
    async fn fetch(self, client: &TMDBClient, id: u64) -> Result<T, ApiFetchError>;
}

/// A trait for adding a `language` query parameter to an API request.
///
/// This is used to specify a language filter for localized responses.
pub trait LanguageQueryParam: HasParams {
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
}

/// A trait for adding an `append_to_response` query parameter to an API request.
///
/// This allows requesting additional fields to be included in the API response.
pub trait AppendToResponseQueryParam: HasParams {
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
        log::debug!(
            "Inserting append_to_response `{}` into query parameters",
            append
        );

        self.params()
            .insert("append_to_response", append.to_owned());
        self
    }
}

/// A trait for adding a `region` query parameter to an API request.
///
/// This is useful for region-based filtering of results.
pub trait RegionQueryParam: HasParams {
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

/// A trait for adding a `page` query parameter to an API request.
///
/// This is used for paginated results.
pub trait PageQueryParam: HasParams {
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
}
