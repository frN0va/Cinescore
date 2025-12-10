use crate::{
    frontend_models::people::{FrontendPeopleList, FrontendPersonDetails},
    generate_request_struct,
    tmdb::{
        client::{ApiFetchError, TMDBClient},
        models::{
            common::PaginatedSearchResult,
            person::{PersonDetails, SearchPerson},
        },
    },
};

use super::traits::{AppendToResponseQueryParam, IdQuery, LanguageQueryParam, Query};

generate_request_struct!(
    PersonDetailsRequest,
    "Request struct for fetching person details."
);

generate_request_struct!(
    TrendingPeopleRequest,
    "Request struct for fetching trending people."
);

impl IdQuery<FrontendPersonDetails> for PersonDetailsRequest {
    async fn fetch(
        self,
        client: &TMDBClient,
        id: u64,
    ) -> Result<FrontendPersonDetails, ApiFetchError> {
        tracing::debug!("Fetching person details for person ID {}", id);

        let response = client
            .get::<PersonDetails>(&format!("person/{}", id), self.params)
            .await?;

        Ok(FrontendPersonDetails::from(response))
    }
}

impl Query<FrontendPeopleList> for TrendingPeopleRequest {
    async fn fetch(self, client: &TMDBClient) -> Result<FrontendPeopleList, ApiFetchError> {
        tracing::debug!("Fetching daily trending people from TMDB API");

        let response = client
            .get::<PaginatedSearchResult<SearchPerson>>("/trending/person/day", self.params)
            .await?;

        Ok(FrontendPeopleList::from(response))
    }
}

impl AppendToResponseQueryParam for PersonDetailsRequest {}
impl LanguageQueryParam for PersonDetailsRequest {}

impl LanguageQueryParam for TrendingPeopleRequest {}
