#![allow(dead_code)]
use serde::Deserialize;

/// Represents a paginated list retrieved from a search query.
#[derive(Debug, Deserialize)]
pub struct PaginatedSearchResult<T> {
    /// Current page number of results.
    pub page: u64,
    /// List of movies matching the search criteria.
    pub results: Vec<T>,
    /// Total number of pages available.
    pub total_pages: u64,
    /// Total number of results found.
    pub total_results: u64,
}
