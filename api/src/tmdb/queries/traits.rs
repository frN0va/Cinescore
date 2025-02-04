#![allow(dead_code)]
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

use chrono::NaiveDate;
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
    /// A `Result` containing a generic details struct on success or a [`ApiFetchError`] on failure.
    ///
    /// # Errors
    ///
    /// This function will return a [`ApiFetchError`] if the request fails or if the response cannot be parsed.
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
    /// A `Result` containing a generic details struct on success or a [`ApiFetchError`] on failure.
    ///
    /// # Errors
    ///
    /// This function will return a [`ApiFetchError`] if the request fails or if the response cannot be parsed.
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
    fn language<S: Into<String> + Display>(mut self, language: S) -> Self
    where
        Self: Sized,
    {
        log::debug!("Inserting language `{}` into query parameters", language);

        self.params().insert("language", language.to_string());
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
    fn append_to_response<S: Into<String> + Display>(mut self, append: S) -> Self
    where
        Self: Sized,
    {
        log::debug!(
            "Inserting append_to_response `{}` into query parameters",
            append
        );

        self.params()
            .insert("append_to_response", append.to_string());
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
    fn region<S: Into<String> + Display>(mut self, region: S) -> Self
    where
        Self: Sized,
    {
        log::debug!("Inserting region `{}` into query parameters", region);

        self.params().insert("region", region.to_string());
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

/// A trait for adding an `include_adult` query parameter to an API request.
///
/// This is used for filtering adult content.
pub trait IncludeAdultQueryParam: HasParams {
    /// Sets the `include_adult` query parameter for the request.
    ///
    /// # Arguments
    ///
    /// * `include_adult` - A `bool` representing whether or not to include adult content.
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn include_adult(mut self, include_adult: bool) -> Self
    where
        Self: Sized,
    {
        log::debug!(
            "Inserting include_adult `{}` into query parameters",
            include_adult
        );

        self.params()
            .insert("include_adult", include_adult.to_string());
        self
    }
}

/// A trait for adding an `include_video` query parameter to an API request.
pub trait IncludeVideoQueryParam: HasParams {
    /// Sets the `include_video` query parameter for the request.
    ///
    /// # Arguments
    ///
    /// * `include_video` - A `bool` representing whether or not to include a video in the response.
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn include_video(mut self, include_video: bool) -> Self
    where
        Self: Sized,
    {
        log::debug!(
            "Inserting include_video `{}` into query parameters",
            include_video
        );

        self.params()
            .insert("include_video", include_video.to_string());
        self
    }
}

/// A trait for adding a `primary_release_year` query parameter to an API request.
///
/// This is used for filtering results to a specific primary release year
pub trait PrimaryReleaseYearQueryParam: HasParams {
    /// Sets the `primary_release_year` query parameter for the request.
    ///
    /// # Arguments
    ///
    /// * `year` - A `String` representing the primary release year to filter content by.
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn primary_release_year<S: Into<String> + Display>(mut self, year: S) -> Self
    where
        Self: Sized,
    {
        log::debug!(
            "Inserting primary_release_year `{}` into query parameters",
            year
        );

        self.params()
            .insert("primary_release_year", year.to_string());
        self
    }
}

/// A trait for adding a `year` query parameter to an API request.
///
/// This is used for filtering results to a specific release year.
pub trait YearQueryParam: HasParams {
    /// Sets the `year` query parameter for the request.
    ///
    /// # Arguments
    ///
    /// * `year` - A `u16` representing the release year to filter content by.
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn year(mut self, year: u16) -> Self
    where
        Self: Sized,
    {
        log::debug!("Inserting year `{}` into query parameters", year);

        self.params().insert("year", year.to_string());
        self
    }
}

/// A trait for adding a `query` query parameter to an API request.
///
/// This is used for filtering results by a query, such as in searching.
pub trait QueryQueryParam: HasParams {
    /// Sets the `query` query parameter for the request.
    ///
    /// # Arguments
    ///
    /// * `query` - A `String` representing the query to search.
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn query<S: Into<String> + Display>(mut self, query: S) -> Self
    where
        Self: Sized,
    {
        log::debug!("Inserting query `{}` into query parameters", query);

        self.params().insert("query", query.to_string());
        self
    }
}

/// A trait for adding a `primary_release_date` query parameter to an API request.
///
/// This is used for filtering results by release date bounds
pub trait PrimaryReleaseDateQueryParam: HasParams {
    /// Sets the `primary_release_date.gte` and/or `primary_release_date.lte` query parameters for the request.
    ///
    /// # Arguments
    ///
    /// * `gte` - An optional lower bound for the release year.
    /// * `lte` - An optional upper bound for the release year.
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn primary_release_date_range(mut self, gte: Option<NaiveDate>, lte: Option<NaiveDate>) -> Self
    where
        Self: Sized,
    {
        if let Some(date) = gte {
            log::debug!(
                "Inserting primary_release_date.gte `{}` into query parameters",
                date
            );
            self.params()
                .insert("primary_release_date.gte", date.to_string());
        }

        if let Some(year) = lte {
            log::debug!(
                "Inserting primary_release_date.lte `{}` into query parameters",
                year
            );
            self.params()
                .insert("primary_release_date.lte", year.to_string());
        }

        self
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    OriginalTitleAsc,
    OriginalTitleDesc,
    PopularityAsc,
    PopularityDesc,
    RevenueAsc,
    RevenueDesc,
    PrimaryReleaseDateAsc,
    PrimaryReleaseDateDesc,
    TitleAsc,
    TitleDesc,
    VoteAverageAsc,
    VoteAverageDesc,
    VoteCountAsc,
    VoteCountDesc,
}

impl Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            SortBy::OriginalTitleAsc => "original_title.asc",
            SortBy::OriginalTitleDesc => "original_title.desc",
            SortBy::PopularityAsc => "popularity.asc",
            SortBy::PopularityDesc => "popularity.desc",
            SortBy::RevenueAsc => "revenue.asc",
            SortBy::RevenueDesc => "revenue.desc",
            SortBy::PrimaryReleaseDateAsc => "primary_release_date.asc",
            SortBy::PrimaryReleaseDateDesc => "primary_release_date.desc",
            SortBy::TitleAsc => "title.asc",
            SortBy::TitleDesc => "title.desc",
            SortBy::VoteAverageAsc => "vote_average.asc",
            SortBy::VoteAverageDesc => "vote_average.desc",
            SortBy::VoteCountAsc => "vote_count.asc",
            SortBy::VoteCountDesc => "vote_count.desc",
        };
        write!(f, "{}", value)
    }
}

/// A trait for adding a `sort_by` query parameter to an API request.
///
/// This is used for sorting the API data
pub trait SortByQueryParam: HasParams {
    /// Sets the `sort_by` query parameter for the request.
    ///
    /// # Arguments
    ///
    /// * `sort_by` - What to sort by
    ///
    /// # Returns
    ///
    /// A new instance of the struct implementing this trait with the updated query parameters.
    fn sort_by(mut self, sort_by: SortBy) -> Self
    where
        Self: Sized,
    {
        log::debug!("Inserting sort_by `{}` into query parameters", sort_by);
        self.params()
            .insert("primary_release_date.gte", sort_by.to_string());

        self
    }
}
