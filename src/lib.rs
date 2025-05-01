//! # OpenSearch API for Rust
//!
//! This crate provides Rust structs and enums for the OpenSearch API specification,
//! along with a client to interact with OpenSearch clusters using a builder pattern.
//!
//! ## Examples
//!
//! ```rust
//! use opensearch_api_rs::{client::{Client, ClientConfig}, prelude::*};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     // Create client with builder pattern
//!     let client = Client::builder()
//!         .base_url("https://localhost:9200")
//!         .username("admin")
//!         .password("admin")
//!         .timeout_secs(30)
//!         .verify_ssl(true)
//!         .build()?
//!         .into_client()?;
//!
//!     // Search using builder pattern
//!     let search_request = client.search_typed::<serde_json::Value>("my-index")
//!         .query(serde_json::json!({
//!             "match": {
//!                 "field": "value"
//!             }
//!         }))
//!         .from(0)
//!         .size(10)
//!         .build()?;
//!
//!     let response = client.search(search_request).await?;
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
