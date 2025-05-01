//! Indices namespace for OpenSearch

use crate::error::Error;
use derive_builder::Builder;
use reqwest::Method;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

/// Client namespace for index-related operations
#[derive(Debug, Clone)]
pub struct IndicesNamespace {
    client: crate::client::Client,
}

impl IndicesNamespace {
    /// Create a new indices namespace with the given client
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self { client }
    }

    /// Check if an index exists
    pub async fn exists(&self, index: impl Into<String>) -> Result<bool, Error> {
        let index_name = index.into();
        let path = format!("/{}", index_name);

        let response = self
            .client
            .http_client
            .head(
                self.client
                    .base_url
                    .join(&path)
                    .map_err(Error::UrlParseError)?,
            )
            .send()
            .await
            .map_err(Error::HttpRequestError)?;

        Ok(response.status().is_success())
    }
}

/// Index settings builder
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(pattern = "mutable")]
#[serde(rename_all = "snake_case")]
pub struct IndexSettings {
    /// Number of shards
    #[builder(default = "1")]
    pub number_of_shards: u32,

    /// Number of replicas
    #[builder(default = "1")]
    pub number_of_replicas: u32,

    /// Refresh interval
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_interval: Option<String>,

    /// Custom analysis settings
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<Value>,
}

impl IndexSettings {
    /// Create a new index settings builder
    pub fn builder() -> IndexSettingsBuilder {
        IndexSettingsBuilder::default()
    }
}

/// Create index request builder
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(pattern = "mutable")]
pub struct CreateIndexRequest {
    /// Index settings
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<IndexSettings>,

    /// Index mappings
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Value>,

    /// Index aliases
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<HashMap<String, Value>>,
}

impl CreateIndexRequest {
    /// Create a new index request builder
    pub fn builder() -> CreateIndexRequestBuilder {
        CreateIndexRequestBuilder::default()
    }
}

impl crate::client::Client {
    /// Access the indices namespace
    pub fn indices(&self) -> IndicesNamespace {
        IndicesNamespace::new(self.clone())
    }
}

impl IndicesNamespace {
    /// Create an index with the given settings
    pub async fn create(
        &self,
        index: impl Into<String>,
        request: CreateIndexRequest,
    ) -> Result<Value, Error> {
        let index_name = index.into();
        let path = format!("/{}", index_name);

        self.client
            .request::<CreateIndexRequest, Value>(Method::PUT, &path, Some(&request))
            .await
    }

    /// Delete an index
    pub async fn delete(&self, index: impl Into<String>) -> Result<Value, Error> {
        let index_name = index.into();
        let path = format!("/{}", index_name);

        self.client
            .request::<(), Value>(Method::DELETE, &path, None)
            .await
    }
}
