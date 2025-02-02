//! This module contains structures and utilities for interacting with the TMDB API.
//! It is divided into submodules based on different types of API requests and responses.

/// Common types and utilities shared across multiple TMDB API models.
pub mod common;

/// Data structures and request models for fetching movie-related data from TMDB.
pub mod movie;

/// Data structures and request models for fetching person-related data from TMDB.
pub mod person;

/// Models related to social media links and external IDs for movies and people.
pub mod socials;
