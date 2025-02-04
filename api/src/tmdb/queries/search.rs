use crate::{
    frontend_models::{movies::FrontendMovieList, people::FrontendPeopleList},
    generate_request_struct,
    tmdb::{
        client::ApiFetchError,
        models::{common::PaginatedSearchResult, movie::SearchMovie, person::SearchPerson},
    },
};

use super::traits::{
    IncludeAdultQueryParam, LanguageQueryParam, PageQueryParam, PrimaryReleaseYearQueryParam,
    Query, QueryQueryParam, RegionQueryParam, YearQueryParam,
};

generate_request_struct!(
    SearchPeopleRequest,
    "Request struct for searching for people in the TMDB API"
);

generate_request_struct!(
    SearchMoviesRequest,
    "Request struct for searching for movies in the TMDB API"
);

impl Query<FrontendPeopleList> for SearchPeopleRequest {
    async fn fetch(
        self,
        client: &crate::tmdb::client::TMDBClient,
    ) -> Result<FrontendPeopleList, crate::tmdb::client::ApiFetchError> {
        // require query param to be set
        match self.params.get("query") {
            Some(v) => log::debug!("Fetching people search results for \"{}\"", v),
            None => {
                return Err(ApiFetchError::MissingQueryParam {
                    param: "query",
                    request_name: "SearchPersonRequest",
                });
            }
        }

        let response = client
            .get::<PaginatedSearchResult<SearchPerson>>("search/person", self.params)
            .await?;

        Ok(FrontendPeopleList::from(response))
    }
}

impl Query<FrontendMovieList> for SearchMoviesRequest {
    async fn fetch(
        self,
        client: &crate::tmdb::client::TMDBClient,
    ) -> Result<FrontendMovieList, crate::tmdb::client::ApiFetchError> {
        // require query param to be set
        match self.params.get("query") {
            Some(v) => log::debug!("Fetching movie search results for \"{}\"", v),
            None => {
                return Err(ApiFetchError::MissingQueryParam {
                    param: "query",
                    request_name: "SearchMoviesRequest",
                });
            }
        }

        let response = client
            .get::<PaginatedSearchResult<SearchMovie>>("search/movie", self.params)
            .await?;

        Ok(FrontendMovieList::from(response))
    }
}

impl QueryQueryParam for SearchPeopleRequest {}
impl IncludeAdultQueryParam for SearchPeopleRequest {}
impl LanguageQueryParam for SearchPeopleRequest {}
impl PageQueryParam for SearchPeopleRequest {}

impl QueryQueryParam for SearchMoviesRequest {}
impl IncludeAdultQueryParam for SearchMoviesRequest {}
impl LanguageQueryParam for SearchMoviesRequest {}
impl PageQueryParam for SearchMoviesRequest {}
impl PrimaryReleaseYearQueryParam for SearchMoviesRequest {}
impl RegionQueryParam for SearchMoviesRequest {}
impl YearQueryParam for SearchMoviesRequest {}
