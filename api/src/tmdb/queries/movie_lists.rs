use std::collections::HashMap;

use crate::{
    frontend_models::FrontendMovieList,
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

#[derive(Default)]
pub struct MovieListPopular {
    params: HashMap<&'static str, String>,
}

impl MovieListPopular {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }
}

impl MovieListQuery for MovieListPopular {
    fn params(&mut self) -> &mut HashMap<&'static str, String> {
        &mut self.params
    }

    async fn fetch(self, client: &TMDBClient) -> Result<FrontendMovieList, reqwest::Error> {
        let response = client
            .get::<MovieListSearch>("movie/popular", self.params)
            .await?;

        Ok(FrontendMovieList::from(response))
    }
}

#[derive(Default)]
pub struct MovieListNowPlaying {
    params: HashMap<&'static str, String>,
}

impl MovieListNowPlaying {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }
}

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
