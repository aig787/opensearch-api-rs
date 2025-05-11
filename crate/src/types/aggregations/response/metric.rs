//! Metric aggregation response types for OpenSearch

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::types::aggregations::StdDeviationBounds;



/// Stats aggregation response - includes both basic and extended stats fields
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatsAggregationResponse {
    /// Count of values
    pub count: i64,

    /// Minimum value
    pub min: Option<f64>,

    /// Maximum value
    pub max: Option<f64>,

    /// Average value
    pub avg: Option<f64>,

    /// Sum of values
    pub sum: Option<f64>,

    /// Sum of squares (extended stats)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_of_squares: Option<f64>,

    /// Variance (extended stats)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variance: Option<f64>,

    /// Standard deviation (extended stats)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_deviation: Option<f64>,

    /// Standard deviation bounds (extended stats)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_deviation_bounds: Option<StdDeviationBounds>,

    /// Min value as string (if string_stats is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_as_string: Option<String>,

    /// Max value as string (if string_stats is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_as_string: Option<String>,

    /// Avg value as string (if string_stats is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_as_string: Option<String>,

    /// Sum value as string (if string_stats is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum_as_string: Option<String>,
}

/// Percentile-based aggregation response (for percentiles and percentile ranks)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PercentileAggregationResponse {
    /// Map of percentile/value pairs
    pub values: HashMap<String, Option<f64>>,

    /// String representation of values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_as_string: Option<HashMap<String, String>>,
}

/// Geo Bounds aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoBoundsAggregationResponse {
    /// Bounding box coordinates
    pub bounds: GeoBounds,
}

/// Geo Bounds coordinates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoBounds {
    /// Top left coordinates
    pub top_left: GeoPoint,

    /// Bottom right coordinates
    pub bottom_right: GeoPoint,
}

/// Geo point coordinates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoPoint {
    /// Latitude
    pub lat: f64,

    /// Longitude
    pub lon: f64,
}

/// Geo Centroid aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoCentroidAggregationResponse {
    /// Count of points
    pub count: i64,

    /// Centroid coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<GeoPoint>,
}

/// Matrix Stats aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatrixStatsAggregationResponse {
    /// Fields statistics
    pub fields: Vec<MatrixStatsField>,
}

/// Matrix stats for a specific field
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatrixStatsField {
    /// Field name
    pub name: String,

    /// Count of values
    pub count: i64,

    /// Mean value
    pub mean: f64,

    /// Variance
    pub variance: f64,

    /// Skewness
    pub skewness: f64,

    /// Kurtosis
    pub kurtosis: f64,

    /// Covariance matrix
    pub covariance: HashMap<String, f64>,

    /// Correlation matrix
    pub correlation: HashMap<String, f64>,
}

/// Top Hits aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopHitsAggregationResponse {
    /// Total number of hits
    pub total: TopHitsTotal,

    /// Maximum score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,

    /// Hit documents
    pub hits: Vec<TopHit>,
}

/// Top hits total information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopHitsTotal {
    /// Value (count)
    pub value: i64,

    /// Relation ("eq" for accurate, "gte" for lower bound)
    pub relation: String,
}

/// Individual hit in top hits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopHit {
    /// Document index
    #[serde(rename = "_index")]
    pub index: String,

    /// Document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// Document score
    #[serde(rename = "_score", skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,

    /// Document source
    #[serde(rename = "_source", skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,

    /// Document fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, Vec<serde_json::Value>>>,
}


/// String Stats aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StringStatsAggregationResponse {
    /// Count of values
    pub count: i64,

    /// Min length
    pub min_length: i64,

    /// Max length
    pub max_length: i64,

    /// Average length
    pub avg_length: f64,

    /// Entropy
    pub entropy: f64,

    /// Distribution of characters (if requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<HashMap<String, f64>>,
}

/// Boxplot aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoxplotAggregationResponse {
    /// Minimum value - always present in a boxplot
    pub min: f64,

    /// Maximum value - always present in a boxplot
    pub max: f64,

    /// First quartile (25th percentile) - always present in a boxplot
    pub q1: f64,

    /// Median (50th percentile) - always present in a boxplot
    pub q2: f64,

    /// Third quartile (75th percentile) - always present in a boxplot
    pub q3: f64,
}
