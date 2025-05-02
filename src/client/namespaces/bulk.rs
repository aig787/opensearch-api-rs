//! Bulk operations namespace for OpenSearch
//!
//! This module provides APIs for executing bulk operations against OpenSearch
//! using a fluent builder pattern interface.
//!
//! # Basic Example
//!
//! ```no_run
//! use opensearch_api::{Client, Error};
//! use opensearch_api::types::bulk::BulkResponse;
//! use opensearch_api::types::common::RefreshPolicy;
//! use serde_json::json;
//!
//! async fn example() -> Result<BulkResponse, Error> {
//!     let client = Client::builder().base_url("http://localhost:9200").build()?;
//!     // Create a bulk request with multiple operations using the fluent API
//!     let response = client.bulk()
//!         .index("my_index", Some("doc1"), &json!({"field": "value1"}))
//!         .create("my_index", Some("doc2"), &json!({"field": "value2"}))
//!         .update("my_index", "doc3", &json!({"field": "updated"}))
//!         .delete::<serde_json::Value>("my_index", "doc4")
//!         .refresh(RefreshPolicy::True)
//!         .send()
//!         .await?;
//!
//!     println!("Bulk operation took {}ms", response.took);
//!     Ok(response)
//! }
//! ```
//!
//! # Advanced Example with Metadata Configuration
//!
//! ```no_run
//!  use opensearch_api::{Client, Error};
//!  use opensearch_api::types::common::{RefreshPolicy, VersionType};
//!  use serde_json::json;
//!  use opensearch_api::types::bulk::BulkResponse;
//!
//!  async fn example() -> Result<BulkResponse, Error> {
//!     let client = Client::builder().base_url("http://localhost:9200").build()?;
//!     // Example using metadata configuration
//!     client.bulk()
//!         .index(
//!             "my_index",
//!             Some("doc1"),
//!             &json!({"field": "value1"}),
//!        )
//!       // Add an update operation with upsert document
//!       .update_with_document(
//!           "my_index",
//!            "doc2",
//!            |doc| doc.doc(json!({"field": "updated"})).doc_as_upsert(true)
//!        )
//!        // Set request parameters
//!       .refresh(RefreshPolicy::True)
//!       .wait_for_active_shards("all")
//!       .send()
//!       .await
//!   }
//! ```

use crate::client::Client;
use crate::error::Error;
use crate::types::bulk::BulkResponse;
use crate::types::common::RefreshPolicy;
use derive_builder::Builder;
use reqwest::Method;
use serde::Serialize;
use serde_json::json;

/// Bulk operations namespace
#[derive(Debug, Clone)]
pub struct BulkNamespace<T = serde_json::Value>
where
    T: Serialize + Clone,
{
    /// Client instance
    client: Client,
    /// Operations to be executed
    operations: Vec<BulkOperation<T>>,
    /// Optional parameters for the request
    params: Option<BulkParams>,
}

/// Bulk operation parameters
#[derive(Debug, Clone, Default, Serialize, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct BulkParams {
    /// Refresh policy after the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub refresh: Option<RefreshPolicy>,

    /// Index to use for items that don't provide one
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub index: Option<String>,

    /// Routing value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub routing: Option<String>,

    /// Timeout for the operation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub timeout: Option<String>,

    /// Wait for active shards before proceeding
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub wait_for_active_shards: Option<String>,

    /// Pipeline to use for processing the documents
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub pipeline: Option<String>,

    /// Whether operations must target an alias
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub require_alias: Option<bool>,
}

impl BulkParams {
    /// Create a new builder for BulkParams
    pub fn builder() -> BulkParamsBuilder {
        BulkParamsBuilder::default()
    }
}

/// Represents a bulk operation
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BulkOperation<T>
where
    T: Serialize,
{
    /// Index a document (create if not exists, update if exists)
    Index {
        #[serde(rename = "index")]
        metadata: BulkIndexMetadata,
        #[serde(skip)]
        document: T,
    },

    /// Create a document (fails if already exists)
    Create {
        #[serde(rename = "create")]
        metadata: BulkIndexMetadata,
        #[serde(skip)]
        document: T,
    },

    /// Update an existing document
    Update {
        #[serde(rename = "update")]
        metadata: BulkUpdateMetadata,
        #[serde(skip)]
        document: BulkUpdateDocument<T>,
    },

    /// Delete a document
    Delete {
        #[serde(rename = "delete")]
        metadata: BulkDeleteMetadata,
    },
}

/// Metadata for bulk index and create operations
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct BulkIndexMetadata {
    /// Target index
    #[serde(rename = "_index", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub index: Option<String>,

    /// Document ID
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub id: Option<String>,
}

impl BulkIndexMetadata {
    /// Create a new builder for BulkIndexMetadata
    pub fn builder() -> BulkIndexMetadataBuilder {
        BulkIndexMetadataBuilder::default()
    }
}

/// Metadata for bulk update operations
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct BulkUpdateMetadata {
    /// Target index
    #[serde(rename = "_index", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub index: Option<String>,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,
}

impl BulkUpdateMetadata {
    /// Create a new builder for BulkUpdateMetadata
    pub fn builder() -> BulkUpdateMetadataBuilder {
        BulkUpdateMetadataBuilder::default()
    }
}

/// Metadata for bulk delete operations
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct BulkDeleteMetadata {
    /// Target index
    #[serde(rename = "_index", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub index: Option<String>,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,
}

impl BulkDeleteMetadata {
    /// Create a new builder for BulkDeleteMetadata
    pub fn builder() -> BulkDeleteMetadataBuilder {
        BulkDeleteMetadataBuilder::default()
    }
}

/// Update document wrapper for bulk update operations
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct BulkUpdateDocument<T>
where
    T: Serialize,
{
    /// Document data to update
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub doc: Option<T>,

    /// Whether this should be an upsert
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub doc_as_upsert: Option<bool>,

    /// Script to execute
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub script: Option<crate::types::script::Script>,
}

impl<T: Serialize> BulkUpdateDocument<T> {
    /// Create a new builder for BulkUpdateDocument
    pub fn builder(doc: T) -> BulkUpdateDocumentBuilder<T> {
        BulkUpdateDocumentBuilder::default().doc(doc)
    }

    /// Create a new builder for script-based updates
    pub fn script_builder(script: crate::types::script::Script) -> BulkUpdateDocumentBuilder<T> {
        BulkUpdateDocumentBuilder::default().script(script)
    }
}

impl<T> BulkNamespace<T>
where
    T: Serialize + Clone,
{
    /// Create a new bulk namespace instance
    pub fn new(client: Client) -> Self {
        BulkNamespace {
            client,
            operations: Vec::new(),
            params: None,
        }
    }

    /// Add an index operation (create or update) to the bulk request
    ///
    /// # Arguments
    /// * `index` - The index to store the document in
    /// * `id` - Optional ID for the document
    /// * `document` - The document to index
    ///
    /// # Returns
    /// The BulkNamespace with the added operation
    pub fn index(
        mut self,
        index: impl Into<String>,
        id: Option<impl Into<String>>,
        document: &T,
    ) -> Self {
        self.operations.push(BulkOperation::Index {
            metadata: BulkIndexMetadata {
                index: Some(index.into()),
                id: id.map(|id| id.into()),
            },
            document: document.clone(),
        });
        self
    }

    /// Add a create operation (fails if document already exists) to the bulk request
    ///
    /// # Arguments
    /// * `index` - The index to store the document in
    /// * `id` - Optional ID for the document
    /// * `document` - The document to create
    ///
    /// # Returns
    /// The BulkNamespace with the added operation
    pub fn create(
        mut self,
        index: impl Into<String>,
        id: Option<impl Into<String>>,
        document: &T,
    ) -> Self {
        self.operations.push(BulkOperation::Create {
            metadata: BulkIndexMetadata {
                index: Some(index.into()),
                id: id.map(|id| id.into()),
            },
            document: document.clone(),
        });
        self
    }

    /// Add an update operation to the bulk request
    ///
    /// # Arguments
    /// * `index` - The index containing the document
    /// * `id` - Document ID to update
    /// * `document` - The document with updated fields
    ///
    /// # Returns
    /// The BulkNamespace with the added operation
    pub fn update(mut self, index: impl Into<String>, id: impl Into<String>, document: &T) -> Self {
        self.operations.push(BulkOperation::Update {
            metadata: BulkUpdateMetadata {
                index: Some(index.into()),
                id: id.into(),
            },
            document: BulkUpdateDocument {
                doc: Some(document.clone()),
                doc_as_upsert: None,
                script: None,
            },
        });
        self
    }

    /// Add a script-based update operation to the bulk request
    ///
    /// # Arguments
    /// * `index` - The index containing the document
    /// * `id` - Document ID to update
    /// * `script` - The script to execute
    ///
    /// # Returns
    /// The BulkNamespace with the added operation
    pub fn update_with_script(
        mut self,
        index: impl Into<String>,
        id: impl Into<String>,
        script: crate::types::script::Script,
    ) -> Self {
        self.operations.push(BulkOperation::Update {
            metadata: BulkUpdateMetadata {
                index: Some(index.into()),
                id: id.into(),
            },
            document: BulkUpdateDocument {
                doc: None,
                doc_as_upsert: None,
                script: Some(script),
            },
        });
        self
    }

    /// Add a delete operation to the bulk request
    ///
    /// # Arguments
    /// * `index` - The index containing the document
    /// * `id` - Document ID to delete
    ///
    /// # Returns
    /// The BulkNamespace with the added operation
    pub fn delete<U: Serialize + Clone>(
        mut self,
        index: impl Into<String>,
        id: impl Into<String>,
    ) -> Self {
        self.operations.push(BulkOperation::Delete {
            metadata: BulkDeleteMetadata {
                index: Some(index.into()),
                id: id.into(),
            },
        });

        self
    }

    /// Add an index operation with additional metadata configuration
    pub fn index_with_metadata(
        mut self,
        index: impl Into<String>,
        id: Option<impl Into<String>>,
        document: &T,
        configure: impl FnOnce(BulkIndexMetadataBuilder) -> BulkIndexMetadataBuilder,
    ) -> Self {
        let mut builder = BulkIndexMetadataBuilder::default().index(index.into());

        if let Some(id) = id {
            builder = builder.id(id.into());
        }

        let metadata = configure(builder).build().unwrap();

        self.operations.push(BulkOperation::Index {
            metadata,
            document: document.clone(),
        });

        self
    }

    /// Add a create operation with additional metadata configuration
    pub fn create_with_metadata(
        mut self,
        index: impl Into<String>,
        id: Option<impl Into<String>>,
        document: &T,
        configure: impl FnOnce(BulkIndexMetadataBuilder) -> BulkIndexMetadataBuilder,
    ) -> Self {
        let mut builder = BulkIndexMetadataBuilder::default().index(index.into());

        if let Some(id) = id {
            builder = builder.id(id.into());
        }

        let metadata = configure(builder).build().unwrap();

        self.operations.push(BulkOperation::Create {
            metadata,
            document: document.clone(),
        });

        self
    }

    /// Add an update operation with additional metadata configuration
    pub fn update_with_metadata(
        mut self,
        index: impl Into<String>,
        id: impl Into<String>,
        document: &T,
        configure: impl FnOnce(BulkUpdateMetadataBuilder) -> BulkUpdateMetadataBuilder,
    ) -> Self {
        let builder = BulkUpdateMetadataBuilder::default()
            .index(index.into())
            .id(id.into());

        let metadata = configure(builder).build().unwrap();

        self.operations.push(BulkOperation::Update {
            metadata,
            document: BulkUpdateDocument {
                doc: Some(document.to_owned()),
                doc_as_upsert: None,
                script: None,
            },
        });

        self
    }

    /// Add an update operation with a custom update document
    ///
    /// # Example
    ///
    /// ```no_run
    /// use opensearch_api::{Client, Error};
    /// use opensearch_api::types::script::{Script, InlineScript};
    /// use serde_json::json;
    /// use std::collections::HashMap;
    ///
    /// async fn example() -> Result<(), Error> {
    ///     let client = Client::builder()
    ///         .base_url("http://localhost:9200")
    ///         .build()?;
    ///
    ///     // Update with partial document
    ///     let response = client.bulk::<serde_json::Value>()
    ///         .update_with_document(
    ///             "my_index",
    ///             "doc1",
    ///             |doc| doc.doc(json!({"field": "updated"}))
    ///         )
    ///         .send()
    ///         .await?;
    ///
    ///     // Update with script
    ///     let mut params = HashMap::new();
    ///     params.insert("count".to_string(), json!(5));
    ///
    ///     let script = Script::Inline(InlineScript {
    ///         source: "ctx._source.count += params.count".to_string(),
    ///         lang: Some("painless".to_string()),
    ///         params: Some(params),
    ///         options: None,
    ///     });
    ///
    ///     let response = client.bulk::<serde_json::Value>()
    ///         .update_with_document(
    ///             "my_index",
    ///             "doc1",
    ///             |doc| doc.script(script)
    ///         )
    ///         .send()
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn update_with_document(
        mut self,
        index: impl Into<String>,
        id: impl Into<String>,
        document: impl FnOnce(BulkUpdateDocumentBuilder<T>) -> BulkUpdateDocumentBuilder<T>,
    ) -> Self {
        let metadata = BulkUpdateMetadata {
            index: Some(index.into()),
            id: id.into(),
        };

        // Start with a default builder and let the caller configure it
        let doc_builder = document(BulkUpdateDocumentBuilder::default());
        let update_doc = doc_builder.build().unwrap();

        self.operations.push(BulkOperation::Update {
            metadata,
            document: update_doc,
        });

        self
    }

    /// Add a delete operation with additional metadata configuration
    pub fn delete_with_metadata<U: Serialize + Clone>(
        self,
        index: impl Into<String>,
        id: impl Into<String>,
        configure: impl FnOnce(BulkDeleteMetadataBuilder) -> BulkDeleteMetadataBuilder,
    ) -> BulkNamespace<U> {
        let mut new_namespace = BulkNamespace {
            client: self.client.clone(),
            operations: Vec::new(),
            params: self.params,
        };

        let builder = BulkDeleteMetadataBuilder::default()
            .index(index.into())
            .id(id.into());

        let metadata = configure(builder).build().unwrap();

        new_namespace
            .operations
            .push(BulkOperation::Delete { metadata });

        new_namespace
    }

    /// Set the refresh policy for the bulk request
    pub fn refresh(mut self, refresh: impl Into<RefreshPolicy>) -> Self {
        let mut params = self.params.unwrap_or_default();
        params.refresh = Some(refresh.into());
        self.params = Some(params);
        self
    }

    /// Set the default index for operations that don't specify one
    pub fn default_index(mut self, index: impl Into<String>) -> Self {
        let mut params = self.params.unwrap_or_default();
        params.index = Some(index.into());
        self.params = Some(params);
        self
    }

    /// Set the routing value for the bulk request
    pub fn routing(mut self, routing: impl Into<String>) -> Self {
        let mut params = self.params.unwrap_or_default();
        params.routing = Some(routing.into());
        self.params = Some(params);
        self
    }

    /// Set the timeout for the bulk request
    pub fn timeout(mut self, timeout: impl Into<String>) -> Self {
        let mut params = self.params.unwrap_or_default();
        params.timeout = Some(timeout.into());
        self.params = Some(params);
        self
    }

    /// Set the wait_for_active_shards parameter for the bulk request
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: impl Into<String>) -> Self {
        let mut params = self.params.unwrap_or_default();
        params.wait_for_active_shards = Some(wait_for_active_shards.into());
        self.params = Some(params);
        self
    }

    /// Set the pipeline for the bulk request
    pub fn pipeline(mut self, pipeline: impl Into<String>) -> Self {
        let mut params = self.params.unwrap_or_default();
        params.pipeline = Some(pipeline.into());
        self.params = Some(params);
        self
    }

    /// Set all parameters at once using a BulkParams instance
    pub fn params(mut self, params: BulkParams) -> Self {
        self.params = Some(params);
        self
    }

    /// Add multiple operations at once
    pub fn operations(mut self, operations: Vec<BulkOperation<T>>) -> Self {
        self.operations.extend(operations);
        self
    }

    /// Execute the bulk request with all added operations
    pub async fn send(self) -> Result<BulkResponse, Error> {
        let mut path = String::from("/_bulk");

        // Add query parameters
        if let Some(params) = self.params {
            let mut query_parts = Vec::new();

            if let Some(refresh) = params.refresh {
                query_parts.push(format!("refresh={}", refresh));
            }

            if let Some(index) = params.index {
                query_parts.push(format!("index={}", index));
            }

            if let Some(routing) = params.routing {
                query_parts.push(format!("routing={}", routing));
            }

            if let Some(timeout) = params.timeout {
                query_parts.push(format!("timeout={}", timeout));
            }

            if let Some(wait_for_active_shards) = params.wait_for_active_shards {
                query_parts.push(format!("wait_for_active_shards={}", wait_for_active_shards));
            }

            if let Some(pipeline) = params.pipeline {
                query_parts.push(format!("pipeline={}", pipeline));
            }

            if let Some(require_alias) = params.require_alias {
                query_parts.push(format!("require_alias={}", require_alias));
            }

            if !query_parts.is_empty() {
                path.push('?');
                path.push_str(&query_parts.join("&"));
            }
        }


        // Build the NDJSON payload
        let mut payload = String::new();
        for operation in self.operations {
            match operation {
                BulkOperation::Index { metadata, document } => {
                    // Add action line
                    payload.push_str(&serde_json::to_string(&json!({ "index": metadata }))?);
                    payload.push('\n');
                    // Add document data
                    payload.push_str(&serde_json::to_string(&document)?);
                    payload.push('\n');
                }
                BulkOperation::Create { metadata, document } => {
                    // Add action line
                    payload.push_str(&serde_json::to_string(&json!({ "create": metadata }))?);
                    payload.push('\n');
                    // Add document data
                    payload.push_str(&serde_json::to_string(&document)?);
                    payload.push('\n');
                }
                BulkOperation::Update { metadata, document } => {
                    // Add action line
                    payload.push_str(&serde_json::to_string(&json!({ "update": metadata }))?);
                    payload.push('\n');
                    // Add document data
                    payload.push_str(&serde_json::to_string(&document)?);
                    payload.push('\n');
                }
                BulkOperation::Delete { metadata } => {
                    // Add action line (no document for delete)
                    payload.push_str(&serde_json::to_string(&json!({ "delete": metadata }))?);
                    payload.push('\n');
                }
            }
        }


        // Set headers for NDJSON content
        let headers = vec![("Content-Type", "application/x-ndjson")];

        // Send the request using the client's request method
        self.client
            .request_with_headers(Method::POST, &path, Some(payload), Some(headers))
            .await
    }
}

impl<T> BulkOperation<T>
where
    T: Serialize,
{
    /// Create an index operation
    pub fn index(index: Option<String>, id: Option<String>, document: T) -> Self {
        BulkOperation::Index {
            metadata: BulkIndexMetadata { index, id },
            document,
        }
    }

    /// Create a create operation
    pub fn create(index: Option<String>, id: Option<String>, document: T) -> Self {
        BulkOperation::Create {
            metadata: BulkIndexMetadata { index, id },
            document,
        }
    }

    /// Create an update operation
    pub fn update(index: Option<String>, id: String, document: T) -> Self {
        BulkOperation::Update {
            metadata: BulkUpdateMetadata {
                index,
                id,
            },
            document: BulkUpdateDocument {
                doc: Some(document),
                doc_as_upsert: None,
                script: None,
            },
        }
    }

    /// Create a script-based update operation
    pub fn update_with_script(
        index: Option<String>,
        id: String,
        script: crate::types::script::Script,
    ) -> Self {
        BulkOperation::Update {
            metadata: BulkUpdateMetadata {
                index,
                id,
            },
            document: BulkUpdateDocument {
                doc: None,
                doc_as_upsert: None,
                script: Some(script),
            },
        }
    }

    /// Create a delete operation
    pub fn delete(index: Option<String>, id: String) -> Self {
        BulkOperation::Delete {
            metadata: BulkDeleteMetadata { index, id },
        }
    }
}

impl crate::client::Client {
    /// Access the bulk operations namespace
    ///
    /// This provides a fluent builder pattern API for bulk operations.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use opensearch_api::{Client, Error};
    /// use opensearch_api::types::common::RefreshPolicy;
    /// use serde_json::json;
    /// async fn example() -> Result<(), Error> {
    ///     let client = Client::builder().base_url("http://localhost:9200").build()?;
    ///     let response = client.bulk()
    ///        .index("my_index", Some("doc1"), &json!({"field": "value1"}))
    ///        .delete::<serde_json::Value>("my_index", "doc3")
    ///        .refresh(RefreshPolicy::True)
    ///        .send()
    ///        .await?;
    ///  Ok(())
    /// }
    /// ```
    pub fn bulk<T: Serialize + Clone>(&self) -> BulkNamespace<T> {
        BulkNamespace::new(self.clone())
    }
}
