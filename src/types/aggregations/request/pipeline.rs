//! Pipeline aggregation request types for OpenSearch
//!
//! This module contains the various pipeline aggregation request types
//! that can be used with the OpenSearch API.

use crate::types::common::SortOrder;
use crate::types::script::Script;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Common parameters for all bucket-based pipeline aggregations
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BucketPathParams {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The gap policy to use for missing data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// Gap policy for handling missing data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapPolicy {
    /// Skip gaps
    Skip,
    /// Insert zeros for gaps
    InsertZeros,
    /// Keep the previous value for gaps
    KeepValues,
}

/// Average Bucket aggregation computes the average value of a specified metric in a sibling aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct AvgBucketAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,
}

impl AvgBucketAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> AvgBucketAggregationBuilder {
        AvgBucketAggregationBuilder::default()
    }
}

/// Sum Bucket aggregation computes the sum of a specified metric in a sibling aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct SumBucketAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,
}

impl SumBucketAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> SumBucketAggregationBuilder {
        SumBucketAggregationBuilder::default()
    }
}



/// Min Bucket aggregation computes the minimum value of a specified metric in a sibling aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct MinBucketAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,
}

impl MinBucketAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> MinBucketAggregationBuilder {
        MinBucketAggregationBuilder::default()
    }
}



/// Max Bucket aggregation computes the maximum value of a specified metric in a sibling aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct MaxBucketAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,
}

impl MaxBucketAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> MaxBucketAggregationBuilder {
        MaxBucketAggregationBuilder::default()
    }
}

/// Stats Bucket aggregation computes stats (min, max, sum, count, avg) over the values of a specified metric in a sibling aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct StatsBucketAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,
}

impl StatsBucketAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> StatsBucketAggregationBuilder {
        StatsBucketAggregationBuilder::default()
    }
}


/// Extended Stats Bucket aggregation computes extended stats over the values of a specified metric in a sibling aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct ExtendedStatsBucketAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,

    /// Sigma value for standard deviation bounds
    #[builder(default)]
    pub sigma: Option<f64>,
}

impl ExtendedStatsBucketAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> ExtendedStatsBucketAggregationBuilder {
        ExtendedStatsBucketAggregationBuilder::default()
    }
}

/// Percentiles Bucket aggregation computes percentiles over the values of a specified metric in a sibling aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct PercentilesBucketAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,

    /// Percentiles to calculate (e.g., [1, 5, 25, 50, 75, 95, 99])
    #[builder(default)]
    pub percents: Option<Vec<f64>>,

    /// Whether to use a keyed response format
    #[builder(default)]
    pub keyed: Option<bool>,
}

impl PercentilesBucketAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> PercentilesBucketAggregationBuilder {
        PercentilesBucketAggregationBuilder::default()
    }
}



/// Derivative aggregation calculates the derivative of a specified metric in a parent histogram (or date_histogram) aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct DerivativeAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,

    /// The unit to use for calculating the derivative
    #[builder(default)]
    pub unit: Option<String>,
}

impl DerivativeAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> DerivativeAggregationBuilder {
        DerivativeAggregationBuilder::default()
    }
}



/// Cumulative Sum aggregation calculates the cumulative sum of a specified metric in a parent histogram (or date_histogram) aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct CumulativeSumAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,
}

impl CumulativeSumAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> CumulativeSumAggregationBuilder {
        CumulativeSumAggregationBuilder::default()
    }
}

/// Moving Average aggregation calculates the moving average of a specified metric in a parent histogram (or date_histogram) aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct MovingAverageAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The model to use for the moving average
    #[builder(default)]
    pub model: Option<MovingAverageModel>,

    /// The window size for the moving average
    #[builder(default)]
    pub window: Option<i32>,

    /// Model-specific settings
    #[builder(default)]
    pub settings: Option<HashMap<String, serde_json::Value>>,

    /// Whether to predict the next points
    #[builder(default)]
    pub predict: Option<i32>,

    /// Minimum number of samples required before calculation
    #[builder(default)]
    pub minimize: Option<bool>,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,
}

/// Moving average model types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MovingAverageModel {
    /// Simple model (unweighted average)
    Simple,
    /// Linear model
    Linear,
    /// Exponentially weighted model
    Ewma,
    /// Holt linear model
    Holt,
    /// Holt-Winters model
    HoltWinters,
}

impl MovingAverageAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> MovingAverageAggregationBuilder {
        MovingAverageAggregationBuilder::default()
    }
}
/// Serial Differencing aggregation applies serial differencing to a specified metric in a parent histogram (or date_histogram) aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct SerialDifferencingAggregation {
    /// The path to the bucket(s) to use for the aggregation
    pub buckets_path: String,

    /// The lag for the differencing
    #[builder(default)]
    pub lag: Option<i32>,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,
}

impl SerialDifferencingAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> SerialDifferencingAggregationBuilder {
        SerialDifferencingAggregationBuilder::default()
    }
}


/// Bucket Script aggregation executes a script for each bucket in a parent histogram (or date_histogram) aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct BucketScriptAggregation {
    /// The paths to the bucket(s) to use for the aggregation
    pub buckets_path: HashMap<String, String>,

    /// The script to execute on the bucket data
    pub script: Script,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,

    /// The format to use for the output
    #[builder(default)]
    pub format: Option<String>,
}

impl BucketScriptAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> BucketScriptAggregationBuilder {
        BucketScriptAggregationBuilder::default()
    }
}

/// Bucket Selector aggregation executes a script for each bucket in a parent histogram (or date_histogram) aggregation
/// and determines whether the bucket should be kept in the response.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct BucketSelectorAggregation {
    /// The paths to the bucket(s) to use for the aggregation
    pub buckets_path: HashMap<String, String>,

    /// The script to execute on the bucket data
    pub script: Script,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,
}

impl BucketSelectorAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> BucketSelectorAggregationBuilder {
        BucketSelectorAggregationBuilder::default()
    }
}

/// Bucket Sort aggregation sorts the buckets of a parent histogram (or date_histogram) aggregation.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), build_fn(error = "crate::Error"))]
pub struct BucketSortAggregation {
    /// Sort order for the buckets
    #[builder(default)]
    pub sort: Option<Vec<HashMap<String, SortOrder>>>,

    /// Number of buckets to return
    #[builder(default)]
    pub size: Option<i32>,

    /// Number of buckets to skip
    #[builder(default)]
    pub from: Option<i32>,

    /// The gap policy to use for missing data
    #[builder(default)]
    pub gap_policy: Option<GapPolicy>,
}

impl BucketSortAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> BucketSortAggregationBuilder {
        BucketSortAggregationBuilder::default()
    }
}

