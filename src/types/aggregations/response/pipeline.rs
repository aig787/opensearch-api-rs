//! Pipeline aggregation response types for OpenSearch

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Value pipeline aggregation response (for min_bucket, max_bucket, avg_bucket, sum_bucket)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValuePipelineAggregationResponse {
    /// The calculated pipeline metric value
    pub value: f64,

    /// Value as string (for formatted output)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
    
    /// Optional metadata about the calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// Stats bucket aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatsBucketAggregationResponse {
    /// Count of buckets
    pub count: i64,

    /// Minimum value across buckets
    pub min: Option<f64>,

    /// Maximum value across buckets
    pub max: Option<f64>,

    /// Average value across buckets
    pub avg: Option<f64>,

    /// Sum of values across buckets
    pub sum: Option<f64>,
    
    /// Optional metadata about the calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// Extended Stats bucket aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExtendedStatsBucketAggregationResponse {
    /// Count of buckets
    pub count: i64,

    /// Minimum value across buckets
    pub min: Option<f64>,

    /// Maximum value across buckets
    pub max: Option<f64>,

    /// Average value across buckets
    pub avg: Option<f64>,

    /// Sum of values across buckets
    pub sum: Option<f64>,

    /// Sum of squares
    pub sum_of_squares: Option<f64>,

    /// Variance
    pub variance: Option<f64>,

    /// Standard deviation
    pub std_deviation: Option<f64>,

    /// Standard deviation bounds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub std_deviation_bounds: Option<StdDeviationBounds>,
    
    /// Optional metadata about the calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// Standard deviation bounds
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StdDeviationBounds {
    /// Upper bound (mean + std_deviation)
    pub upper: f64,

    /// Lower bound (mean - std_deviation)
    pub lower: f64,
}

/// Percentiles bucket aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PercentilesBucketAggregationResponse {
    /// Map of percentile to values
    pub values: HashMap<String, Option<f64>>,
    
    /// Optional metadata about the calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// Moving average aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MovingAverageAggregationResponse {
    /// The calculated moving average value
    pub value: f64,
    
    /// Optional metadata about the calculation
    pub keys: Vec<String>,
}

/// Derivative aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DerivativeAggregationResponse {
    /// The calculated derivative value
    pub value: f64,
    
    /// The normalized value if a unit was specified
    pub normalized_value: f64,
    
    /// Optional metadata about the calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// Cumulative sum aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CumulativeSumAggregationResponse {
    /// The calculated cumulative sum value
    pub value: f64,
    
    /// Optional metadata about the calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// Serial differencing aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SerialDifferencingAggregationResponse {
    /// The calculated difference value
    pub value: f64,
    
    /// Optional metadata about the calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// Bucket script aggregation response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BucketScriptAggregationResponse {
    /// The calculated script value
    pub value: f64,
    
    /// Optional metadata about the calculation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

/// Bucket selector aggregation response - typically empty as it just filters buckets
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BucketSelectorAggregationResponse {
    /// Indicator that this is a bucket selector response
    pub is_bucket_selector: bool,
}
