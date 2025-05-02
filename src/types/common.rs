//! Common data types used across multiple OpenSearch API endpoints

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;

/// HTTP method types used in API operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
}

/// Time units used in OpenSearch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeUnit {
    D,
    H,
    M,
    S,
    Ms,
    Micros,
    Nanos,
}

/// Size units used in OpenSearch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SizeUnit {
    B,
    Kb,
    Mb,
    Gb,
    Tb,
    Pb,
}

/// Distance units used in geo queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DistanceUnit {
    Mi,
    Miles,
    Yd,
    Yards,
    Ft,
    Feet,
    In,
    Inch,
    Km,
    Kilometers,
    M,
    Meters,
    Cm,
    Centimeters,
    Mm,
    Millimeters,
    NauticMi,
    NauticalMiles,
}

/// Refresh policy for indexing operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RefreshPolicy {
    /// Wait until refresh has happened
    Wait,

    /// Do an immediate refresh
    #[serde(rename = "true")]
    True,

    /// Do not refresh
    #[serde(rename = "false")]
    False,
}

impl Display for RefreshPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefreshPolicy::Wait => write!(f, "wait_for"),
            RefreshPolicy::True => write!(f, "true"),
            RefreshPolicy::False => write!(f, "false"),
        }
    }
}

/// Version type for document operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
    /// Internal versioning
    Internal,

    /// External versioning
    External,

    /// External versioning with greater than semantics
    ExternalGte,

    /// Force version (deprecated)
    Force,
}

/// Operations that can be performed on documents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    /// Index operation
    Index,

    /// Create operation
    Create,

    /// Update operation
    Update,

    /// Delete operation
    Delete,
}

/// Write operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpType {
    /// Index operation (allows updates)
    Index,

    /// Create operation (fails if document exists)
    Create,
}
/// Sort order options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Health status of a cluster, index, or shard
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Green,
    Yellow,
    Red,
}

/// Statistics about an operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationStats {
    /// Total number of operations
    pub total: u64,

    /// Number of successful operations
    pub successful: u64,

    /// Number of failed operations
    pub failed: u64,

    /// Detailed information about failures
    #[serde(default)]
    pub failures: Vec<OperationFailure>,
}

/// Information about a failed operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationFailure {
    /// Index where the failure occurred
    pub index: Option<String>,

    /// Type of failure
    #[serde(rename = "type")]
    pub error_type: String,

    /// Reason for the failure
    pub reason: String,

    /// Status code of the failure
    pub status: Option<u16>,

    /// Shard where the failure occurred
    pub shard: Option<i32>,

    /// Node where the failure occurred
    pub node: Option<String>,
}

/// Response header metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Request took time in milliseconds
    pub took: i64,

    /// Whether the operation timed out
    pub timed_out: bool,
}

/// Generic response with shards information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShardsResponse {
    /// Information about the shards involved in the operation
    pub _shards: ShardStatistics,
}

/// Statistics about shards
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ShardStatistics {
    /// Total number of shards
    pub total: u32,

    /// Number of successful shards
    pub successful: u32,

    /// Number of failed shards
    pub failed: u32,

    /// Information about shard failures
    #[serde(default)]
    pub failures: Vec<ShardFailure>,
}

/// Information about a shard failure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShardFailure {
    /// Shard index
    pub shard: Option<i32>,

    /// Index name
    pub index: Option<String>,

    /// Node ID
    pub node: Option<String>,

    /// Reason for the failure
    pub reason: ShardFailureReason,
}

/// Reason for a shard failure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShardFailureReason {
    /// Type of error
    #[serde(rename = "type")]
    pub error_type: String,

    /// Reason message
    pub reason: String,

    /// Additional error details
    #[serde(default)]
    pub caused_by: Option<HashMap<String, serde_json::Value>>,
}

/// Geo point representation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoPoint {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
}

impl GeoPoint {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }
}

/// Options for expanding wildcard expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpandWildcards {
    Open,
    Closed,
    Hidden,
    None,
    All,
}

impl ToString for ExpandWildcards {
    fn to_string(&self) -> String {
        match self {
            ExpandWildcards::Open => "open".to_string(),
            ExpandWildcards::Closed => "closed".to_string(),
            ExpandWildcards::Hidden => "hidden".to_string(),
            ExpandWildcards::None => "none".to_string(),
            ExpandWildcards::All => "all".to_string(),
        }
    }
}
