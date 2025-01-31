use std::collections::HashMap;

use reqwest::{
    header::{ACCEPT, AUTHORIZATION},
    Client,
};
use serde::de::DeserializeOwned;

pub const IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p/original/";
pub const API_BASE_URL: &str = "https://api.themoviedb.org/3";

#[derive(Debug, Clone)]
pub struct TMDBClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl TMDBClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: API_BASE_URL.to_string(),
        }
    }

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
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(response)
    }
}
