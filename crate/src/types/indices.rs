//! OpenSearch indices response types
//! Types for Indices operations

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Response from delete index operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteIndexResponse {
    /// Whether the operation was acknowledged
    pub acknowledged: bool,
}

/// Response from close index operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseIndexResponse {
    /// Whether the operation was acknowledged
    pub acknowledged: bool,
}

/// Response from open index operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenIndexResponse {
    /// Whether the operation was acknowledged
    pub acknowledged: bool,
}

/// Response from update index settings operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIndexSettingsResponse {
    /// Whether the operation was acknowledged
    pub acknowledged: bool,
}

/// Response from put mapping operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutMappingResponse {
    /// Whether the operation was acknowledged
    pub acknowledged: bool,
}

/// Response from update aliases operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAliasesResponse {
    /// Whether the operation was acknowledged
    pub acknowledged: bool,
}

/// Response from refresh index operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshIndexResponse {
    /// Refresh details
    pub _shards: ShardResponse,
}

/// Shard response information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardResponse {
    /// Total number of shards
    pub total: u32,
    /// Number of successful shards
    pub successful: u32,
    /// Number of failed shards
    pub failed: u32,
    /// Array of failures if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Value>>,
}

/// Response from an index creation operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateIndexResponse {
    /// Whether the request was acknowledged
    pub acknowledged: bool,

    /// Whether all shards responded to the request
    pub shards_acknowledged: bool,

    /// The name of the index that was created
    #[serde(rename = "index")]
    pub index_name: String,
}
