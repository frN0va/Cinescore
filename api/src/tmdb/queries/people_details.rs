use std::collections::HashMap;

use crate::{
    frontend_models::{FrontendPeopleList, FrontendPersonDetails},
    generate_request_struct,
    tmdb::{
        client::TMDBClient,
        models::{PaginatedSearchResult, PersonDetails, SearchPerson},
    },
};

use super::common::DetailsQuery;

generate_request_struct!(
    PersonDetailsRequest,
    "Request struct for fetching person details."
);

generate_request_struct!(
    TrendingPeopleRequest,
    "Request struct for fetching trending people."
);

/// A trait for querying a list of people from the TMDB API.
#[allow(dead_code)]
pub trait TrendingPeopleQuery {
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
    /// A `Result` containing a `FrontendPeopleList` on success or a `reqwest::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function will return a `reqwest::Error` if the request fails or if the response cannot be parsed.
    async fn fetch(self, client: &TMDBClient) -> Result<FrontendPeopleList, reqwest::Error>;

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
        self.params().insert("page", page.to_string());
        self
    }
}

impl DetailsQuery<FrontendPersonDetails> for PersonDetailsRequest {
    fn params(&mut self) -> &mut std::collections::HashMap<&'static str, String> {
        &mut self.params
    }

    async fn fetch(
        self,
        client: &TMDBClient,
        id: u64,
    ) -> Result<FrontendPersonDetails, reqwest::Error> {
        let response = client
            .get::<PersonDetails>(&format!("person/{}", id), self.params)
            .await?;

        Ok(FrontendPersonDetails::from(response))
    }
}

impl TrendingPeopleQuery for TrendingPeopleRequest {
    fn params(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.params
    }

    async fn fetch(self, client: &TMDBClient) -> Result<FrontendPeopleList, reqwest::Error> {
        let response = client
            .get::<PaginatedSearchResult<SearchPerson>>("/trending/movie/day", self.params)
            .await?;

        Ok(FrontendPeopleList::from(response))
    }
}
