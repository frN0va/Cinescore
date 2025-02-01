use crate::{
    frontend_models::FrontendPersonDetails, generate_request_struct, tmdb::models::PersonDetails,
};

use super::common::DetailsQuery;

generate_request_struct!(
    PersonDetailsRequest,
    "Request struct for fetching person details."
);

impl DetailsQuery<FrontendPersonDetails> for PersonDetailsRequest {
    fn params(&mut self) -> &mut std::collections::HashMap<&'static str, String> {
        &mut self.params
    }

    async fn fetch(
        self,
        client: &crate::tmdb::client::TMDBClient,
        id: u64,
    ) -> Result<FrontendPersonDetails, reqwest::Error> {
        let response = client
            .get::<PersonDetails>(&format!("person/{}", id), self.params)
            .await?;

        Ok(FrontendPersonDetails::from(response))
    }
}
