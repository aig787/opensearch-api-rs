//! Document-related data types

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
    pub _shards: super::ShardStatistics,

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
    pub _shards: super::ShardStatistics,

    /// Sequence number for optimistic concurrency control
    #[serde(rename = "_seq_no")]
    pub seq_no: u64,

    /// Primary term for optimistic concurrency control
    #[serde(rename = "_primary_term")]
    pub primary_term: u64,
}

/// Response for a bulk operation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkResponse {
    /// Time taken to execute the bulk operation in milliseconds
    pub took: i64,

    /// Whether the bulk operation timed out
    pub timed_out: bool,

    /// Information about the items in the bulk operation
    pub items: Vec<BulkResponseItem>,

    /// Information about shards involved in the operation
    pub _shards: super::ShardStatistics,
}

/// Individual item in a bulk operation response
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkResponseItem {
    /// Index operation response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<IndexResponse>,

    /// Create operation response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<IndexResponse>,

    /// Update operation response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateResponse>,

    /// Delete operation response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<DeleteResponse>,
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
    pub _shards: super::ShardStatistics,

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
