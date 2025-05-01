use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::types::common::GeoPoint;
use crate::types::script::Script;
use crate::types::search::SortOrder;

/// Bucket script aggregation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketScriptAggregation {
    /// Paths to the buckets
    pub buckets_path: HashMap<String, String>,

    /// Script to execute
    pub script: Script,

    /// Gap policy (how to handle missing values)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<String>,

    /// Format for the output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// Bucket selector aggregation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketSelectorAggregation {
    /// Paths to the buckets
    pub buckets_path: HashMap<String, String>,

    /// Script to execute
    pub script: Script,

    /// Gap policy (how to handle missing values)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<String>,
}

/// Bucket sort aggregation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketSortAggregation {
    /// Sort criteria
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<HashMap<String, SortOrder>>>,

    /// Number of buckets to skip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<u32>,

    /// Maximum number of buckets to return
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    /// Gap policy (how to handle missing values)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<String>,
}

/// Serial differencing aggregation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialDifferencingAggregation {
    /// Path to the buckets
    pub buckets_path: String,

    /// Lag value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag: Option<u32>,

    /// Gap policy (how to handle missing values)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<String>,

    /// Format for the output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// Matrix stats aggregation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixStatsAggregation {
    /// Fields to analyze
    pub fields: Vec<String>,

    /// Mode for handling missing values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Missing values to use for fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing: Option<HashMap<String, f64>>,
}

/// Aggregation results for different aggregation types
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AggregationResult {
    /// Single value result
    SingleValue {
        /// The value
        value: f64,

        /// Value as string
        value_as_string: Option<String>,
    },

    /// Multi-value result
    MultiValue {
        /// Values map
        values: HashMap<String, f64>,
    },

    /// Buckets result
    Buckets {
        /// List of buckets
        buckets: Vec<Bucket>,
    },

    /// Keyed buckets result
    KeyedBuckets {
        /// Map of buckets
        buckets: HashMap<String, Bucket>,
    },

    /// Document hit for top hits
    Hit {
        /// Hits object
        hits: super::search::SearchHits<serde_json::Value>,
    },

    /// Stats result
    Stats {
        /// Count of values
        count: u64,

        /// Minimum value
        min: f64,

        /// Maximum value
        max: f64,

        /// Average value
        avg: f64,

        /// Sum of values
        sum: f64,

        /// Values as strings
        min_as_string: Option<String>,
        max_as_string: Option<String>,
        avg_as_string: Option<String>,
        sum_as_string: Option<String>,
    },

    /// Extended stats result
    ExtendedStats {
        /// Count of values
        count: u64,

        /// Minimum value
        min: f64,

        /// Maximum value
        max: f64,

        /// Average value
        avg: f64,

        /// Sum of values
        sum: f64,

        /// Sum of squares
        sum_of_squares: f64,

        /// Variance
        variance: f64,

        /// Standard deviation
        std_deviation: f64,

        /// Upper and lower bounds for std deviation
        std_deviation_bounds: StdDeviationBounds,

        /// Values as strings
        min_as_string: Option<String>,
        max_as_string: Option<String>,
        avg_as_string: Option<String>,
        sum_as_string: Option<String>,
        variance_as_string: Option<String>,
        std_deviation_as_string: Option<String>,
    },

    /// Percentiles result
    Percentiles {
        /// List of percentile values
        values: Vec<PercentileValue>,
    },

    /// String stats result
    StringStats {
        /// Count of values
        count: u64,

        /// Minimum length
        min_length: u64,

        /// Maximum length
        max_length: u64,

        /// Average length
        avg_length: f64,

        /// Entropy
        entropy: f64,

        /// Distribution of characters
        distribution: Option<HashMap<String, f64>>,
    },

    /// Geo bounds result
    GeoBounds {
        /// Top left corner
        top_left: GeoPoint,

        /// Bottom right corner
        bottom_right: GeoPoint,
    },

    /// Geo centroid result
    GeoCentroid {
        /// Centroid coordinates
        location: GeoPoint,

        /// Count of points
        count: u64,
    },

    /// Any other result type
    Other(serde_json::Value),
}

/// Standard deviation bounds
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdDeviationBounds {
    /// Upper bound
    pub upper: f64,

    /// Lower bound
    pub lower: f64,

    /// Upper bound as string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_as_string: Option<String>,

    /// Lower bound as string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_as_string: Option<String>,
}

/// Percentile value
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PercentileValue {
    /// Percentile key
    pub key: f64,

    /// Value for this percentile
    pub value: f64,

    /// Value as string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}


/// Bucket in a bucket aggregation
#[serde_with::skip_serializing_none]
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    /// Bucket key
    pub key: serde_json::Value,

    /// Key as string (for date histograms)
    pub key_as_string: Option<String>,

    /// Document count in this bucket
    pub doc_count: u64,

    /// From value for range buckets
    pub from: Option<serde_json::Value>,

    /// From value as string (for date ranges)
    pub from_as_string: Option<String>,

    /// To value for range buckets
    pub to: Option<serde_json::Value>,

    /// To value as string (for date ranges)
    pub to_as_string: Option<String>,

    /// Sub-aggregations
    #[serde(flatten)]
    pub aggregations: HashMap<String, AggregationResult>,
}
