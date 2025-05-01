//! Common data types used across the OpenSearch API

pub mod search;
pub mod query;
pub mod aggregations;
pub mod document;
pub mod common;
pub mod script;
pub mod builder;

pub use aggregations::*;
pub use builder::QueryDsl;
pub use common::*;
pub use document::*;
pub use query::*;

/// Statistics about shards
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardStatistics {
    /// Total number of shards
    pub total: u32,

    /// Number of successful shards
    pub successful: u32,

    /// Number of failed shards
    pub failed: u32,

    /// Number of skipped shards
    #[serde(default)]
    pub skipped: u32,
}

/// Common parameters shared across API calls
#[derive(Debug, Clone, Default)]
pub struct CommonParameters {
    /// Request timeout in milliseconds
    pub timeout: Option<String>,

    /// Master timeout for the operation
    pub master_timeout: Option<String>,

    /// Pretty format the returned JSON response
    pub pretty: Option<bool>,

    /// Human-readable output for statistics
    pub human: Option<bool>,

    /// Whether specified concrete indices should be ignored when unavailable
    pub ignore_unavailable: Option<bool>,

    /// Whether to expand wildcard expressions to concrete indices
    pub expand_wildcards: Option<ExpandWildcards>,
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
