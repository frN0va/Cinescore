//! This module provides an interface for interacting with the TMDB API.
//! It includes the API client, data models, and query structures for making requests.

/// The TMDB API client for handling requests and authentication.
pub mod client;

/// Data models representing TMDB API responses
pub mod models;

/// Query structures for sending API requests with automatic serialization and deserialization.
pub mod queries;
