//! Search-related data types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Response for a search operation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    #[serde(default)]
    pub aggregations: Option<HashMap<String, Aggregation>>,

    /// Suggestion results (if suggestions were requested)
    #[serde(default)]
    pub suggest: Option<HashMap<String, Vec<Suggestion>>>,

    /// Profile information (if profiling was requested)
    #[serde(default)]
    pub profile: Option<serde_json::Value>,

    /// Scroll ID (if scroll was requested)
    #[serde(rename = "_scroll_id", default)]
    pub scroll_id: Option<String>,
}

/// Information about search hits
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchHits<T: Default = serde_json::Value> {
    /// Total number of matching hits
    pub total: TotalHits,

    /// Maximum score among the hits
    #[serde(default)]
    pub max_score: Option<f64>,

    /// List of hits (matching documents)
    pub hits: Vec<SearchHit<T>>,
}

/// Total number of hits information
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HighlightOptions {
    /// Fields to highlight
    pub fields: HashMap<String, HighlightField>,

    /// Type of highlighter to use
    #[serde(rename = "type")]
    pub type_: Option<HighlighterType>,

    /// Text to use as pre-tag
    #[serde(rename = "pre_tags")]
    pub pre_tags: Option<Vec<String>>,

    /// Text to use as post-tag
    #[serde(rename = "post_tags")]
    pub post_tags: Option<Vec<String>>,

    /// Whether to highlight empty fields
    #[serde(rename = "require_field_match")]
    pub require_field_match: Option<bool>,

    /// Number of characters to return around each highlight
    pub fragment_size: Option<i32>,

    /// Number of fragments to return
    #[serde(rename = "number_of_fragments")]
    pub number_of_fragments: Option<i32>,

    /// Order of the highlighted fragments
    #[serde(rename = "order")]
    pub order: Option<String>,

    /// Encoder to use
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
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum HighlightField {
    /// Specific highlight configuration
    Config {
        /// Type of highlighter to use
        #[serde(rename = "type")]
        type_: Option<HighlighterType>,

        /// Number of characters to return around each highlight
        fragment_size: Option<i32>,

        /// Number of fragments to return
        #[serde(rename = "number_of_fragments")]
        number_of_fragments: Option<i32>,

        /// How to break fragments
        #[serde(rename = "fragment_offset")]
        fragment_offset: Option<i32>,

        /// Whether to combine matches on multiple fields
        #[serde(rename = "matched_fields")]
        matched_fields: Option<Vec<String>>,

        /// Override global pre-tags
        #[serde(rename = "pre_tags")]
        pre_tags: Option<Vec<String>>,

        /// Override global post-tags
        #[serde(rename = "post_tags")]
        post_tags: Option<Vec<String>>,

        /// Custom highlight query
        highlight_query: Option<serde_json::Value>,
    },
    /// Empty configuration (use defaults)
    Empty(HashMap<String, serde_json::Value>),

}

/// Individual search hit
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchHit<T = serde_json::Value> {
    /// Index where the document is stored
    #[serde(rename = "_index")]
    pub index: String,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document score (relevance)
    #[serde(rename = "_score", default)]
    pub score: Option<f64>,

    /// Document source data
    #[serde(rename = "_source", default)]
    pub source: Option<T>,

    /// Requested fields
    #[serde(default)]
    pub fields: Option<HashMap<String, Vec<serde_json::Value>>>,

    /// Highlight results
    #[serde(default)]
    pub highlight: Option<HashMap<String, Vec<String>>>,

    /// Inner hits results
    #[serde(rename = "inner_hits", default)]
    pub inner_hits: Option<HashMap<String, InnerHitsResult>>,

    /// Sort values used for this hit
    #[serde(default)]
    pub sort: Option<Vec<serde_json::Value>>,
}

/// Inner hits result
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InnerHitsResult {
    /// Hits within the inner hits
    pub hits: SearchHits,
}

/// Suggestion result
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct SearchAfter(pub Vec<serde_json::Value>);

/// Point in time ID for search
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct PointInTimeId(pub String);
