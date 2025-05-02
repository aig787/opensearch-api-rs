//! Metric aggregation request types for OpenSearch
//!
//! This module contains the various metric aggregation request types
//! that can be used with the OpenSearch API.

use crate::types::common::SortOrder;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Avg aggregation calculates the average of numeric values extracted from the aggregated documents.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct AvgAggregation {
    /// The field to calculate the average on
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,
}

impl AvgAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> AvgAggregationBuilder {
        AvgAggregationBuilder::default()
    }
}

/// Max aggregation calculates the maximum value of numeric values extracted from the aggregated documents.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct MaxAggregation {
    /// The field to calculate the maximum on
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,
}

impl MaxAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> MaxAggregationBuilder {
        MaxAggregationBuilder::default()
    }
}



/// Min aggregation calculates the minimum value of numeric values extracted from the aggregated documents.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct MinAggregation {
    /// The field to calculate the minimum on
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,
}

impl MinAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> MinAggregationBuilder {
        MinAggregationBuilder::default()
    }
}



/// Sum aggregation calculates the sum of numeric values extracted from the aggregated documents.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct SumAggregation {
    /// The field to calculate the sum on
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,
}

impl SumAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> SumAggregationBuilder {
        SumAggregationBuilder::default()
    }
}

/// Value Count aggregation counts the number of values that are extracted from the aggregated documents.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct ValueCountAggregation {
    /// The field to count values from
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,
}

impl ValueCountAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> ValueCountAggregationBuilder {
        ValueCountAggregationBuilder::default()
    }
}

/// Cardinality aggregation calculates the approximate count of distinct values in a field.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct CardinalityAggregation {
    /// The field to calculate the cardinality on
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,

    /// Precision threshold for the cardinality algorithm
    /// Higher values give more accurate results but use more memory
    #[builder(default)]
    pub precision_threshold: Option<i64>,

    /// Whether to use pre-computed hashes for the cardinality calculation
    #[builder(default)]
    pub rehash: Option<bool>,
}

impl CardinalityAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> CardinalityAggregationBuilder {
        CardinalityAggregationBuilder::default()
    }
}

/// Stats aggregation calculates statistics (min, max, sum, count, avg) over numeric values extracted from the aggregated documents.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct StatsAggregation {
    /// The field to calculate statistics on
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,
}

impl StatsAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> StatsAggregationBuilder {
        StatsAggregationBuilder::default()
    }
}

/// Extended Stats aggregation calculates extended statistics (including standard deviation, variance, etc.)
/// over numeric values extracted from the aggregated documents.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct ExtendedStatsAggregation {
    /// The field to calculate extended statistics on
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,

    /// Sigma value to calculate standard deviation bounds
    #[builder(default)]
    pub sigma: Option<f64>,
}

impl ExtendedStatsAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> ExtendedStatsAggregationBuilder {
        ExtendedStatsAggregationBuilder::default()
    }
}

/// Percentiles aggregation calculates percentile ranks over numeric values extracted from the aggregated documents.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct PercentilesAggregation {
    /// The field to calculate percentiles on
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,

    /// Percentiles to calculate (e.g., [1, 5, 25, 50, 75, 95, 99])
    #[builder(default)]
    pub percents: Option<Vec<f64>>,

    /// Whether to use a keyed response format
    #[builder(default)]
    pub keyed: Option<bool>,

    /// Settings for HDR Histogram algorithm if using it
    #[builder(default)]
    pub hdr: Option<HdrHistogramSettings>,

    /// Settings for TDigest algorithm if using it
    #[builder(default)]
    pub tdigest: Option<TDigestSettings>,
}

/// HDR Histogram settings for percentile calculations
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct HdrHistogramSettings {
    /// Number of significant digits for the histogram
    pub number_of_significant_value_digits: u8,
}

/// TDigest algorithm settings for percentile calculations
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct TDigestSettings {
    /// Compression factor for the TDigest algorithm
    pub compression: f64,
}

impl PercentilesAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> PercentilesAggregationBuilder {
        PercentilesAggregationBuilder::default()
    }
}

/// Percentile Ranks aggregation calculates the percentile rankings for the provided values
/// over numeric values extracted from the aggregated documents.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct PercentileRanksAggregation {
    /// The field to calculate percentile ranks on
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,

    /// Values for which to calculate percentile ranks
    pub values: Vec<f64>,

    /// Whether to use a keyed response format
    #[builder(default)]
    pub keyed: Option<bool>,

    /// Settings for HDR Histogram algorithm if using it
    #[builder(default)]
    pub hdr: Option<HdrHistogramSettings>,

    /// Settings for TDigest algorithm if using it
    #[builder(default)]
    pub tdigest: Option<TDigestSettings>,
}

impl PercentileRanksAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> PercentileRanksAggregationBuilder {
        PercentileRanksAggregationBuilder::default()
    }
}

/// Geo Bounds aggregation computes the geographic bounding box containing all geo values for a field.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct GeoBoundsAggregation {
    /// The field to calculate geo bounds on (must be a geo_point field)
    pub field: String,

    /// Whether to wrap the longitude to -180° to 180°
    #[builder(default)]
    pub wrap_longitude: Option<bool>,
}

impl GeoBoundsAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> GeoBoundsAggregationBuilder {
        GeoBoundsAggregationBuilder::default()
    }
}

/// Top Hits aggregation returns the top matching documents for each bucket in a bucket aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct TopHitsAggregation {
    /// Number of hits to return
    #[builder(default)]
    pub size: Option<i32>,

    /// From position to start returning hits
    #[builder(default)]
    pub from: Option<i32>,

    /// Specific fields to return
    #[builder(default)]
    pub _source: Option<SourceFilter>,

    /// Sort order for the results
    #[builder(default)]
    pub sort: Option<Vec<SortOption>>,

    /// Script fields to compute for each hit
    #[builder(default)]
    pub script_fields: Option<HashMap<String, ScriptField>>,
}

/// Configuration for source filtering in top hits aggregation
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceFilter {
    /// Whether to include the source
    Enabled(bool),
    /// List of fields to include
    Includes(Vec<String>),
    /// Configuration for includes and excludes
    IncludesExcludes {
        /// List of fields to include
        includes: Vec<String>,
        /// List of fields to exclude
        excludes: Vec<String>,
    },
}

/// Sort option for top hits aggregation
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SortOption {
    /// Sort by field
    Field(HashMap<String, SortOrder>),
    /// Sort by script
    Script {
        /// Script to compute sort values
        script: ScriptField,
        /// Type of the script result
        type_field: String,
        /// Sort order
        order: SortOrder,
    },
}

/// Script field configuration
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct ScriptField {
    /// The script source code
    pub source: String,

    /// The script language
    #[builder(default)]
    pub lang: Option<String>,

    /// Script parameters
    #[builder(default)]
    pub params: Option<HashMap<String, serde_json::Value>>,
}

impl TopHitsAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> TopHitsAggregationBuilder {
        TopHitsAggregationBuilder::default()
    }
}

/// Scripted Metric aggregation allows executing custom scripts for metric calculations.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct ScriptedMetricAggregation {
    /// Script executed at the start of aggregation
    pub init_script: Option<Script>,

    /// Script executed on each document
    pub map_script: Script,

    /// Script executed after all document processing is complete on each shard
    pub combine_script: Option<Script>,

    /// Script executed to combine results from each shard
    pub reduce_script: Option<Script>,

    /// Parameters passed to the scripts
    #[builder(default)]
    pub params: Option<HashMap<String, serde_json::Value>>,
}

/// Script definition
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Script {
    /// Inline script source
    Source(String),
    /// Complete script definition
    Object {
        /// Script source code
        source: String,
        /// Script language
        #[serde(skip_serializing_if = "Option::is_none")]
        lang: Option<String>,
        /// Script parameters
        #[serde(skip_serializing_if = "Option::is_none")]
        params: Option<HashMap<String, serde_json::Value>>,
    },
}

impl ScriptedMetricAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> ScriptedMetricAggregationBuilder {
        ScriptedMetricAggregationBuilder::default()
    }
}

/// Median Absolute Deviation aggregation computes the median absolute deviation of numeric values in a field.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct MedianAbsoluteDeviationAggregation {
    /// The field to calculate the median absolute deviation on
    pub field: String,

    /// The script to use for the aggregation
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,
}

impl MedianAbsoluteDeviationAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> MedianAbsoluteDeviationAggregationBuilder {
        MedianAbsoluteDeviationAggregationBuilder::default()
    }
}

/// Geo Centroid aggregation computes the weighted centroid of all geo points for a field.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct GeoCentroidAggregation {
    /// The field to calculate geo centroid on (must be a geo_point field)
    pub field: String,
}

impl GeoCentroidAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> GeoCentroidAggregationBuilder {
        GeoCentroidAggregationBuilder::default()
    }
}

/// Weighted Average aggregation computes the weighted average of numeric values from selected field.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct WeightedAvgAggregation {
    /// The field to use for values in the weighted average
    pub value: WeightedAvgValue,

    /// The field to use for weights in the weighted average
    pub weight: WeightedAvgValue,
}

/// Value or weight specification for weighted average aggregation
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct WeightedAvgValue {
    /// The field to use
    #[builder(default)]
    pub field: Option<String>,

    /// The script to use
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,
}

impl WeightedAvgAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> WeightedAvgAggregationBuilder {
        WeightedAvgAggregationBuilder::default()
    }
}

/// String Stats aggregation calculates statistics over string fields.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct StringStatsAggregation {
    /// The field to calculate string statistics on
    pub field: String,

    /// Whether to show distribution of characters
    #[builder(default)]
    pub show_distribution: Option<bool>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<String>,
}

impl StringStatsAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> StringStatsAggregationBuilder {
        StringStatsAggregationBuilder::default()
    }
}

/// Boxplot aggregation computes boxplot statistics over numeric values.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct BoxplotAggregation {
    /// The field to calculate boxplot statistics on
    pub field: String,

    /// The compression factor to use (higher compression means lower memory usage but potential precision loss)
    #[builder(default)]
    pub compression: Option<f64>,
}

impl BoxplotAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> BoxplotAggregationBuilder {
        BoxplotAggregationBuilder::default()
    }
}

/// Rate aggregation computes rate of occurrences in a specified time period.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct RateAggregation {
    /// The field containing the timestamps
    pub field: String,

    /// The time unit to calculate the rate in
    pub unit: RateUnit,

    /// Custom calendar interval
    #[builder(default)]
    pub calendar_interval: Option<String>,
}

/// Unit for rate calculation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RateUnit {
    /// Per second
    Second,
    /// Per minute
    Minute,
    /// Per hour
    Hour,
    /// Per day
    Day,
    /// Per week
    Week,
    /// Per month
    Month,
    /// Per year
    Year,
}

impl RateAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> RateAggregationBuilder {
        RateAggregationBuilder::default()
    }
}

/// Matrix Stats aggregation computes statistics for a set of fields, returning covariance and correlation matrices.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct MatrixStatsAggregation {
    /// The fields to calculate matrix statistics on
    pub fields: Vec<String>,

    /// Missing value to use when a document doesn't have a field
    #[builder(default)]
    pub missing: Option<HashMap<String, f64>>,

    /// The mode for handling missing values
    #[builder(default)]
    pub mode: Option<MatrixStatsMode>,
}

/// Mode for handling missing values in matrix stats aggregation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatrixStatsMode {
    /// Treat documents with any missing fields as if all fields were missing
    Complete,
    /// Process all documents, regardless of missing values
    Incomplete,
}

impl MatrixStatsAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> MatrixStatsAggregationBuilder {
        MatrixStatsAggregationBuilder::default()
    }
}
