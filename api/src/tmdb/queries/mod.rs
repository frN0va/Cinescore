//! This module handles API queries to TMDB with automatic serialization and deserialization.
//! It provides request structures for different types of queries and a macro for generating them.

/// Common utilities and types used in multiple API queries.
pub mod common;

/// Request models for fetching detailed movie information from TMDB.
pub mod movie_details;

/// Request models for fetching lists of movies, such as trending or now playing.
pub mod movie_lists;

/// Request models for fetching detailed information about people from TMDB.
pub mod people_details;

/// Macro for generating API request structures with predefined parameters.
#[macro_export]
macro_rules! generate_request_struct {
    ($request_name:ident, $docs:expr) => {
        #[doc=$docs]
        #[derive(Default)]
        pub struct $request_name {
            params: std::collections::HashMap<&'static str, String>,
        }

        impl $request_name {
            /// Creates a new instance of the request structure with empty parameters.
            pub fn new() -> Self {
                Self {
                    params: std::collections::HashMap::new(),
                }
            }
        }
    };
}
