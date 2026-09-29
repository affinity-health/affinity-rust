//! # Affinity API SDK
//!
//! The official Rust SDK for the Affinity API.
//!
//! ## Getting Started
//!
//! ```rust
//! use affinity_health_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         api_key: Some("<value>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = ApiClient::new(config).expect("Failed to build client");
//!     client
//!         .locations
//!         .list_practice_locations(
//!             &"prac_01j2y8m6jcc9tt24af5pw9x1bc".to_string(),
//!             &ListPracticeLocationsQueryRequest {
//!                 starting_after: Some("loc_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
//!                 ending_before: Some("loc_01j2y8m6jcc9tt24af5pw9x1bc".to_string()),
//!                 ..Default::default()
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
