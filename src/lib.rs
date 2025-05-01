//! # OpenSearch API for Rust
//!
//! This crate provides Rust structs and enums for the OpenSearch API specification,
//! along with a client to interact with OpenSearch clusters using a builder pattern.
//!
//! ## Examples
//!
//! ```rust,no_run
//! use opensearch_api::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     // Create client with builder pattern
//!     use opensearch_api::{Client, Error};
//! use opensearch_api::builder::MatchAllQuery;
//! let client = Client::builder()
//!         .base_url("https://localhost:9200")
//!         .username("admin")
//!         .password("admin")
//!         .timeout_secs(30)
//!         .verify_ssl(true)
//!         .build()?;
//!
//!     // Search using builder pattern
//!     let response = client.search::<serde_json::Value>()
//!         .from(0)
//!         .size(10)
//!         .query(MatchAllQuery::builder().build_query().unwrap())
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
mod client;
mod error;
mod types;

#[cfg(feature = "client")]
pub use client::namespaces::indices;
#[cfg(feature = "client")]
pub use client::namespaces::cluster;
#[cfg(feature = "client")]
pub use client::namespaces::documents;
#[cfg(feature = "client")]
pub use client::Client;
#[cfg(feature = "client")]
pub use client::ClientConfig;
pub use error::{Error, Result};
pub use types::*;

pub mod prelude {
    //! Common types and traits for working with the OpenSearch API.
    #[cfg(feature = "client")]
    pub use crate::client::namespaces::indices;
    #[cfg(feature = "client")]
    pub use crate::client::namespaces::cluster;
    #[cfg(feature = "client")]
    pub use crate::client::namespaces::documents;
    #[cfg(feature = "client")]
    pub use crate::client::{Client, ClientConfig};
    pub use crate::error::Error;
    pub use crate::types::*;
}
