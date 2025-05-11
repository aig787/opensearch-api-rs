//! Bucket aggregation response types for OpenSearch

use crate::types::aggregations::AggregationResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

/// Terms bucket aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TermsAggregationResponse {
    /// Upper bound of error on document counts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_count_error_upper_bound: Option<i64>,

    /// Sum of document counts not included in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_other_doc_count: Option<i64>,

    /// Buckets in the aggregation
    pub buckets: Vec<TermsBucket>,
}

/// Terms bucket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TermsBucket {
    /// Key for the bucket
    pub key: String,

    /// Key as string representation (for numeric keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_as_string: Option<String>,

    /// Document count in the bucket
    pub doc_count: i64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResponse>,
}

/// Enum to identify the type of bucket aggregation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BucketType {
    /// Filter aggregation
    Filter,
    /// Nested aggregation
    Nested,
    /// Reverse nested aggregation
    ReverseNested,
    /// Diversified sampler aggregation
    DiversifiedSampler,
    /// Sampler aggregation
    Sampler,
    /// Children aggregation
    Children,
    /// Parent aggregation
    Parent,
    /// Global aggregation
    Global,
    /// Missing aggregation
    Missing,
}

/// Common response for bucket-type aggregations with document count and sub-aggregations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BucketAggregationResponse {
    /// Document count in the bucket
    pub doc_count: i64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResponse>,
}

impl BucketAggregationResponse {
    /// Create a new bucket aggregation response with the specified document count
    pub fn new(doc_count: i64) -> Self {
        Self {
            doc_count,
            aggregations: HashMap::new(),
        }
    }
    
    /// Create a new bucket aggregation response with the specified document count and aggregations
    pub fn with_aggregations(doc_count: i64, aggregations: HashMap<String, AggregationResponse>) -> Self {
        Self {
            doc_count,
            aggregations,
        }
    }
}

impl From<i64> for BucketAggregationResponse {
    fn from(doc_count: i64) -> Self {
        Self::new(doc_count)
    }
}

/// Filters aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FiltersAggregationResponse {
    /// Buckets in the aggregation
    pub buckets: HashMap<String, BucketAggregationResponse>,
}

/// Range aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RangeAggregationResponse {
    /// Buckets in the aggregation
    pub buckets: Vec<RangeBucket>,
}

/// Range bucket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RangeBucket {
    /// From value (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,

    /// To value (exclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,

    /// Key for the bucket (if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Document count in the bucket
    pub doc_count: i64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResponse>,
}

/// Date range aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DateRangeAggregationResponse {
    /// Buckets in the aggregation
    pub buckets: Vec<DateRangeBucket>,
}

/// Date range bucket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DateRangeBucket {
    /// From value (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,

    /// To value (exclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,

    /// From value as string (if date format specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_as_string: Option<String>,

    /// To value as string (if date format specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_as_string: Option<String>,

    /// Key for the bucket (if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Document count in the bucket
    pub doc_count: i64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResponse>,
}

/// Histogram aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistogramAggregationResponse {
    /// Buckets in the aggregation
    pub buckets: Vec<HistogramBucket>,
}

/// Histogram bucket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistogramBucket {
    /// Key for the bucket (lower bound of interval for histograms, timestamp in milliseconds for date histograms)
    pub key: Value,

    /// Key as string representation (if string_format specified or date format specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_as_string: Option<String>,

    /// Document count in the bucket
    pub doc_count: i64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResponse>,
}

/// Adjacency matrix aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdjacencyMatrixAggregationResponse {
    /// Buckets in the aggregation
    pub buckets: HashMap<String, BucketAggregationResponse>,
}

/// Significant terms aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignificantTermsAggregationResponse {
    /// Background document count for significance calculation
    pub doc_count: i64,

    /// Buckets in the aggregation
    pub buckets: Vec<SignificantTermsBucket>,
}

/// Significant terms bucket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignificantTermsBucket {
    /// Key for the bucket
    pub key: serde_json::Value,

    /// Key as string representation (for numeric keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_as_string: Option<String>,

    /// Document count in the bucket
    pub doc_count: i64,

    /// Background document count
    pub bg_count: i64,

    /// Score indicating the statistical significance of the term
    pub score: f64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResponse>,
}

/// IP range aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IPRangeAggregationResponse {
    /// Buckets in the aggregation
    pub buckets: Vec<IPRangeBucket>,
}

/// IP range bucket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IPRangeBucket {
    /// From value (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// To value (exclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    /// CIDR mask (if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask: Option<String>,

    /// Key for the bucket (if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Document count in the bucket
    pub doc_count: i64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResponse>,
}

/// Geo distance aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoDistanceAggregationResponse {
    /// Buckets in the aggregation
    pub buckets: Vec<GeoDistanceBucket>,
}

/// Geo distance bucket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoDistanceBucket {
    /// From value (inclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,

    /// To value (exclusive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,

    /// Key for the bucket (if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Document count in the bucket
    pub doc_count: i64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResponse>,
}

/// Composite aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompositeAggregationResponse {
    /// Buckets in the aggregation
    pub buckets: Vec<CompositeBucket>,

    /// After key for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_key: Option<HashMap<String, serde_json::Value>>,
}

/// Composite bucket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompositeBucket {
    /// Key map for the bucket
    pub key: HashMap<String, serde_json::Value>,

    /// Document count in the bucket
    pub doc_count: i64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResponse>,
}

/// Geo grid aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoGridAggregationResponse {
    /// Buckets in the aggregation
    pub buckets: Vec<GeoGridBucket>,
}

/// Geo grid bucket
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoGridBucket {
    /// Key for the bucket (grid cell identifier)
    pub key: String,

    /// Document count in the bucket
    pub doc_count: i64,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResponse>,
}
