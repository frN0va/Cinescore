use std::collections::HashMap;

use crate::{
    frontend_models::FrontendMovieDetails,
    generate_request_struct,
    tmdb::{client::TMDBClient, models::MovieDetails},
};

#[allow(dead_code)]
pub trait MovieDetailsQuery {
    fn params(&mut self) -> &mut HashMap<&'static str, String>;
    async fn fetch(
        self,
        client: &TMDBClient,
        id: u64,
    ) -> Result<FrontendMovieDetails, reqwest::Error>;

    fn language(mut self, language: String) -> Self
    where
        Self: Sized,
    {
        self.params().insert("language", language);
        self
    }

    fn append_to_response(mut self, append: &str) -> Self
    where
        Self: Sized,
    {
        self.params()
            .insert("append_to_response", append.to_string());
        self
    }
}

generate_request_struct!(MovieDetailsRequest);

impl MovieDetailsQuery for MovieDetailsRequest {
    fn params(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.params
    }

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
