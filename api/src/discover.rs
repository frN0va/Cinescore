use axum::Json;
use reqwest::header::{ACCEPT, AUTHORIZATION};

use crate::{frontend_models::Trending, tmdb_models::PopularMovies};

pub async fn fetch_trending() -> Json<Trending> {
    let client = reqwest::Client::new();

    let resp = match client
        .get("https://api.themoviedb.org/3/movie/popular?language=en-US&page=1")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", std::env::var("TMDB_API_KEY").unwrap()),
        )
        .header(ACCEPT, "application/json")
        .send()
        .await
    {
        Ok(v) => v,
        Err(_) => panic!("TODO"),
    };

    match resp.json::<PopularMovies>().await {
        Ok(v) => Json(Trending::from(v)),
        Err(e) => panic!("{}", e),
    }
}
