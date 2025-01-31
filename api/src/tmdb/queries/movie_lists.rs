use std::collections::HashMap;

use crate::{
    frontend_models::FrontendMovieList,
    generate_request_struct,
    tmdb::{client::TMDBClient, models::MovieListSearch},
};

#[allow(dead_code)]
pub trait MovieListQuery {
    fn params(&mut self) -> &mut HashMap<&'static str, String>;
    async fn fetch(self, client: &TMDBClient) -> Result<FrontendMovieList, reqwest::Error>;

    fn language(mut self, language: String) -> Self
    where
        Self: Sized,
    {
        self.params().insert("language", language);
        self
    }

    fn page(mut self, page: u32) -> Self
    where
        Self: Sized,
    {
        self.params().insert("page", page.to_string());
        self
    }

    fn region(mut self, region: String) -> Self
    where
        Self: Sized,
    {
        self.params().insert("region", region);
        self
    }
}

generate_request_struct!(MovieListTrendingRequest);

impl MovieListQuery for MovieListTrendingRequest {
    fn params(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.params
    }

    async fn fetch(self, client: &TMDBClient) -> Result<FrontendMovieList, reqwest::Error> {
        let response = client
            .get::<MovieListSearch>("/trending/movie/day", self.params)
            .await?;

        Ok(FrontendMovieList::from(response))
    }
}

generate_request_struct!(MovieListNowPlaying);

impl MovieListQuery for MovieListNowPlaying {
    fn params(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.params
    }

    async fn fetch(self, client: &TMDBClient) -> Result<FrontendMovieList, reqwest::Error> {
        let response = client
            .get::<MovieListSearch>("movie/now_playing", self.params)
            .await?;

        Ok(FrontendMovieList::from(response))
    }
}
