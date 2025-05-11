//! # OpenSearch API for Rust
//!
//! This crate provides Rust structs and enums for the OpenSearch API specification,
//! along with a client to interact with OpenSearch clusters using a builder pattern.
//!
//! ## Examples
//!
//! ```rust,no_run
//! use opensearch_api::Error;
//! use opensearch_api::Client;
//! use opensearch_api::types::query::MatchAllQuery;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     let client = Client::builder()
//!         .base_url("https://localhost:9200")
//!         .username("admin")
//!         .password("admin")
//!         .timeout_secs(30)
//!         .verify_ssl(true)
//!         .build()?;
//!
//!     // Search using builder pattern
//!     let response = client.search::<serde_json::Value>("index")
//!         .from(0)
//!         .size(10)
//!         .query(MatchAllQuery::builder().build().unwrap())
//!         .build()
//!         .unwrap()
//!         .send()
//!         .await?;
//!
//!     println!("Found {} hits", response.hits.total.value);
//!
//!     Ok(())
//! }
//! ```
#[cfg(feature = "client")]
pub(crate) mod client;
pub(crate) mod document;
pub(crate) mod error;
pub mod types;
#[macro_use]
pub(crate) mod macros;

#[cfg(feature = "client")]
pub use client::namespaces::*;
#[cfg(feature = "client")]
pub use client::*;
pub use error::{Error, Result};
