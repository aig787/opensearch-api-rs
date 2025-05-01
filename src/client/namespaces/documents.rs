//! Documents namespace for OpenSearch

use crate::error::Error;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// Document operations have been moved to the Client implementation
// This file is kept for backward compatibility

/// Re-export document types for easier access
pub use crate::types::document::{
    BulkResponse, DeleteResponse, GetResponse, IndexResponse, UpdateResponse,
};

/// Client namespace for document-related operations
#[derive(Debug, Clone)]
pub struct DocumentsNamespace {
    client: crate::client::Client,
}

impl DocumentsNamespace {
    /// Create a new documents namespace with the given client
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self { client }
    }

    /// Index a document with the given ID
    pub async fn index<T>(
        &self,
        index: impl Into<String>,
        id: Option<impl Into<String>>,
        document: &T,
    ) -> Result<Value, Error>
    where
        T: Serialize + ?Sized,
    {
        let index_name = index.into();

        let (method, path) = if let Some(id) = id {
            // Index with specific ID
            let doc_id = id.into();
            (Method::PUT, format!("/{}/doc/{}", index_name, doc_id))
        } else {
            // Auto-generate ID
            (Method::POST, format!("/{}/doc", index_name))
        };

        self.client
            .request::<T, Value>(method, &path, Some(document))
            .await
    }

    /// Get a document by ID
    pub async fn get<T>(
        &self,
        index: impl Into<String>,
        id: impl Into<String>,
    ) -> Result<Option<T>, Error>
    where
        T: for<'de> Deserialize<'de> + Send + Sync,
    {
        let index_str = index.into();
        let id_str = id.into();
        let path = format!("{}/doc/{}", index_str, id_str);

        let url = self
            .client
            .base_url
            .join(&path)
            .map_err(Error::UrlParseError)?;

        let response = self
            .client
            .http_client
            .get(url)
            .send()
            .await
            .map_err(Error::HttpRequestError)?;

        // If document not found, return None
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        // Check for other errors
        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(Error::ApiError {
                status_code: status.as_u16(),
                message: error_body,
            });
        }

        // Parse response
        let get_response = response
            .json::<crate::types::document::GetResponse<T>>()
            .await
            .map_err(|e| Error::DeserializationError(e))?;

        Ok(get_response.source)
    }

    /// Update a document by ID
    pub async fn update<T>(
        &self,
        index: impl Into<String>,
        id: impl Into<String>,
        document: &T,
    ) -> Result<Value, Error>
    where
        T: Serialize + ?Sized,
    {
        let index_name = index.into();
        let doc_id = id.into();
        let path = format!("/{}/doc/{}", index_name, doc_id);

        // Wrap document in update syntax
        let update_doc = json!({
            "doc": document,
            "doc_as_upsert": true
        });

        self.client
            .request::<Value, Value>(
                Method::POST,
                &format!("{}/_update", path),
                Some(&update_doc),
            )
            .await
    }

    /// Delete a document by ID
    pub async fn delete(
        &self,
        index: impl Into<String>,
        id: impl Into<String>,
    ) -> Result<Value, Error> {
        let index_name = index.into();
        let doc_id = id.into();
        let path = format!("/{}/doc/{}", index_name, doc_id);

        self.client
            .request::<(), Value>(Method::DELETE, &path, None)
            .await
    }

    /// Refresh one or more indices
    pub async fn refresh(&self, index: impl Into<String>) -> Result<Value, Error> {
        let index_name = index.into();
        let path = format!("/{}/_refresh", index_name);

        self.client
            .request::<(), Value>(Method::POST, &path, None)
            .await
    }
}

impl crate::client::Client {
    /// Access the documents namespace
    pub fn documents(&self) -> DocumentsNamespace {
        DocumentsNamespace::new(self.clone())
    }
}
