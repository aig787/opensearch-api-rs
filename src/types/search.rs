//! Search-related data types

use crate::types::query::Query;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::types::common::ShardStatistics;

/// Response for a search operation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse<T: Default = serde_json::Value> {
    /// Time taken to execute the search in milliseconds
    pub took: i64,

    /// Whether the search timed out
    pub timed_out: bool,

    /// Information about shards involved in the search
    pub _shards: ShardStatistics,

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

impl<T> SearchHit<T> {
    pub fn source_ref_required(&self) -> &T {
        self.source.as_ref().expect("source is required")
    }
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

/// Sort specification for search results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Sort {
    /// Sort by field name with default order (ascending)
    Field(String),

    /// Sort by field with specific order
    FieldWithOrder {
        /// Field name to sort by
        #[serde(flatten)]
        field: HashMap<String, SortOptions>,
    },

    /// Sort by script
    Script(ScriptSort),

    /// Sort by _score
    Score { _score: SortOrder },

    /// Sort by _doc (natural order)
    Doc { _doc: SortOrder },
}

/// Script sort specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptSort {
    /// The script to execute for sorting
    #[serde(rename = "_script")]
    pub script: ScriptSortOptions,
}

/// Script sort options
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptSortOptions {
    /// Script to execute
    pub script: crate::types::script::Script,

    /// Type of the sort values produced by the script
    #[serde(rename = "type")]
    pub type_: ScriptSortType,

    /// Sort order
    pub order: Option<SortOrder>,

    /// Mode for sorting on array values
    pub mode: Option<SortMode>,

    /// Behavior when a document is missing the field
    pub missing: Option<serde_json::Value>,

    /// Whether the sort should contain nested objects
    pub nested: Option<NestedSortOptions>,
}

/// Type of script sort values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScriptSortType {
    /// String type
    String,
    /// Numeric type
    Number,
    /// Version type (for semantic version sorting)
    Version,
}

/// Sort options for a field
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SortOptions {
    /// Sort order
    pub order: Option<SortOrder>,

    /// Mode for sorting on array values
    pub mode: Option<SortMode>,

    /// Behavior when a document is missing the field
    pub missing: Option<serde_json::Value>,

    /// Format for the sort values
    pub format: Option<String>,

    /// Whether to use unmapped fields
    pub unmapped_type: Option<String>,

    /// Whether the sort should contain nested objects
    pub nested: Option<NestedSortOptions>,
}

/// Options for nested sorting
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedSortOptions {
    /// Path to the nested object
    pub path: String,

    /// Filter for the nested objects to include in sorting
    pub filter: Option<Query>,

    /// Maximum number of children to consider per parent
    pub max_children: Option<u32>,
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Ascending order (default)
    Asc,
    /// Descending order
    Desc,
}

/// Mode for sorting arrays
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortMode {
    /// Use minimum value
    Min,
    /// Use maximum value
    Max,
    /// Use sum of values
    Sum,
    /// Use average of values
    Avg,
    /// Use median value
    Median,
}

/// Source filtering options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SourceFilter {
    /// Enable or disable source retrieval
    Enabled(bool),

    /// Include specified fields
    Include(Vec<String>),

    /// Include and exclude patterns
    Filter {
        /// Patterns of fields to include
        #[serde(skip_serializing_if = "Option::is_none")]
        includes: Option<Vec<String>>,

        /// Patterns of fields to exclude
        #[serde(skip_serializing_if = "Option::is_none")]
        excludes: Option<Vec<String>>,
    },
}

/// Script field definition
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptField {
    /// Script to execute for the field
    pub script: crate::types::script::Script,

    /// Whether to ignore failures during script execution
    pub ignore_failure: Option<bool>,
}

/// Search-after parameters for pagination
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct SearchAfter(pub Vec<serde_json::Value>);

/// Point in time ID for search
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct PointInTimeId(pub String);

/// Response from a scroll request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollResponse<T>
where
    T: Default,
{
    /// Scroll ID for subsequent requests
    #[serde(rename = "_scroll_id")]
    pub scroll_id: String,

    /// Document hits
    pub hits: Hits<T>,

    /// Total time spent on the request in milliseconds
    pub took: u64,

    /// Whether the request timed out
    pub timed_out: bool,

    /// Shard information
    #[serde(rename = "_shards")]
    pub shards: ShardInfo,
}

/// Hits contains the search results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hits<T>
where
    T: Default,
{
    /// Total hits information
    pub total: TotalHits,

    /// Maximum score across all hits
    #[serde(default)]
    pub max_score: Option<f64>,

    /// Array of search hits
    pub hits: Vec<SearchHit<T>>,
}

/// Information about shards involved in a search operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShardInfo {
    /// Total number of shards
    pub total: u32,

    /// Number of successful shards
    pub successful: u32,

    /// Number of skipped shards
    #[serde(default)]
    pub skipped: Option<u32>,

    /// Number of failed shards
    pub failed: u32,

    /// Details about failures in shards, if any
    #[serde(default)]
    pub failures: Option<Vec<ShardFailure>>,
}

/// Details about a shard failure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShardFailure {
    /// Shard index
    pub shard: u32,

    /// Index name
    pub index: String,

    /// Node ID
    pub node: String,

    /// Reason for the failure
    pub reason: serde_json::Value,
}

/// Response from a clear_scroll request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearScrollResponse {
    /// Whether the request was successful
    pub succeeded: bool,

    /// Number of scroll contexts cleared
    pub num_freed: u64,
}

/// Item for multi-search request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MSearchItem {
    /// Header information (index, search_type, etc.)
    pub header: MSearchHeader,

    /// Search request body
    pub body: serde_json::Value,
}

/// Header for an msearch request item
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), default)]
pub struct MSearchHeader {
    /// Index to search in
    pub index: Option<String>,

    /// Search type
    pub search_type: Option<SearchType>,

    /// Preference for shards
    pub preference: Option<String>,

    /// Routing value
    pub routing: Option<String>,
}

impl MSearchHeader {
    pub fn builder() -> MSearchHeaderBuilder {
        MSearchHeaderBuilder::default()
    }
}

/// Type of search execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    /// Query then fetch (default)
    QueryThenFetch,
    /// Distributed frequency for DFS query then fetch
    DfsQueryThenFetch,
}

/// Multi-search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MSearchResponse<T: Default = serde_json::Value> {
    /// Information about the overall response
    pub took: i64,

    /// Responses for each search request
    pub responses: Vec<SearchResponse<T>>,
}

/// Point-in-time response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointInTimeResponse {
    /// Point-in-time ID
    pub id: String,
}

/// Delete point-in-time response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePointInTimeResponse {
    /// Whether the delete was successful
    pub succeeded: bool,

    /// Number of search contexts that were deleted
    pub num_freed: u64,
}
