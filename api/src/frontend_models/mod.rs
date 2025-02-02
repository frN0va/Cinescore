//! This module contains data structures representing frontend models
//! used for API responses. It is organized into submodules based on
//! different categories of data.

/// Common types and utilities used across multiple frontend models.
pub mod common;

/// Data structures for representing movie credits, including cast and crew.
pub mod credits;

/// Models related to movies, such as movie lists and movie details.
pub mod movies;

/// Data structures for representing people, including actors and directors.
pub mod people;

/// Models related to social media links and external IDs.
pub mod socials;
