//! Documents namespace for OpenSearch

use crate::error::Error;
use derive_builder::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::types::document::{BulkOptions, DeleteOptions, ExistsOptions, GetOptions, IndexOptions, MgetOptions, UpdateOptions, WaitForActiveShards};
/// Re-export document types for easier access
pub use crate::types::document::{DeleteResponse, GetResponse, IndexResponse, UpdateResponse};

/// Client namespace for document-related operations
#[derive(Debug, Clone)]
pub struct DocumentsNamespace {
    client: crate::client::Client,
}

/// Builder for index document requests
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct IndexRequest<'a, T: Serialize + ?Sized + Clone = serde_json::Value> {
    /// Documents namespace reference
    #[builder(pattern = "immutable")]
    client: &'a DocumentsNamespace,

    /// Index to store the document in
    #[builder(pattern = "immutable")]
    index: String,

    /// Document to index
    document: &'a T,

    /// Document ID (optional, will be auto-generated if not provided)
    #[builder(default)]
    id: Option<String>,

    /// Index options
    #[builder(default)]
    options: Option<IndexOptions>,
}

impl<'a, T: Clone + Serialize + ?Sized> IndexRequestBuilder<'a, T> {
    /// Build and send the index request to the server
    pub async fn send(self) -> Result<IndexResponse, Error> {
        self.build().unwrap().send().await
    }

    /// Set the refresh option
    pub fn refresh(mut self, refresh: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.refresh = Some(refresh.into());
        self
    }

    /// Set the routing option
    pub fn routing(mut self, routing: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.routing = Some(routing.into());
        self
    }

    /// Set the timeout option
    pub fn timeout(mut self, timeout: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.timeout = Some(timeout.into());
        self
    }

    /// Set the version option
    pub fn version(mut self, version: i64) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.version = Some(version);
        self
    }

    /// Set the version_type option
    pub fn version_type(mut self, version_type: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.version_type = Some(version_type.into());
        self
    }

    /// Set the wait_for_active_shards option
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: WaitForActiveShards) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
}

impl<'a, T: Serialize + ?Sized + Clone> IndexRequest<'a, T> {
    /// Create a new index request builder
    pub(crate) fn new(
        client: &'a DocumentsNamespace,
        index: impl Into<String>,
    ) -> IndexRequestBuilder<'a, T> {
        IndexRequestBuilder::default()
            .client(client)
            .index(index.into())
    }

    /// Send the index request to the server
    pub async fn send(self) -> Result<IndexResponse, Error> {
        let index_str = self.index;
        let mut method = Method::POST;
        let mut path = format!("/{}", index_str);

        if let Some(id) = self.id {
            path.push_str(&format!("/_doc/{}", id));
            // Use PUT when ID is provided
            method = Method::PUT;
        } else {
            path.push_str("/_doc");
        }

        // Add query parameters from options
        if let Some(options) = &self.options {
            let mut query_params = Vec::new();

            if let Some(refresh) = &options.refresh {
                query_params.push(format!("refresh={}", refresh));
            }

            if let Some(routing) = &options.routing {
                query_params.push(format!("routing={}", routing));
            }

            if let Some(timeout) = &options.timeout {
                query_params.push(format!("timeout={}", timeout));
            }

            if let Some(version) = options.version {
                query_params.push(format!("version={}", version));
            }

            if let Some(version_type) = &options.version_type {
                query_params.push(format!("version_type={}", version_type));
            }

            if let Some(wait_for_active_shards) = &options.wait_for_active_shards {
                let value = match wait_for_active_shards {
                    WaitForActiveShards::Value(v) => v.to_string(),
                    WaitForActiveShards::Count(n) => n.to_string(),
                };
                query_params.push(format!("wait_for_active_shards={}", value));
            }

            if !query_params.is_empty() {
                path.push_str(&format!("?{}", query_params.join("&")));
            }
        }

        self.client
            .client
            .request::<_, IndexResponse>(method, &path, Some(self.document))
            .await
    }
}

/// Builder for get document requests
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct GetRequest<'a, T: Clone + for<'de> Deserialize<'de> + Send + Sync> {
    /// Documents namespace reference
    #[builder(pattern = "immutable")]
    client: &'a DocumentsNamespace,
    /// Index to get the document from
    #[builder(pattern = "immutable")]
    index: String,
    /// Document ID
    #[builder(pattern = "immutable")]
    id: String,
    /// Get options
    #[builder(default)]
    options: Option<GetOptions>,
    /// Type parameter marker
    #[builder(setter(skip), default = "std::marker::PhantomData")]
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: Clone + for<'de> Deserialize<'de> + Send + Sync> GetRequestBuilder<'a, T> {
    /// Set the source option
    pub fn source(mut self, source: bool) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.source = Some(source);
        self
    }

    /// Set the source_includes option
    pub fn source_includes(mut self, source_includes: Vec<impl Into<String>>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.source_includes = Some(source_includes.into_iter().map(Into::into).collect());
        self
    }

    /// Set the source_excludes option
    pub fn source_excludes(mut self, source_excludes: Vec<impl Into<String>>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.source_excludes = Some(source_excludes.into_iter().map(Into::into).collect());
        self
    }

    /// Set the routing option
    pub fn routing(mut self, routing: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.routing = Some(routing.into());
        self
    }

    /// Set the preference option
    pub fn preference(mut self, preference: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.preference = Some(preference.into());
        self
    }

    /// Set the realtime option
    pub fn realtime(mut self, realtime: bool) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.realtime = Some(realtime);
        self
    }

    /// Set the refresh option
    pub fn refresh(mut self, refresh: bool) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.refresh = Some(refresh);
        self
    }

    /// Set the version option
    pub fn version(mut self, version: i64) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.version = Some(version);
        self
    }

    /// Set the version_type option
    pub fn version_type(mut self, version_type: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.version_type = Some(version_type.into());
        self
    }

    /// Build and send the get request
    pub async fn send(self) -> Result<Option<GetResponse<T>>, Error> {
        self.build().unwrap().send().await
    }
}

impl<'a, T: Clone + for<'de> Deserialize<'de> + Send + Sync> GetRequest<'a, T> {
    /// Create a new get request builder
    pub(crate) fn new(
        client: &'a DocumentsNamespace,
        index: impl Into<String>,
        id: impl Into<String>,
    ) -> GetRequestBuilder<'a, T> {
        GetRequestBuilder::default()
            .client(client)
            .index(index)
            .id(id)
    }

    /// Build and send the get request
    pub async fn send(self) -> Result<Option<GetResponse<T>>, Error> {
        let index_str = self.index;
        let id_str = self.id;
        let mut path = format!("/{index_str}/_doc/{id_str}");

        // Add query parameters from options
        let mut query_params = Vec::new();
        if let Some(options) = &self.options {
            if let Some(source) = options.source {
                query_params.push(format!("_source={}", source));
            }

            if let Some(source_includes) = &options.source_includes {
                let includes = source_includes.join(",");
                query_params.push(format!("_source_includes={}", includes));
            }

            if let Some(source_excludes) = &options.source_excludes {
                let excludes = source_excludes.join(",");
                query_params.push(format!("_source_excludes={}", excludes));
            }

            if let Some(routing) = &options.routing {
                query_params.push(format!("routing={}", routing));
            }

            if let Some(preference) = &options.preference {
                query_params.push(format!("preference={}", preference));
            }

            if let Some(realtime) = options.realtime {
                query_params.push(format!("realtime={}", realtime));
            }

            if let Some(refresh) = options.refresh {
                query_params.push(format!("refresh={}", refresh));
            }

            if let Some(version) = options.version {
                query_params.push(format!("version={}", version));
            }

            if let Some(version_type) = &options.version_type {
                query_params.push(format!("version_type={}", version_type));
            }
        }

        // Add query parameters to path
        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        log::debug!("Sending GET request to path: {}", path);

        // Make a direct request to properly handle 404 responses
        let url = self
            .client
            .client
            .base_url
            .join(&path)
            .map_err(Error::UrlParseError)?;
        let result = self.client.client.http_client.get(url).send().await;

        match result {
            Ok(response) => {
                let status = response.status();
                log::debug!("GET request returned status: {}", status);

                // Return None for 404 responses
                if status == reqwest::StatusCode::NOT_FOUND {
                    log::debug!("Document not found (404), returning None");
                    return Ok(None);
                }

                // Handle other error responses
                if !status.is_success() {
                    let error_text = response.text().await.unwrap_or_default();
                    return Err(Error::ApiError {
                        status_code: status.as_u16(),
                        message: error_text,
                        request_body_info: String::new(),
                    });
                }

                // Parse successful response
                let response_text = response.text().await.map_err(Error::HttpRequestError)?;
                match serde_json::from_str::<GetResponse<T>>(&response_text) {
                    Ok(get_response) => Ok(Some(get_response)),
                    Err(err) => {
                        log::error!("Failed to parse GET response: {}", err);
                        Err(Error::DeserializationErrorWithResponse {
                            error: err,
                            response_text,
                            path: "".to_string(),
                            expected_type: std::any::type_name::<GetResponse<T>>().to_string(),
                        })
                    }
                }
            }
            Err(err) => {
                // Handle network errors and other request failures
                if let Some(status) = err.status() {
                    if status == reqwest::StatusCode::NOT_FOUND {
                        log::debug!("Document not found (404), returning None");
                        return Ok(None);
                    }
                }
                log::error!("GET request failed: {}", err);
                Err(Error::HttpRequestError(err))
            }
        }
    }
}

/// Builder for update document requests
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct UpdateRequest<'a, T: Clone + Serialize + ?Sized> {
    /// Documents namespace reference
    #[builder(pattern = "immutable")]
    client: &'a DocumentsNamespace,
    /// Index to update the document in
    #[builder(pattern = "immutable")]
    index: String,
    /// Document ID
    #[builder(pattern = "immutable")]
    id: String,
    /// Document to update with
    #[builder(pattern = "immutable")]
    document: &'a T,
    /// Update options
    #[builder(default)]
    options: Option<UpdateOptions>,
}

impl<'a, T: Clone + Serialize + ?Sized> UpdateRequestBuilder<'a, T> {
    /// Set the doc_as_upsert option
    pub fn doc_as_upsert(mut self, doc_as_upsert: bool) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.doc_as_upsert = Some(doc_as_upsert);
        self
    }

    /// Set the retry_on_conflict option
    pub fn retry_on_conflict(mut self, retry_on_conflict: i32) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.retry_on_conflict = Some(retry_on_conflict);
        self
    }

    /// Set the refresh option
    pub fn refresh(mut self, refresh: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.refresh = Some(refresh.into());
        self
    }

    /// Set the routing option
    pub fn routing(mut self, routing: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.routing = Some(routing.into());
        self
    }

    /// Set the timeout option
    pub fn timeout(mut self, timeout: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.timeout = Some(timeout.into());
        self
    }

    /// Set the wait_for_active_shards option
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: WaitForActiveShards) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }

    /// Set the require_alias option
    pub fn require_alias(mut self, require_alias: bool) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.require_alias = Some(require_alias);
        self
    }

    /// Build and send the update request
    pub async fn send(self) -> Result<UpdateResponse, Error> {
        self.build().unwrap().send().await
    }
}

impl<'a, T: Clone + Serialize + ?Sized> UpdateRequest<'a, T> {
    /// Create a new update request builder
    pub(crate) fn new(
        client: &'a DocumentsNamespace,
        index: impl Into<String>,
        id: impl Into<String>,
        document: &'a T,
    ) -> UpdateRequestBuilder<'a, T> {
        UpdateRequestBuilder::default()
            .client(client)
            .index(index)
            .id(id)
            .document(document)
    }

    /// Build and send the update request
    pub async fn send(self) -> Result<UpdateResponse, Error> {
        let index_str = self.index;
        let id_str = self.id;
        let mut path = format!("/{index_str}/_update/{id_str}");

        // Build update document with proper structure
        let mut update_doc = json!({
            "doc": self.document
        });

        // Add options to update document
        if let Some(options) = &self.options {
            if let Some(doc_as_upsert) = options.doc_as_upsert {
                update_doc["doc_as_upsert"] = json!(doc_as_upsert);
            }
        }

        // Add query parameters from options
        let mut query_params = Vec::new();
        if let Some(options) = &self.options {
            if let Some(retry_on_conflict) = options.retry_on_conflict {
                query_params.push(format!("retry_on_conflict={}", retry_on_conflict));
            }

            if let Some(refresh) = &options.refresh {
                query_params.push(format!("refresh={}", refresh));
            }

            if let Some(routing) = &options.routing {
                query_params.push(format!("routing={}", routing));
            }

            if let Some(timeout) = &options.timeout {
                query_params.push(format!("timeout={}", timeout));
            }

            if let Some(wait_for_active_shards) = &options.wait_for_active_shards {
                let value = match wait_for_active_shards {
                    WaitForActiveShards::Value(v) => v.to_string(),
                    WaitForActiveShards::Count(n) => n.to_string(),
                };
                query_params.push(format!("wait_for_active_shards={}", value));
            }

            if let Some(require_alias) = options.require_alias {
                query_params.push(format!("require_alias={}", require_alias));
            }
        }

        // Add query parameters to path
        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        log::debug!("Sending UPDATE request to path: {}", path);
        self.client
            .client
            .request::<_, UpdateResponse>(Method::POST, &path, Some(&update_doc))
            .await
    }
}

/// Builder for delete document requests
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct DeleteRequest<'a> {
    /// Documents namespace reference
    #[builder(pattern = "immutable")]
    client: &'a DocumentsNamespace,
    /// Index to delete the document from
    #[builder(pattern = "immutable")]
    index: String,
    /// Document ID
    #[builder(pattern = "immutable")]
    id: String,
    /// Delete options
    #[builder(default)]
    options: Option<DeleteOptions>,
}

impl<'a> DeleteRequestBuilder<'a> {
    /// Set the refresh option
    pub fn refresh(mut self, refresh: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.refresh = Some(refresh.into());
        self
    }

    /// Set the routing option
    pub fn routing(mut self, routing: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.routing = Some(routing.into());
        self
    }

    /// Set the timeout option
    pub fn timeout(mut self, timeout: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.timeout = Some(timeout.into());
        self
    }

    /// Set the version option
    pub fn version(mut self, version: i64) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.version = Some(version);
        self
    }

    /// Set the version_type option
    pub fn version_type(mut self, version_type: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.version_type = Some(version_type.into());
        self
    }

    /// Set the wait_for_active_shards option
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: WaitForActiveShards) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }

    /// Build and send the delete request
    pub async fn send(self) -> Result<DeleteResponse, Error> {
        self.build().unwrap().send().await
    }
}

impl<'a> DeleteRequest<'a> {
    /// Create a new delete request builder
    pub(crate) fn new(
        client: &'a DocumentsNamespace,
        index: impl Into<String>,
        id: impl Into<String>,
    ) -> DeleteRequestBuilder<'a> {
        DeleteRequestBuilder::default()
            .client(client)
            .index(index)
            .id(id)
    }

    /// Build and send the delete request to the server
    pub async fn send(self) -> Result<DeleteResponse, Error> {
        let index_str = self.index;
        let id_str = self.id;
        let mut path = format!("/{index_str}/_doc/{id_str}");

        // Add query parameters from options
        let mut query_params = Vec::new();
        if let Some(options) = &self.options {
            if let Some(refresh) = &options.refresh {
                query_params.push(format!("refresh={}", refresh));
            }

            if let Some(routing) = &options.routing {
                query_params.push(format!("routing={}", routing));
            }

            if let Some(timeout) = &options.timeout {
                query_params.push(format!("timeout={}", timeout));
            }

            if let Some(version) = options.version {
                query_params.push(format!("version={}", version));
            }

            if let Some(version_type) = &options.version_type {
                query_params.push(format!("version_type={}", version_type));
            }

            if let Some(wait_for_active_shards) = &options.wait_for_active_shards {
                let value = match wait_for_active_shards {
                    WaitForActiveShards::Value(v) => v.to_string(),
                    WaitForActiveShards::Count(n) => n.to_string(),
                };
                query_params.push(format!("wait_for_active_shards={}", value));
            }
        }

        // Add query parameters to path
        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        log::debug!("Sending DELETE request to path: {}", path);

        // Make a direct request to handle 404 responses specially
        let url = self
            .client
            .client
            .base_url
            .join(&path)
            .map_err(Error::UrlParseError)?;
        let result = self.client.client.http_client.delete(url).send().await;

        match result {
            Ok(response) => {
                let status = response.status();
                log::debug!("DELETE request returned status: {}", status);

                // For both success and 404 status, try to parse the response
                if status.is_success() || status == reqwest::StatusCode::NOT_FOUND {
                    let response_text = response.text().await.map_err(Error::HttpRequestError)?;

                    // Try to parse the response
                    match serde_json::from_str::<DeleteResponse>(&response_text) {
                        Ok(delete_response) => Ok(delete_response),
                        Err(err) => {
                            log::error!("Failed to parse DELETE response: {}", err);
                            Err(Error::DeserializationErrorWithResponse {
                                error: err,
                                response_text,
                                path: "".to_string(),
                                expected_type: std::any::type_name::<DeleteResponse>().to_string(),
                            })
                        }
                    }
                } else {
                    // Handle other error responses
                    let error_text = response.text().await.unwrap_or_default();
                    Err(Error::ApiError {
                        status_code: status.as_u16(),
                        message: error_text,
                        request_body_info: String::new(),
                    })
                }
            }
            Err(err) => {
                log::error!("DELETE request failed: {}", err);
                Err(Error::HttpRequestError(err))
            }
        }
    }
}

/// Builder for exists document requests
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct ExistsRequest<'a> {
    /// Documents namespace reference
    #[builder(pattern = "immutable")]
    client: &'a DocumentsNamespace,
    /// Index to check for the document in
    #[builder(pattern = "immutable")]
    index: String,
    /// Document ID
    #[builder(pattern = "immutable")]
    id: String,
    /// Exists options
    #[builder(default)]
    options: Option<ExistsOptions>,
}

impl<'a> ExistsRequestBuilder<'a> {
    /// Set the routing option
    pub fn routing(mut self, routing: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.routing = Some(routing.into());
        self
    }

    /// Set the preference option
    pub fn preference(mut self, preference: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.preference = Some(preference.into());
        self
    }

    /// Set the realtime option
    pub fn realtime(mut self, realtime: bool) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.realtime = Some(realtime);
        self
    }

    /// Set the refresh option
    pub fn refresh(mut self, refresh: bool) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.refresh = Some(refresh);
        self
    }

    /// Set the version option
    pub fn version(mut self, version: i64) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.version = Some(version);
        self
    }

    /// Set the version_type option
    pub fn version_type(mut self, version_type: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.version_type = Some(version_type.into());
        self
    }

    /// Build and send the exists request
    pub async fn send(self) -> Result<bool, Error> {
        self.build().unwrap().send().await
    }
}

impl<'a> ExistsRequest<'a> {
    /// Create a new exists request builder
    pub(crate) fn new(
        client: &'a DocumentsNamespace,
        index: impl Into<String>,
        id: impl Into<String>,
    ) -> ExistsRequestBuilder<'a> {
        ExistsRequestBuilder::default()
            .client(client)
            .index(index)
            .id(id)
    }

    /// Send the exists request to the server
    pub async fn send(self) -> Result<bool, Error> {
        let index_str = self.index;
        let id_str = self.id;
        let mut path = format!("/{index_str}/_doc/{id_str}");

        // Add query parameters from options
        let mut query_params = Vec::new();
        if let Some(options) = &self.options {
            if let Some(routing) = &options.routing {
                query_params.push(format!("routing={}", routing));
            }

            if let Some(preference) = &options.preference {
                query_params.push(format!("preference={}", preference));
            }

            if let Some(realtime) = options.realtime {
                query_params.push(format!("realtime={}", realtime));
            }

            if let Some(refresh) = options.refresh {
                query_params.push(format!("refresh={}", refresh));
            }

            if let Some(version) = options.version {
                query_params.push(format!("version={}", version));
            }

            if let Some(version_type) = &options.version_type {
                query_params.push(format!("version_type={}", version_type));
            }
        }

        // Add query parameters to path
        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        log::debug!("Checking document existence at path: {}", path);

        // Use the URL builder from the client
        let url = self
            .client
            .client
            .base_url
            .join(&path)
            .map_err(Error::UrlParseError)?;

        // Make a HEAD request to check existence
        let result = self.client.client.http_client.head(url).send().await;

        match result {
            Ok(response) => {
                let status = response.status();
                log::debug!("Exists request returned status: {}", status);
                Ok(status.is_success())
            }
            Err(err) => {
                // HTTP 404 indicates document doesn't exist, not an error
                if let Some(status) = err.status() {
                    if status == reqwest::StatusCode::NOT_FOUND {
                        log::debug!("Document not found (404), returning false");
                        return Ok(false);
                    }
                    log::warn!("Exists request failed with status: {}", status);
                } else {
                    log::error!("Exists request failed: {}", err);
                }
                Err(Error::HttpRequestError(err))
            }
        }
    }
}

/// Builder for refresh requests
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct RefreshRequest<'a> {
    /// Documents namespace reference
    #[builder(pattern = "immutable")]
    client: &'a DocumentsNamespace,
    /// Index to refresh
    #[builder(pattern = "immutable")]
    index: String,
}

impl<'a> RefreshRequest<'a> {
    /// Create a new refresh request builder
    pub(crate) fn new(
        client: &'a DocumentsNamespace,
        index: impl Into<String>,
    ) -> RefreshRequestBuilder<'a> {
        RefreshRequestBuilder::default().client(client).index(index)
    }

    /// Send the refresh request to the server
    pub async fn send(self) -> Result<serde_json::Value, Error> {
        let index_str = self.index;
        let path = format!("{}/_refresh", index_str);
        self.client
            .client
            .request::<(), serde_json::Value>(Method::POST, &path, None)
            .await
    }
}

impl DocumentsNamespace {
    /// Create a new documents namespace with the given client
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self { client }
    }

    /// # Fluent Builder API
    ///
    /// The DocumentsNamespace provides a fluent builder pattern API for document operations:
    ///
    /// The builder pattern enables a readable and chainable API for complex operations
    /// and is the recommended approach for all document operations.
    ///
    /// Example of the fluent builder pattern:
    ///
    /// ```no_run
    /// # use opensearch_api::{Client, Error};
    /// # use serde_json::json;
    /// # async fn example() -> Result<(), Error> {
    /// # let client = Client::builder().base_url("http://localhost:9200").build()?;
    /// // Using the fluent builder API:
    /// let response = client.documents()
    ///     .index("my_index")
    ///     .document(&json!({"field": "value"}))
    ///     .id("doc1")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```

    /// Create a builder for indexing a document
    ///
    /// This allows for a fluent API to set options and execute the index operation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::{Client, Error};
    /// # use serde_json::json;
    /// # async fn example() -> Result<(), Error> {
    /// # let client = Client::builder().base_url("http://localhost:9200").build()?;
    /// let response = client.documents()
    ///     .index("my_index")
    ///     .document(&json!({"field": "value"}))
    ///     .id("doc1")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn index<T>(&self, index: impl Into<String>) -> IndexRequestBuilder<T>
    where
        T: Serialize + ?Sized + Clone,
    {
        IndexRequest::new(self, index)
    }

    /// Create a builder for getting a document
    ///
    /// This allows for a fluent API to set options and execute the get operation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::{Client, Error};
    /// # use serde_json::Value;
    /// # async fn example() -> Result<(), Error> {
    /// # let client = Client::builder().base_url("http://localhost:9200").build()?;
    /// let response = client.documents()
    ///     .get::<Value>("my_index", "doc1")
    ///     .source(true)
    ///     .routing("user1")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get<'a, T>(
        &'a self,
        index: impl Into<String>,
        id: impl Into<String>,
    ) -> GetRequestBuilder<'a, T>
    where
        T: Clone + for<'de> Deserialize<'de> + Send + Sync,
    {
        GetRequest::new(self, index, id)
    }

    /// Create a builder for updating a document
    ///
    /// This allows for a fluent API to set options and execute the update operation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::{Client, Error};
    /// # use serde_json::json;
    /// # async fn example() -> Result<(), Error> {
    /// # let client = Client::builder().base_url("http://localhost:9200").build()?;
    /// let response = client.documents()
    ///     .update("my_index", "doc1", &json!({"field": "new value"}))
    ///     .doc_as_upsert(true)
    ///     .retry_on_conflict(3)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update<'a, T>(
        &'a self,
        index: impl Into<String>,
        id: impl Into<String>,
        document: &'a T,
    ) -> UpdateRequestBuilder<'a, T>
    where
        T: Clone + Serialize + ?Sized,
    {
        UpdateRequest::new(self, index, id, document)
    }

    /// Create a builder for deleting a document
    ///
    /// This allows for a fluent API to set options and execute the delete operation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::{Client, Error};
    /// # async fn example() -> Result<(), Error> {
    /// # let client = Client::builder().base_url("http://localhost:9200").build()?;
    /// let response = client.documents()
    ///     .delete("my_index", "doc1")
    ///     .refresh("true")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, index: impl Into<String>, id: impl Into<String>) -> DeleteRequestBuilder {
        DeleteRequest::new(self, index, id)
    }

    /// Create a builder for checking if a document exists
    ///
    /// This allows for a fluent API to set options and execute the exists operation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::{Client, Error};
    /// # async fn example() -> Result<(), Error> {
    /// # let client = Client::builder().base_url("http://localhost:9200").build()?;
    /// let exists = client.documents()
    ///     .exists("my_index", "doc1")
    ///     .routing("user1")
    ///     .realtime(true)
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn exists(&self, index: impl Into<String>, id: impl Into<String>) -> ExistsRequestBuilder {
        ExistsRequest::new(self, index, id)
    }

    /// Create a builder for refreshing an index
    ///
    /// This allows for a fluent API to execute the refresh operation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::{Client, Error};
    /// # async fn example() -> Result<(), Error> {
    /// # let client = Client::builder().base_url("http://localhost:9200").build()?;
    /// let response = client.documents()
    ///     .refresh("my_index")
    ///     .build()?
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn refresh(&self, index: impl Into<String>) -> RefreshRequestBuilder {
        RefreshRequest::new(self, index)
    }

    /// Create a builder for bulk operations
    ///
    /// This allows for a fluent API to execute bulk operations.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::{Client, Error};
    /// # use serde_json::json;
    /// # async fn example() -> Result<(), Error> {
    /// # let client = Client::builder().base_url("http://localhost:9200").build()?;
    /// let bulk_body = [
    ///     json!({"index": {"_index": "test", "_id": "1"}}),
    ///     json!({"field": "value1"}),
    ///     json!({"index": {"_index": "test", "_id": "2"}}),
    ///     json!({"field": "value2"}),
    /// ];
    /// let response = client.documents()
    ///     .bulk()
    ///     .operations(bulk_body.as_slice())
    ///     .refresh("true")
    ///     .build()?
    ///     .send()
    ///     .await?;
    /// Ok(())
    /// }
    /// ```
    pub fn bulk(&self) -> BulkRequestBuilder {
        BulkRequest::new(self)
    }

    /// Create a builder for multi-get operations
    ///
    /// This allows for a fluent API to execute multi-get operations.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use opensearch_api::{Client, Error};
    /// # use serde_json::json;
    /// # async fn example() -> Result<(), Error> {
    /// # let client = Client::builder().base_url("http://localhost:9200").build()?;
    /// let docs = vec![
    ///     json!({"_index": "test", "_id": "1"}),
    ///     json!({"_index": "test", "_id": "2"}),
    /// ];
    /// let response = client.documents()
    ///     .mget::<serde_json::Value>()
    ///     .docs(docs.as_slice())
    ///     .build()?
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn mget<'a, T>(&'a self) -> MgetRequestBuilder<'a, T>
    where
        T: Clone + for<'de> Deserialize<'de> + Send + Sync,
    {
        MgetRequest::new(self)
    }
}

impl crate::client::Client {
    /// Access the documents namespace
    pub fn documents(&self) -> DocumentsNamespace {
        DocumentsNamespace::new(self.clone())
    }
}

/// Builder for bulk operation requests
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct BulkRequest<'a> {
    /// Documents namespace reference
    #[builder(pattern = "immutable")]
    client: &'a DocumentsNamespace,

    /// Operations to perform in bulk
    #[builder(default)]
    operations: Option<&'a [serde_json::Value]>,

    /// Bulk options
    #[builder(default)]
    options: Option<BulkOptions>,
}

impl<'a> BulkRequestBuilder<'a> {
    /// Set the refresh option
    pub fn refresh(mut self, refresh: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.refresh = Some(refresh.into());
        self
    }

    /// Set the timeout option
    pub fn timeout(mut self, timeout: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.timeout = Some(timeout.into());
        self
    }

    /// Set the wait_for_active_shards option
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: WaitForActiveShards) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }

    /// Build and send the bulk request
    pub async fn send(self) -> Result<serde_json::Value, Error> {
        self.build().unwrap().send().await
    }
}

impl<'a> BulkRequest<'a> {
    /// Create a new bulk request builder
    pub(crate) fn new(client: &'a DocumentsNamespace) -> BulkRequestBuilder<'a> {
        BulkRequestBuilder::default().client(client)
    }

    /// Build and send the bulk request
    pub async fn send(self) -> Result<serde_json::Value, Error> {
        let mut path = "/_bulk".to_string();

        // Add query parameters from options
        let mut query_params = Vec::new();
        if let Some(options) = &self.options {
            if let Some(refresh) = &options.refresh {
                query_params.push(format!("refresh={}", refresh));
            }

            if let Some(timeout) = &options.timeout {
                query_params.push(format!("timeout={}", timeout));
            }

            if let Some(wait_for_active_shards) = &options.wait_for_active_shards {
                let value = match wait_for_active_shards {
                    WaitForActiveShards::Value(v) => v.to_string(),
                    WaitForActiveShards::Count(n) => n.to_string(),
                };
                query_params.push(format!("wait_for_active_shards={}", value));
            }
        }

        // Add query parameters to path
        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        // Create the request body
        let mut body = String::new();
        if let Some(operations) = self.operations {
            for operation in operations {
                body.push_str(&(serde_json::to_string(operation).unwrap() + "\n"));
            }
        }

        log::debug!("Sending BULK request to path: {}", path);
        self.client
            .client
            .request_with_string_body::<serde_json::Value>(Method::POST, &path, Some(body))
            .await
    }
}

/// Builder for multi-get operation requests
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct MgetRequest<'a, T: Clone + for<'de> Deserialize<'de> + Send + Sync> {
    /// Documents namespace reference
    #[builder(pattern = "immutable")]
    client: &'a DocumentsNamespace,

    /// Index to get documents from (optional)
    #[builder(default)]
    index: Option<String>,

    /// Document IDs to get (when index is specified)
    #[builder(default)]
    ids: Option<Vec<String>>,

    /// Documents to get (when no index is specified)
    #[builder(default)]
    docs: Option<&'a [serde_json::Value]>,

    /// Mget options
    #[builder(default)]
    options: Option<MgetOptions>,

    /// Type parameter marker
    #[builder(setter(skip), default = "std::marker::PhantomData")]
    _marker: std::marker::PhantomData<T>,
}

/// Response from a multi-get operation
#[derive(Debug, Clone, Deserialize)]
pub struct MgetResponse<T> {
    /// Documents retrieved
    pub docs: Vec<GetResponse<T>>,
}

impl<'a, T: Clone + for<'de> Deserialize<'de> + Send + Sync> MgetRequestBuilder<'a, T> {
    /// Set the preference option
    pub fn preference(mut self, preference: impl Into<String>) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.preference = Some(preference.into());
        self
    }

    /// Set the realtime option
    pub fn realtime(mut self, realtime: bool) -> Self {
        let options = self.options.get_or_insert_default().get_or_insert_default();
        options.realtime = Some(realtime);
        self
    }

    /// Build and send the mget request
    pub async fn send(self) -> Result<MgetResponse<T>, Error> {
        self.build().unwrap().send().await
    }
}

impl<'a, T: Clone + for<'de> Deserialize<'de> + Send + Sync> MgetRequest<'a, T> {
    /// Create a new mget request builder
    pub(crate) fn new(client: &'a DocumentsNamespace) -> MgetRequestBuilder<'a, T> {
        MgetRequestBuilder::default().client(client)
    }

    /// Build and send the mget request
    pub async fn send(self) -> Result<MgetResponse<T>, Error> {
        let mut path = if let Some(index) = &self.index {
            format!("/{}/_mget", index)
        } else {
            "/_mget".to_string()
        };

        // Add query parameters from options
        let mut query_params = Vec::new();
        if let Some(options) = &self.options {
            if let Some(preference) = &options.preference {
                query_params.push(format!("preference={}", preference));
            }

            if let Some(realtime) = options.realtime {
                query_params.push(format!("realtime={}", realtime));
            }
        }

        // Add query parameters to path
        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        // Create the request body
        let body = if let Some(ids) = &self.ids {
            if self.index.is_none() {
                return Err(Error::InvalidArgument(
                    "Index must be specified when using IDs".to_string(),
                ));
            }
            serde_json::json!({ "ids": ids })
        } else if let Some(docs) = &self.docs {
            serde_json::json!({ "docs": docs })
        } else {
            return Err(Error::InvalidArgument(
                "Either 'ids' or 'docs' must be specified".to_string(),
            ));
        };

        log::debug!("Sending MGET request to path: {}", path);
        self.client
            .client
            .request::<_, MgetResponse<T>>(Method::POST, &path, Some(&body))
            .await
    }
}
