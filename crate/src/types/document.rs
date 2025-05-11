//! Document-related data types

use crate::types::common::ShardStatistics;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Document metadata
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentMetadata {
    /// Index where the document is stored
    #[serde(rename = "_index")]
    pub index: String,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document version
    #[serde(rename = "_version", skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,

    /// Sequence number for optimistic concurrency control
    #[serde(rename = "_seq_no", skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<u64>,

    /// Primary term for optimistic concurrency control
    #[serde(rename = "_primary_term", skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<u64>,
}

/// Wait For Active Shards options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WaitForActiveShards {
    /// Number of active shards
    Count(i32),
    /// Special values like "all" or "majority"
    Value(String),
}

/// Options for indexing a document
#[derive(Default, Debug, Clone, Builder)]
#[builder(setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct IndexOptions {
    /// Whether to refresh the affected shards after the operation
    #[builder(default)]
    pub refresh: Option<String>,

    /// Custom routing value
    #[builder(default)]
    pub routing: Option<String>,

    /// Operation timeout
    #[builder(default)]
    pub timeout: Option<String>,

    /// Document version for optimistic concurrency control
    #[builder(default)]
    pub version: Option<i64>,

    /// Type of versioning to use
    #[builder(default)]
    pub version_type: Option<String>,

    /// Number of active shards to wait for
    #[builder(default)]
    pub wait_for_active_shards: Option<WaitForActiveShards>,
}

impl IndexOptions {
    /// Create a new builder for IndexOptions
    pub fn builder() -> IndexOptionsBuilder {
        IndexOptionsBuilder::default()
    }
}

/// Options for retrieving a document
#[derive(Default, Debug, Clone, Builder)]
#[builder(setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct GetOptions {
    /// Whether to include the _source field in the response
    #[builder(default)]
    pub source: Option<bool>,

    /// List of source fields to include
    #[builder(default)]
    pub source_includes: Option<Vec<String>>,

    /// List of source fields to exclude
    #[builder(default)]
    pub source_excludes: Option<Vec<String>>,

    /// Custom routing value
    #[builder(default)]
    pub routing: Option<String>,

    /// Preference value for executing the request
    #[builder(default)]
    pub preference: Option<String>,

    /// Whether to execute the get in realtime or search mode
    #[builder(default)]
    pub realtime: Option<bool>,

    /// Whether to refresh the shard before retrieving the document
    #[builder(default)]
    pub refresh: Option<bool>,

    /// Document version for optimistic concurrency control
    #[builder(default)]
    pub version: Option<i64>,

    /// Type of versioning to use
    #[builder(default)]
    pub version_type: Option<String>,
}

impl GetOptions {
    /// Create a new builder for GetOptions
    pub fn builder() -> GetOptionsBuilder {
        GetOptionsBuilder::default()
    }
}

/// Options for checking if a document exists
#[derive(Default, Debug, Clone, Builder)]
#[builder(setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct ExistsOptions {
    /// Custom routing value
    #[builder(default)]
    pub routing: Option<String>,

    /// Preference value for executing the request
    #[builder(default)]
    pub preference: Option<String>,

    /// Whether to execute the check in realtime or search mode
    #[builder(default)]
    pub realtime: Option<bool>,

    /// Whether to refresh the shard before checking
    #[builder(default)]
    pub refresh: Option<bool>,

    /// Document version for optimistic concurrency control
    #[builder(default)]
    pub version: Option<i64>,

    /// Type of versioning to use
    #[builder(default)]
    pub version_type: Option<String>,
}

impl ExistsOptions {
    /// Create a new builder for ExistsOptions
    pub fn builder() -> ExistsOptionsBuilder {
        ExistsOptionsBuilder::default()
    }
}

/// Options for updating a document
#[derive(Default, Debug, Clone, Builder)]
#[builder(setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct UpdateOptions {
    /// Whether to use the document as the upsert document
    #[builder(default = "Some(true)")]
    pub doc_as_upsert: Option<bool>,

    /// Number of retries for conflict errors
    #[builder(default)]
    pub retry_on_conflict: Option<i32>,

    /// Whether to refresh the affected shards after the operation
    #[builder(default)]
    pub refresh: Option<String>,

    /// Custom routing value
    #[builder(default)]
    pub routing: Option<String>,

    /// Operation timeout
    #[builder(default)]
    pub timeout: Option<String>,

    /// Number of active shards to wait for
    #[builder(default)]
    pub wait_for_active_shards: Option<WaitForActiveShards>,

    /// Whether to require the destination to be an alias
    #[builder(default)]
    pub require_alias: Option<bool>,
}

impl UpdateOptions {
    /// Create a new builder for UpdateOptions
    pub fn builder() -> UpdateOptionsBuilder {
        UpdateOptionsBuilder::default()
    }
}

/// Options for deleting a document
#[derive(Default, Debug, Clone, Builder)]
#[builder(setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct DeleteOptions {
    /// Whether to refresh the affected shards after the operation
    #[builder(default)]
    pub refresh: Option<String>,

    /// Custom routing value
    #[builder(default)]
    pub routing: Option<String>,

    /// Operation timeout
    #[builder(default)]
    pub timeout: Option<String>,

    /// Document version for optimistic concurrency control
    #[builder(default)]
    pub version: Option<i64>,

    /// Type of versioning to use
    #[builder(default)]
    pub version_type: Option<String>,

    /// Number of active shards to wait for
    #[builder(default)]
    pub wait_for_active_shards: Option<WaitForActiveShards>,
}

impl DeleteOptions {
    /// Create a new builder for DeleteOptions
    pub fn builder() -> DeleteOptionsBuilder {
        DeleteOptionsBuilder::default()
    }
}
/// Options for bulk operation
#[derive(Default, Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct BulkOptions {
    /// Refresh policy for the operation
    #[builder(default)]
    pub refresh: Option<String>,

    /// Timeout for the operation
    #[builder(default)]
    pub timeout: Option<String>,

    /// Number of active shards that must be available before proceeding
    #[builder(default)]
    pub wait_for_active_shards: Option<WaitForActiveShards>,
}

impl BulkOptions {
    /// Create a new builder for BulkOptions
    pub fn builder() -> BulkOptionsBuilder {
        BulkOptionsBuilder::default()
    }
}

/// Options for multi-get operation
#[derive(Default, Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct MgetOptions {
    /// Preference value for the shard selection
    #[builder(default)]
    pub preference: Option<String>,

    /// Whether to get the documents in real-time
    #[builder(default)]
    pub realtime: Option<bool>,

    /// Whether to refresh the shard before getting the documents
    #[builder(default)]
    pub refresh: Option<bool>,

    /// Routing value for the documents
    #[builder(default)]
    pub routing: Option<String>,
}

impl MgetOptions {
    /// Create a new builder for MgetOptions
    pub fn builder() -> MgetOptionsBuilder {
        MgetOptionsBuilder::default()
    }
}

/// Document to retrieve in a multi-get operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgetDoc {
    /// Index where the document is stored
    #[serde(rename = "_index")]
    pub index: String,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Whether to include the source in the response
    #[serde(rename = "_source", skip_serializing_if = "Option::is_none")]
    pub source: Option<bool>,

    /// Fields to include in the source
    #[serde(rename = "_source_includes", skip_serializing_if = "Option::is_none")]
    pub source_includes: Option<Vec<String>>,

    /// Fields to exclude from the source
    #[serde(rename = "_source_excludes", skip_serializing_if = "Option::is_none")]
    pub source_excludes: Option<Vec<String>>,
}

/// Response for a multi-get operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MgetResponse<T = serde_json::Value> {
    /// Documents retrieved
    pub docs: Vec<Option<GetResponse<T>>>,
}

/// Options for delete-by-query operation
#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(strip_option))]
pub struct DeleteByQueryOptions {
    /// How to handle conflicts during the operation
    #[builder(default)]
    pub conflicts: Option<String>,

    /// Refresh policy for the operation
    #[builder(default)]
    pub refresh: Option<bool>,

    /// Routing value for the documents
    #[builder(default)]
    pub routing: Option<String>,

    /// Size of the scroll request batch
    #[builder(default)]
    pub scroll_size: Option<u32>,

    /// Timeout for the operation
    #[builder(default)]
    pub timeout: Option<String>,

    /// Number of active shards that must be available before proceeding
    #[builder(default)]
    pub wait_for_active_shards: Option<WaitForActiveShards>,

    /// Number of slices for parallel execution
    #[builder(default)]
    pub slices: Option<u32>,

    /// Maximum number of documents to process
    #[builder(default)]
    pub max_docs: Option<u32>,
}

impl DeleteByQueryOptions {
    /// Create a new builder for DeleteByQueryOptions
    pub fn builder() -> DeleteByQueryOptionsBuilder {
        DeleteByQueryOptionsBuilder::default()
    }
}

/// Response for a delete-by-query operation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteByQueryResponse {
    /// Time taken to execute the operation in milliseconds
    pub took: i64,

    /// Whether the operation timed out
    pub timed_out: bool,

    /// Total number of documents that were processed
    pub total: u64,

    /// Number of documents that were deleted
    pub deleted: u64,

    /// Number of batches that were executed
    pub batches: u64,

    /// Number of version conflicts that were detected
    pub version_conflicts: u64,

    /// Number of search retries that were performed
    pub retries: DeleteByQueryRetries,

    /// Number of throttling activations that occurred
    pub throttled_millis: u64,

    /// Requests per second that were set
    #[serde(rename = "requests_per_second")]
    pub requests_per_second: f64,

    /// Time spent throttling in milliseconds
    #[serde(rename = "throttled_until_millis")]
    pub throttled_until_millis: u64,

    /// Number of documents that failed to be processed
    pub failures: Vec<serde_json::Value>,
}

/// Retry information for delete-by-query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteByQueryRetries {
    /// Number of bulk retries
    pub bulk: u64,

    /// Number of search retries
    pub search: u64,
}

/// Options for update-by-query operation
#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(strip_option))]
pub struct UpdateByQueryOptions {
    /// How to handle conflicts during the operation
    #[builder(default)]
    pub conflicts: Option<String>,

    /// Refresh policy for the operation
    #[builder(default)]
    pub refresh: Option<bool>,

    /// Routing value for the documents
    #[builder(default)]
    pub routing: Option<String>,

    /// Size of the scroll request batch
    #[builder(default)]
    pub scroll_size: Option<u32>,

    /// Timeout for the operation
    #[builder(default)]
    pub timeout: Option<String>,

    /// Number of active shards that must be available before proceeding
    #[builder(default)]
    pub wait_for_active_shards: Option<WaitForActiveShards>,

    /// Number of slices for parallel execution
    #[builder(default)]
    pub slices: Option<u32>,

    /// Maximum number of documents to process
    #[builder(default)]
    pub max_docs: Option<u32>,
}

impl UpdateByQueryOptions {
    /// Create a new builder for UpdateByQueryOptions
    pub fn builder() -> UpdateByQueryOptionsBuilder {
        UpdateByQueryOptionsBuilder::default()
    }
}

/// Response for a update-by-query operation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateByQueryResponse {
    /// Time taken to execute the operation in milliseconds
    pub took: i64,

    /// Whether the operation timed out
    pub timed_out: bool,

    /// Total number of documents that were processed
    pub total: u64,

    /// Number of documents that were updated
    pub updated: u64,

    /// Number of documents that were created
    pub created: Option<u64>,

    /// Number of documents that were deleted
    pub deleted: Option<u64>,

    /// Number of batches that were executed
    pub batches: u64,

    /// Number of version conflicts that were detected
    pub version_conflicts: u64,

    /// Number of search retries that were performed
    pub retries: UpdateByQueryRetries,

    /// Number of throttling activations that occurred
    pub throttled_millis: u64,

    /// Requests per second that were set
    #[serde(rename = "requests_per_second")]
    pub requests_per_second: f64,

    /// Time spent throttling in milliseconds
    #[serde(rename = "throttled_until_millis")]
    pub throttled_until_millis: u64,

    /// Number of documents that failed to be processed
    pub failures: Vec<serde_json::Value>,
}

/// Retry information for update-by-query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateByQueryRetries {
    /// Number of bulk retries
    pub bulk: u64,

    /// Number of search retries
    pub search: u64,
}

/// Definition of a bulk operation
#[derive(Debug, Clone)]
pub enum BulkOperation<T> {
    /// Index operation (create or update a document)
    Index {
        /// Index name
        index: String,
        /// Document ID (optional, auto-generated if not provided)
        id: Option<String>,
        /// Document to index
        document: T,
    },

    /// Create operation (create a document, fail if already exists)
    Create {
        /// Index name
        index: String,
        /// Document ID (optional, auto-generated if not provided)
        id: Option<String>,
        /// Document to create
        document: T,
    },

    /// Update operation (update an existing document)
    Update {
        /// Index name
        index: String,
        /// Document ID (required for update)
        id: String,
        /// Document or partial document to update
        document: T,
    },

    /// Delete operation (delete an existing document)
    Delete {
        /// Index name
        index: String,
        /// Document ID to delete
        id: String,
    },
}

/// Response for a document indexing operation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexResponse {
    /// Index where the document was indexed
    #[serde(rename = "_index")]
    pub index: String,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document version after indexing
    #[serde(rename = "_version")]
    pub version: u64,

    /// Result of the operation (e.g., "created", "updated")
    pub result: String,

    /// Information about shards involved in the operation
    pub _shards: ShardStatistics,

    /// Sequence number for optimistic concurrency control
    #[serde(rename = "_seq_no")]
    pub seq_no: u64,

    /// Primary term for optimistic concurrency control
    #[serde(rename = "_primary_term")]
    pub primary_term: u64,
}

/// Response for a document get operation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse<T = serde_json::Value> {
    /// Index where the document is stored
    #[serde(rename = "_index")]
    pub index: String,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Whether the document was found
    pub found: bool,

    /// Document version
    #[serde(rename = "_version", skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,

    /// Sequence number for optimistic concurrency control
    #[serde(rename = "_seq_no", skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<u64>,

    /// Primary term for optimistic concurrency control
    #[serde(rename = "_primary_term", skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<u64>,

    /// Document source data
    #[serde(rename = "_source", skip_serializing_if = "Option::is_none")]
    pub source: Option<T>,

    /// Document fields (when specific fields are requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, serde_json::Value>>,
}

impl<T> GetResponse<T> {
    pub fn source_ref_required(&self) -> &T {
        self.source.as_ref().expect("source is required")
    }
}

/// Response for a document delete operation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// Index where the document was deleted
    #[serde(rename = "_index")]
    pub index: String,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document version after deletion
    #[serde(rename = "_version")]
    pub version: u64,

    /// Result of the operation (e.g., "deleted", "not_found")
    pub result: String,

    /// Information about shards involved in the operation
    pub _shards: ShardStatistics,

    /// Sequence number for optimistic concurrency control
    #[serde(rename = "_seq_no")]
    pub seq_no: u64,

    /// Primary term for optimistic concurrency control
    #[serde(rename = "_primary_term")]
    pub primary_term: u64,
}

/// Response for a document update operation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateResponse {
    /// Index where the document was updated
    #[serde(rename = "_index")]
    pub index: String,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document version after update
    #[serde(rename = "_version")]
    pub version: u64,

    /// Result of the operation (e.g., "updated", "noop")
    pub result: String,

    /// Information about shards involved in the operation
    pub _shards: ShardStatistics,

    /// Sequence number for optimistic concurrency control
    #[serde(rename = "_seq_no")]
    pub seq_no: u64,

    /// Primary term for optimistic concurrency control
    #[serde(rename = "_primary_term")]
    pub primary_term: u64,

    /// Updated document source (when requested)
    #[serde(rename = "get", skip_serializing_if = "Option::is_none")]
    pub get_result: Option<GetResponse>,
}

/// Options for refreshing an index
#[derive(Debug, Clone, Builder)]
pub struct RefreshOptions {
    /// Name of the index to refresh
    pub index: String,
}

impl RefreshOptions {
    /// Create a new builder for RefreshOptions
    pub fn builder() -> RefreshOptionsBuilder {
        RefreshOptionsBuilder::default()
    }

    /// Create new RefreshOptions with the given index name
    pub fn new(index: impl Into<String>) -> Self {
        Self {
            index: index.into(),
        }
    }
}

/// Response for refresh operation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefreshResponse {
    /// Number of shards that were successful
    pub _shards: ShardStatistics,
}

#[cfg(test)]
mod tests {
    use crate::documents::{DeleteResponse, GetResponse, IndexResponse};
    use crate::types::common::ShardStatistics;
    use crate::types::document::{DocumentMetadata, WaitForActiveShards};
    use crate::Error;
    use serde_json::{json, Value};

    /// Helper function to test serialization and deserialization roundtrip
    fn test_serde_roundtrip<T>(value: &T, expected_json: &str) -> Result<(), Error>
    where
        T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
    {
        // Serialize to string
        let serialized = serde_json::to_string(&value)?;

        // Parse both as Value for comparison that ignores formatting/whitespace
        let value_json: Value = serde_json::from_str(&serialized)?;
        let expected_value: Value = serde_json::from_str(expected_json)?;

        assert_eq!(
            value_json, expected_value,
            "Serialized JSON doesn't match expected JSON"
        );

        // Deserialize back
        let deserialized: T = serde_json::from_str(&serialized)?;

        // Verify roundtrip
        assert_eq!(
            &deserialized, value,
            "Deserialized value doesn't match original"
        );

        Ok(())
    }

    #[test]
    fn test_document_metadata() -> Result<(), Error> {
        let metadata = DocumentMetadata {
            index: "test-index".to_string(),
            id: "123".to_string(),
            version: Some(1),
            seq_no: Some(42),
            primary_term: Some(1),
        };

        let expected_json = r#"{
                    "_index": "test-index",
                    "_id": "123",
                    "_version": 1,
                    "_seq_no": 42,
                    "_primary_term": 1
                }"#;

        test_serde_roundtrip(&metadata, expected_json)
    }

    #[test]
    fn test_wait_for_active_shards() -> Result<(), Error> {
        let count = WaitForActiveShards::Count(2);
        let expected_json = "2";
        test_serde_roundtrip(&count, expected_json)?;

        let all = WaitForActiveShards::Value("all".to_string());
        let expected_json = r#""all""#;
        test_serde_roundtrip(&all, expected_json)
    }

    #[test]
    fn test_index_response() -> Result<(), Error> {
        let response = IndexResponse {
            index: "test-index".to_string(),
            id: "123".to_string(),
            version: 1,
            result: "created".to_string(),
            _shards: ShardStatistics {
                total: 2,
                successful: 2,
                failed: 0,
                failures: vec![],
            },
            seq_no: 0,
            primary_term: 1,
        };

        let expected_json = r#"{
                    "_index": "test-index",
                    "_id": "123",
                    "_version": 1,
                    "result": "created",
                    "_shards": {
                        "total": 2,
                        "successful": 2,
                        "failed": 0,
                        "failures": []
                    },
                    "_seq_no": 0,
                    "_primary_term": 1
                }"#;

        test_serde_roundtrip(&response, expected_json)
    }

    #[test]
    fn test_get_response() -> Result<(), Error> {
        let response: GetResponse<serde_json::Value> = GetResponse {
            index: "test-index".to_string(),
            id: "123".to_string(),
            found: true,
            version: Some(1),
            seq_no: Some(42),
            primary_term: Some(1),
            source: Some(json!({
                "title": "Test Document",
                "content": "This is a test document"
            })),
            fields: None,
        };

        let expected_json = r#"{
                    "_index": "test-index",
                    "_id": "123",
                    "found": true,
                    "_version": 1,
                    "_seq_no": 42,
                    "_primary_term": 1,
                    "_source": {
                        "title": "Test Document",
                        "content": "This is a test document"
                    }
                }"#;

        test_serde_roundtrip(&response, expected_json)
    }

    #[test]
    fn test_delete_response() -> Result<(), Error> {
        let response = DeleteResponse {
            index: "test-index".to_string(),
            id: "123".to_string(),
            version: 2,
            result: "deleted".to_string(),
            _shards: ShardStatistics {
                total: 2,
                successful: 2,
                failed: 0,
                failures: vec![],
            },
            seq_no: 43,
            primary_term: 1,
        };

        let expected_json = r#"{
                    "_index": "test-index",
                    "_id": "123",
                    "_version": 2,
                    "result": "deleted",
                    "_shards": {
                        "total": 2,
                        "successful": 2,
                        "failed": 0,
                        "failures": []
                    },
                    "_seq_no": 43,
                    "_primary_term": 1
                }"#;

        test_serde_roundtrip(&response, expected_json)
    }
}
