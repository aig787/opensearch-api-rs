//! Search-related data types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response for a search operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse<T: Default = serde_json::Value> {
    /// Time taken to execute the search in milliseconds
    pub took: i64,

    /// Whether the search timed out
    pub timed_out: bool,

    /// Information about shards involved in the search
    pub _shards: super::ShardStatistics,

    /// Information about hits (matching documents)
    pub hits: SearchHits<T>,

    /// Aggregation results (if aggregations were requested)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<HashMap<String, Aggregation>>,

    /// Suggestion results (if suggestions were requested)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest: Option<HashMap<String, Vec<Suggestion>>>,

    /// Profile information (if profiling was requested)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<serde_json::Value>,

    /// Scroll ID (if scroll was requested)
    #[serde(rename = "_scroll_id", default, skip_serializing_if = "Option::is_none")]
    pub scroll_id: Option<String>,
}

/// Information about search hits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHits<T: Default = serde_json::Value> {
    /// Total number of matching hits
    pub total: TotalHits,

    /// Maximum score among the hits
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,

    /// List of hits (matching documents)
    pub hits: Vec<SearchHit<T>>,
}

/// Total number of hits information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalHits {
    /// Total number of hits
    pub value: u64,

    /// Relation of the value to the actual total ("eq" or "gte")
    pub relation: TotalHitsRelation,
}

/// Relation of the reported total hits to the actual total
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TotalHitsRelation {
    /// Exact total
    #[serde(rename = "eq")]
    Equal,

    /// Total is greater than or equal to the reported value
    #[serde(rename = "gte")]
    GreaterThanOrEqual,

    /// Total is less than or equal to the reported value
    #[serde(rename = "lte")]
    LessThanOrEqual,
}

/// Aggregation results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Aggregation {
    /// Bucket aggregation with sub-buckets
    Buckets {
        /// Doc count
        #[serde(skip_serializing_if = "Option::is_none")]
        doc_count_error_upper_bound: Option<i64>,

        /// Sum of other doc counts
        #[serde(skip_serializing_if = "Option::is_none")]
        sum_other_doc_count: Option<i64>,

        /// Buckets in the aggregation
        buckets: Vec<Bucket>,
    },

    /// Single bucket aggregation
    SingleBucket {
        /// Doc count
        doc_count: i64,

        /// Sub-aggregations
        #[serde(flatten)]
        aggregations: HashMap<String, Aggregation>,
    },

    /// Metric aggregation result
    Metric(serde_json::Value),

    /// Other aggregation types
    Other(serde_json::Value),
}

/// Bucket in a bucket aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    /// Bucket key
    pub key: serde_json::Value,

    /// Bucket key as text (for some aggregations)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_as_string: Option<String>,

    /// Document count in the bucket
    pub doc_count: i64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, Aggregation>,
}

/// Highlighting options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightOptions {
    /// Fields to highlight
    pub fields: HashMap<String, HighlightField>,

    /// Type of highlighter to use
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<HighlighterType>,

    /// Text to use as pre-tag
    #[serde(rename = "pre_tags", skip_serializing_if = "Option::is_none")]
    pub pre_tags: Option<Vec<String>>,

    /// Text to use as post-tag
    #[serde(rename = "post_tags", skip_serializing_if = "Option::is_none")]
    pub post_tags: Option<Vec<String>>,

    /// Whether to highlight empty fields
    #[serde(rename = "require_field_match", skip_serializing_if = "Option::is_none")]
    pub require_field_match: Option<bool>,

    /// Number of characters to return around each highlight
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_size: Option<i32>,

    /// Number of fragments to return
    #[serde(rename = "number_of_fragments", skip_serializing_if = "Option::is_none")]
    pub number_of_fragments: Option<i32>,

    /// Order of the highlighted fragments
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,

    /// Encoder to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder: Option<String>,
}

/// Highlighter types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HighlighterType {
    /// Plain highlighter
    Plain,

    /// Post-based highlighter (default)
    Unified,

    /// Fast vector highlighter
    Fvh,
}

/// Highlight field configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HighlightField {
    /// Empty configuration (use defaults)
    Empty(HashMap<String, serde_json::Value>),

    /// Specific highlight configuration
    Config {
        /// Type of highlighter to use
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        type_: Option<HighlighterType>,

        /// Number of characters to return around each highlight
        #[serde(skip_serializing_if = "Option::is_none")]
        fragment_size: Option<i32>,

        /// Number of fragments to return
        #[serde(rename = "number_of_fragments", skip_serializing_if = "Option::is_none")]
        number_of_fragments: Option<i32>,

        /// How to break fragments
        #[serde(rename = "fragment_offset", skip_serializing_if = "Option::is_none")]
        fragment_offset: Option<i32>,

        /// Whether to combine matches on multiple fields
        #[serde(rename = "matched_fields", skip_serializing_if = "Option::is_none")]
        matched_fields: Option<Vec<String>>,

        /// Override global pre-tags
        #[serde(rename = "pre_tags", skip_serializing_if = "Option::is_none")]
        pre_tags: Option<Vec<String>>,

        /// Override global post-tags
        #[serde(rename = "post_tags", skip_serializing_if = "Option::is_none")]
        post_tags: Option<Vec<String>>,

        /// Custom highlight query
        #[serde(skip_serializing_if = "Option::is_none")]
        highlight_query: Option<serde_json::Value>,
    },
}

/// Individual search hit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHit<T = serde_json::Value> {
    /// Index where the document is stored
    #[serde(rename = "_index")]
    pub index: String,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document score (relevance)
    #[serde(rename = "_score", default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,

    /// Document source data
    #[serde(rename = "_source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<T>,

    /// Requested fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, Vec<serde_json::Value>>>,

    /// Highlight results
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<HashMap<String, Vec<String>>>,

    /// Inner hits results
    #[serde(rename = "inner_hits", default, skip_serializing_if = "Option::is_none")]
    pub inner_hits: Option<HashMap<String, InnerHitsResult>>,

    /// Sort values used for this hit
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<serde_json::Value>>,
}

/// Inner hits result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerHitsResult {
    /// Hits within the inner hits
    pub hits: SearchHits,
}

/// Suggestion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    /// Text of the suggestion
    pub text: String,

    /// Offset of the suggestion in the original text
    pub offset: u64,

    /// Length of the suggestion in the original text
    pub length: u64,

    /// Options for the suggestion
    pub options: Vec<SuggestionOption>,
}

/// Option within a suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionOption {
    /// Text of the suggestion option
    pub text: String,

    /// Score of the suggestion option
    pub score: f64,

    /// Whether this is a collated result
    #[serde(default)]
    pub collate_match: Option<bool>,
}

/// Search-after parameters for pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SearchAfter(pub Vec<serde_json::Value>);

/// Point in time ID for search
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PointInTimeId(pub String);
