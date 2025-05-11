use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

/// Represents possible errors that can occur during bulk operations
#[derive(Debug, Error)]
pub enum BulkError {
    #[error("Failed to serialize request: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Bulk operation failed: {0}")]
    OperationFailed(String),
}

/// Represents the types of operations that can be performed in a bulk request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BulkOperation {
    Index,
    Create,
    Update,
    Delete,
}

/// Update operation specific fields for bulk requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUpdateOperation<T>
where
    T: Serialize + Clone,
{
    /// The partial document to be merged with the existing document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<T>,

    /// Script to execute for the update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,

    /// Whether to upsert the document if it doesn't exist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_as_upsert: Option<bool>,

    /// Document to insert if the document doesn't exist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upsert: Option<T>,
}

/// Represents the response for a single bulk operation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulkOperationResponse {
    /// The index that was operated on
    #[serde(rename = "_index")]
    pub index: String,

    /// The document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// The version of the document after the operation
    #[serde(rename = "_version", skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,

    /// The result of the operation
    pub result: Option<String>,

    /// The HTTP status code
    pub status: u16,

    /// Whether the operation was successful
    #[serde(rename = "_shards")]
    pub shards: Option<ShardResponse>,

    /// The sequence number
    #[serde(rename = "_seq_no", skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<u64>,

    /// The primary term
    #[serde(rename = "_primary_term", skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<u64>,

    /// Error information if the operation failed
    pub error: Option<BulkResponseError>,
}

/// Represents shard information in a response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShardResponse {
    /// Total number of shards
    pub total: u32,

    /// Number of successful shards
    pub successful: u32,

    /// Number of failed shards
    pub failed: u32,
}

/// Error information in a bulk response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulkResponseError {
    /// The type of error
    #[serde(rename = "type")]
    pub error_type: String,

    /// The error reason
    pub reason: String,

    /// Additional error information
    #[serde(flatten)]
    pub additional: HashMap<String, Value>,
}

/// Represents the items array in a bulk response
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulkResponseItem {
    /// Index operation response
    pub index: Option<BulkOperationResponse>,

    /// Create operation response
    pub create: Option<BulkOperationResponse>,

    /// Update operation response
    pub update: Option<BulkOperationResponse>,

    /// Delete operation response
    pub delete: Option<BulkOperationResponse>,
}

/// Represents the complete response from a bulk operation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulkResponse {
    /// Time in milliseconds the operation took
    pub took: u64,

    /// Whether the operation timed out
    pub errors: bool,

    /// Array of response items, one for each operation in the request
    pub items: Vec<BulkResponseItem>,
}
