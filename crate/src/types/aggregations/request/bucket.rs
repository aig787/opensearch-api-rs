//! Bucket aggregation request types for OpenSearch
//!
//! This module contains the various bucket aggregation request types
//! that can be used with the OpenSearch API.

use crate::types::aggregations::Aggregations;
use crate::types::common::SortOrder;
use crate::types::query::Query;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Terms aggregation creates a bucket for each unique term in a field.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct TermsAggregation {
    /// The field to get terms from
    pub field: String,

    /// Maximum number of buckets to return
    #[builder(default)]
    pub size: Option<i64>,

    /// Number of terms to request from each shard
    #[builder(default)]
    pub shard_size: Option<i64>,

    /// How to order the buckets
    #[builder(default)]
    pub order: Option<TermsOrder>,

    /// Minimum document count for a bucket to be returned
    #[builder(default)]
    pub min_doc_count: Option<i64>,

    /// Script to generate terms
    #[builder(default)]
    pub script: Option<String>,

    /// Include terms that match the provided patterns
    #[builder(default)]
    pub include: Option<TermsInclude>,

    /// Exclude terms that match the provided patterns
    #[builder(default)]
    pub exclude: Option<TermsExclude>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<serde_json::Value>,

    /// Sub-aggregations
    #[builder(default)]
    #[serde(skip)]
    pub aggs: Option<Aggregations>,
}

/// Order specification for terms aggregation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TermsOrder {
    /// Sort by a single criterion
    Simple(TermsOrderItem),
    /// Sort by multiple criteria
    Multiple(Vec<TermsOrderItem>),
}

/// Single ordering criterion for terms aggregation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TermsOrderItem {
    /// Field or aggregation to sort by
    pub key: String,
    /// Direction to sort
    pub order: SortOrder,
}

/// Include specification for terms aggregation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TermsInclude {
    /// Include terms matching a pattern
    Pattern(String),
    /// Include terms matching patterns
    Patterns(Vec<String>),
    /// Include terms in the provided list
    Values(Vec<serde_json::Value>),
    /// Include terms within a numeric range
    Range {
        /// Minimum value (inclusive)
        from: serde_json::Value,
        /// Maximum value (inclusive)
        to: serde_json::Value,
    },
}

/// Exclude specification for terms aggregation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TermsExclude {
    /// Exclude terms matching a pattern
    Pattern(String),
    /// Exclude terms matching patterns
    Patterns(Vec<String>),
    /// Exclude terms in the provided list
    Values(Vec<serde_json::Value>),
}

impl TermsAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> TermsAggregationBuilder {
        TermsAggregationBuilder::default()
    }
}

/// Filter aggregation creates a single bucket for documents that match a specified filter.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct FilterAggregation {
    /// The filter query
    pub filter: Query,

    /// Sub-aggregations
    #[builder(default)]
    pub aggs: Option<Aggregations>,
}

impl FilterAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> FilterAggregationBuilder {
        FilterAggregationBuilder::default()
    }
}

/// Filters aggregation creates multiple buckets based on different filters.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct FiltersAggregation {
    /// The filter queries, each creating a separate bucket
    pub filters: FiltersDefinition,

    /// Whether to create a bucket for documents that don't match any filter
    #[builder(default)]
    pub other_bucket: Option<bool>,

    /// Key for the other bucket if created
    #[builder(default)]
    pub other_bucket_key: Option<String>,

    /// Sub-aggregations
    #[builder(default)]
    pub aggs: Option<Aggregations>,
}

/// Filters definition - can be anonymous or named
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FiltersDefinition {
    /// Anonymous filters - buckets are labeled by their position
    Anonymous {
        /// List of filter queries
        filters: Vec<serde_json::Value>,
    },
    /// Named filters - buckets are labeled by their keys
    Named {
        /// Map of bucket key to filter query
        filters: HashMap<String, serde_json::Value>,
    },
}

impl FiltersAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> FiltersAggregationBuilder {
        FiltersAggregationBuilder::default()
    }
}
/// Range aggregation creates buckets based on predefined ranges.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct RangeAggregation {
    /// The field to apply ranges to
    pub field: String,

    /// Ranges to create buckets for
    pub ranges: Vec<RangeDefinition>,

    /// Whether to use a keyed response format
    #[builder(default)]
    pub keyed: Option<bool>,

    /// Script to calculate values
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,

    /// Sub-aggregations
    #[builder(default)]
    #[serde(skip)]
    pub aggs: Option<Aggregations>,
}

/// Range definition for a bucket
#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), default)]
pub struct RangeDefinition {
    /// Optional key for the range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Lower bound of the range (inclusive unless specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,

    /// Upper bound of the range (exclusive unless specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,
}

impl RangeDefinition {
    pub fn builder() -> RangeDefinitionBuilder {
        RangeDefinitionBuilder::default()
    }
}

impl RangeAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> RangeAggregationBuilder {
        RangeAggregationBuilder::default()
    }
}

/// Date Range aggregation creates buckets based on date ranges.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct DateRangeAggregation {
    /// The field to apply date ranges to
    pub field: String,

    /// Ranges to create buckets for
    pub ranges: Vec<DateRangeDefinition>,

    /// Format to use for parsing date strings
    #[builder(default)]
    pub format: Option<String>,

    /// Time zone to use for date calculations
    #[builder(default)]
    pub time_zone: Option<String>,

    /// Whether to use a keyed response format
    #[builder(default)]
    pub keyed: Option<bool>,

    /// Script to calculate values
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<String>,

    /// Sub-aggregations
    #[builder(default)]
    #[serde(skip)]
    pub aggs: Option<Aggregations>,
}

/// Date range definition for a bucket
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateRangeDefinition {
    /// Optional key for the range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Lower bound of the range (inclusive unless specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,

    /// Upper bound of the range (exclusive unless specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl DateRangeAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> DateRangeAggregationBuilder {
        DateRangeAggregationBuilder::default()
    }
}

/// Histogram aggregation buckets documents based on a specified interval.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct HistogramAggregation {
    /// The field to create histogram from
    pub field: String,

    /// Interval for the histogram buckets
    pub interval: f64,

    /// Whether to include empty buckets in the response
    #[builder(default)]
    pub min_doc_count: Option<i64>,

    /// Extended bounds to ensure buckets are created in a specific range
    #[builder(default)]
    pub extended_bounds: Option<HistogramBounds>,

    /// Hard bounds to limit the buckets created
    #[builder(default)]
    pub hard_bounds: Option<HistogramBounds>,

    /// Offset for the buckets
    #[builder(default)]
    pub offset: Option<f64>,

    /// Whether to use a keyed response format
    #[builder(default)]
    pub keyed: Option<bool>,

    /// Script to calculate values
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<f64>,

    /// Sub-aggregations
    #[builder(default)]
    #[serde(skip)]
    pub aggs: Option<Aggregations>,
}

/// Bounds specification for histogram aggregation
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistogramBounds {
    /// Minimum bound
    pub min: f64,
    /// Maximum bound
    pub max: f64,
}

impl HistogramAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> HistogramAggregationBuilder {
        HistogramAggregationBuilder::default()
    }
}

/// Date Histogram aggregation buckets documents based on a date/time interval.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct DateHistogramAggregation {
    /// The field to create date histogram from
    pub field: String,

    /// Calendar interval for the histogram buckets
    #[builder(default)]
    pub calendar_interval: Option<String>,

    /// Fixed interval for the histogram buckets
    #[builder(default)]
    pub fixed_interval: Option<String>,

    /// Interval for the histogram buckets (deprecated, use calendar_interval or fixed_interval)
    #[builder(default)]
    pub interval: Option<String>,

    /// Format to use for dates in response
    #[builder(default)]
    pub format: Option<String>,

    /// Time zone to use for date calculations
    #[builder(default)]
    pub time_zone: Option<String>,

    /// Offset for the buckets
    #[builder(default)]
    pub offset: Option<String>,

    /// Whether to include empty buckets in the response
    #[builder(default)]
    pub min_doc_count: Option<i64>,

    /// Extended bounds to ensure buckets are created in a specific date range
    #[builder(default)]
    pub extended_bounds: Option<DateHistogramBounds>,

    /// Hard bounds to limit the date buckets created
    #[builder(default)]
    pub hard_bounds: Option<DateHistogramBounds>,

    /// Whether to use a keyed response format
    #[builder(default)]
    pub keyed: Option<bool>,

    /// Script to calculate values
    #[builder(default)]
    pub script: Option<String>,

    /// Missing value to use when a document doesn't have the field
    #[builder(default)]
    pub missing: Option<String>,

    /// Sub-aggregations
    #[builder(default)]
    #[serde(skip)]
    pub aggs: Option<Aggregations>,
}

/// Bounds specification for date histogram aggregation
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateHistogramBounds {
    /// Minimum bound (date string)
    pub min: String,
    /// Maximum bound (date string)
    pub max: String,
}

impl DateHistogramAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> DateHistogramAggregationBuilder {
        DateHistogramAggregationBuilder::default()
    }
}

// Remove duplicate implementations as they're now defined for each struct

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
pub struct NestedPath {
    pub path: String,
}

/// Nested aggregation allows aggregating nested objects within a document.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct NestedAggregation {
    /// Path to the nested objects
    #[builder(setter(custom))]
    pub nested: NestedPath,
    pub aggs: Aggregations,
}

impl NestedAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> NestedAggregationBuilder {
        NestedAggregationBuilder::default()
    }
}

impl NestedAggregationBuilder {
    pub fn path(&mut self, path: impl Into<String>) -> &mut Self {
        self.nested = Some(NestedPath { path: path.into() });
        self
    }
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
pub struct ReverseNestedPath {
    /// Optional path to a specific parent level (defaults to the root)
    pub path: Option<String>,
}

/// Reverse nested aggregation allows to aggregate on parent docs from nested docs.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct ReverseNestedAggregation {
    /// Optional path to a specific parent level (defaults to the root)
    #[builder(setter(custom), default)]
    pub reverse_nested: ReverseNestedPath,
}

impl ReverseNestedAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> ReverseNestedAggregationBuilder {
        ReverseNestedAggregationBuilder::default()
    }
}

impl ReverseNestedAggregationBuilder {
    pub fn path(&mut self, path: impl Into<String>) -> &mut Self {
        self.reverse_nested = Some(ReverseNestedPath {
            path: Some(path.into()),
        });
        self
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
pub struct Sampler {
    pub shard_size: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct SampleAggregation {
    pub sampler: Sampler,
    #[builder(setter(strip_option), default)]
    pub aggs: Option<Aggregations>,
}

impl SampleAggregationBuilder {
    pub fn shard_size(&mut self, shard_size: i64) -> &mut Self {
        self.sampler = Some(Sampler {
            shard_size: Some(shard_size),
        });
        self
    }
}

/// Adjacency Matrix aggregation creates a matrix of intersecting filters.
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(
    pattern = "mutable",
    setter(into, strip_option),
    build_fn(error = "crate::Error")
)]
pub struct AdjacencyMatrixAggregation {
    /// Named filters defining the matrix dimensions
    pub filters: HashMap<String, serde_json::Value>,

    /// Sub-aggregations
    #[builder(default)]
    #[serde(skip)]
    pub aggs: Option<Aggregations>,
}

impl AdjacencyMatrixAggregation {
    /// Create a new builder for this aggregation
    pub fn builder() -> AdjacencyMatrixAggregationBuilder {
        AdjacencyMatrixAggregationBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::query::{MatchQuery, MatchQueryRule, TermQuery, TermQueryRule};
    use anyhow::Error;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_terms_aggregation_builder_simple() -> Result<(), Error> {
        let agg = TermsAggregation::builder()
            .field("category")
            .size(10)
            .build()?;

        assert_eq!(agg.field, "category");
        assert_eq!(agg.size, Some(10));

        let serialized = serde_json::to_value(&agg)?;
        let expected = json!({
            "field": "category",
            "size": 10
        });

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn test_terms_aggregation_builder_complete() -> Result<(), Error> {
        let order_item = TermsOrderItem {
            key: "_count".to_string(),
            order: SortOrder::Desc,
        };

        let include = TermsInclude::Patterns(vec!["elec.*".to_string(), "digi.*".to_string()]);
        let exclude = TermsExclude::Pattern("obsolete.*".to_string());

        let agg = TermsAggregation::builder()
            .field("category")
            .size(20)
            .shard_size(100)
            .order(TermsOrder::Simple(order_item))
            .min_doc_count(5)
            .script("doc['category'].value")
            .include(include)
            .exclude(exclude)
            .missing("N/A")
            .build()?;

        let serialized = serde_json::to_value(&agg)?;
        let expected = json!({
            "field": "category",
            "size": 20,
            "shard_size": 100,
            "order": {
                "key": "_count",
                "order": "desc"
            },
            "min_doc_count": 5,
            "script": "doc['category'].value",
            "include": ["elec.*", "digi.*"],
            "exclude": "obsolete.*",
            "missing": "N/A"
        });

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn test_terms_aggregation_builder_multiple_order() -> Result<(), Error> {
        let order_items = vec![
            TermsOrderItem {
                key: "_count".to_string(),
                order: SortOrder::Desc,
            },
            TermsOrderItem {
                key: "_key".to_string(),
                order: SortOrder::Asc,
            },
        ];

        let agg = TermsAggregation::builder()
            .field("category")
            .order(TermsOrder::Multiple(order_items))
            .build()?;

        let serialized = serde_json::to_value(&agg)?;
        let expected = json!({
            "field": "category",
            "order": [
                {
                    "key": "_count",
                    "order": "desc"
                },
                {
                    "key": "_key",
                    "order": "asc"
                }
            ]
        });

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn test_terms_include_variants() -> Result<(), Error> {
        // Test pattern variant
        let include_pattern = TermsInclude::Pattern("elec.*".to_string());
        let serialized = serde_json::to_value(&include_pattern)?;
        assert_eq!(serialized, json!("elec.*"));

        // Test patterns variant
        let include_patterns =
            TermsInclude::Patterns(vec!["elec.*".to_string(), "digi.*".to_string()]);
        let serialized = serde_json::to_value(&include_patterns)?;
        assert_eq!(serialized, json!(["elec.*", "digi.*"]));

        // Test values variant
        let include_values = TermsInclude::Values(vec![json!("electronics"), json!("digital")]);
        let serialized = serde_json::to_value(&include_values)?;
        assert_eq!(serialized, json!(["electronics", "digital"]));

        // Test range variant
        let include_range = TermsInclude::Range {
            from: json!(100),
            to: json!(200),
        };
        let serialized = serde_json::to_value(&include_range)?;
        assert_eq!(serialized, json!({"from": 100, "to": 200}));

        Ok(())
    }

    #[test]
    fn test_filter_aggregation_builder() -> Result<(), Error> {
        let match_query = MatchQuery::builder()
            .field(
                "category",
                MatchQueryRule::Simple("electronics".to_string()),
            )
            .build()?;

        let filter_agg = FilterAggregation::builder()
            .filter(Query::from(match_query))
            .build()?;

        let serialized = serde_json::to_value(&filter_agg)?;
        let expected = json!({
            "filter": {
                "match": {
                    "category": "electronics"
                }
            }
        });

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn test_filters_aggregation_builder_anonymous() -> Result<(), Error> {
        let match_query1 = Query::from(
            MatchQuery::builder()
                .field(
                    "category",
                    MatchQueryRule::Simple("electronics".to_string()),
                )
                .build()?,
        );

        let match_query2 = Query::from(
            MatchQuery::builder()
                .field("category", MatchQueryRule::Simple("clothing".to_string()))
                .build()?,
        );

        let filters_def = FiltersDefinition::Anonymous {
            filters: vec![json!(match_query1), json!(match_query2)],
        };

        let filters_agg = FiltersAggregation::builder()
            .filters(filters_def)
            .other_bucket(true)
            .other_bucket_key("other_categories")
            .build()?;

        let serialized = serde_json::to_value(&filters_agg)?;

        // Check for key properties in serialized output
        assert!(serialized.get("filters").is_some());
        assert_eq!(serialized["other_bucket"], json!(true));
        assert_eq!(serialized["other_bucket_key"], json!("other_categories"));

        Ok(())
    }

    #[test]
    fn test_filters_aggregation_builder_named() -> Result<(), Error> {
        let match_query1 = Query::from(
            MatchQuery::builder()
                .field(
                    "category",
                    MatchQueryRule::Simple("electronics".to_string()),
                )
                .build()?,
        );

        let match_query2 = Query::from(
            MatchQuery::builder()
                .field("category", MatchQueryRule::Simple("clothing".to_string()))
                .build()?,
        );

        let mut filters_map = HashMap::new();
        filters_map.insert("electronics".to_string(), json!(match_query1));
        filters_map.insert("clothing".to_string(), json!(match_query2));

        let filters_def = FiltersDefinition::Named {
            filters: filters_map,
        };

        let filters_agg = FiltersAggregation::builder().filters(filters_def).build()?;

        let serialized = serde_json::to_value(&filters_agg)?;

        // Check for filters property in serialized output
        assert!(serialized.get("filters").is_some());
        assert!(serialized.get("filters").unwrap().get("filters").is_some());

        Ok(())
    }

    #[test]
    fn test_range_aggregation_builder() -> Result<(), Error> {
        let range1 = RangeDefinition::builder().key("cheap").to(50.0).build()?;

        let range2 = RangeDefinition::builder()
            .key("moderate")
            .from(50.0)
            .to(100.0)
            .build()?;

        let range3 = RangeDefinition::builder()
            .key("expensive")
            .from(100.0)
            .build()?;

        let range_agg = RangeAggregation::builder()
            .field("price")
            .ranges(vec![range1, range2, range3])
            .keyed(true)
            .missing(0.0)
            .build()?;

        let serialized = serde_json::to_value(&range_agg)?;
        let expected = json!({
            "field": "price",
            "ranges": [
                {
                    "key": "cheap",
                    "to": 50.0
                },
                {
                    "key": "moderate",
                    "from": 50.0,
                    "to": 100.0
                },
                {
                    "key": "expensive",
                    "from": 100.0
                }
            ],
            "keyed": true,
            "missing": 0.0
        });

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn test_date_range_aggregation_builder() -> Result<(), Error> {
        let date_range_agg = DateRangeAggregation::builder()
            .field("created_date")
            .format("yyyy-MM-dd")
            .time_zone("UTC")
            .ranges(vec![
                DateRangeDefinition {
                    key: Some("old".to_string()),
                    from: None,
                    to: Some("2020-01-01".to_string()),
                },
                DateRangeDefinition {
                    key: Some("current".to_string()),
                    from: Some("2020-01-01".to_string()),
                    to: Some("2023-01-01".to_string()),
                },
                DateRangeDefinition {
                    key: Some("new".to_string()),
                    from: Some("2023-01-01".to_string()),
                    to: None,
                },
            ])
            .keyed(true)
            .missing("1970-01-01")
            .build()?;

        let serialized = serde_json::to_value(&date_range_agg)?;
        let expected = json!({
            "field": "created_date",
            "format": "yyyy-MM-dd",
            "time_zone": "UTC",
            "ranges": [
                {
                    "key": "old",
                    "to": "2020-01-01"
                },
                {
                    "key": "current",
                    "from": "2020-01-01",
                    "to": "2023-01-01"
                },
                {
                    "key": "new",
                    "from": "2023-01-01"
                }
            ],
            "keyed": true,
            "missing": "1970-01-01"
        });

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn test_histogram_aggregation_builder() -> Result<(), Error> {
        let hist_bounds = HistogramBounds {
            min: 0.0,
            max: 100.0,
        };

        let hard_bounds = HistogramBounds {
            min: 10.0,
            max: 90.0,
        };

        let hist_agg = HistogramAggregation::builder()
            .field("price")
            .interval(10.0)
            .min_doc_count(1)
            .extended_bounds(hist_bounds)
            .hard_bounds(hard_bounds)
            .offset(5.0)
            .keyed(true)
            .missing(0.0)
            .build()?;

        let serialized = serde_json::to_value(&hist_agg)?;
        let expected = json!({
            "field": "price",
            "interval": 10.0,
            "min_doc_count": 1,
            "extended_bounds": {
                "min": 0.0,
                "max": 100.0
            },
            "hard_bounds": {
                "min": 10.0,
                "max": 90.0
            },
            "offset": 5.0,
            "keyed": true,
            "missing": 0.0
        });

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn test_date_histogram_aggregation_builder() -> Result<(), Error> {
        let date_bounds = DateHistogramBounds {
            min: "2020-01-01".to_string(),
            max: "2023-01-01".to_string(),
        };

        let date_hist_agg = DateHistogramAggregation::builder()
            .field("created_date")
            .calendar_interval("month")
            .format("yyyy-MM-dd")
            .time_zone("UTC")
            .offset("+1d")
            .min_doc_count(1)
            .extended_bounds(date_bounds)
            .keyed(true)
            .missing("1970-01-01")
            .build()?;

        let serialized = serde_json::to_value(&date_hist_agg)?;
        let expected = json!({
            "field": "created_date",
            "calendar_interval": "month",
            "format": "yyyy-MM-dd",
            "time_zone": "UTC",
            "offset": "+1d",
            "min_doc_count": 1,
            "extended_bounds": {
                "min": "2020-01-01",
                "max": "2023-01-01"
            },
            "keyed": true,
            "missing": "1970-01-01"
        });

        assert_eq!(serialized, expected);
        Ok(())
    }

    #[test]
    fn test_nested_aggregation_builder() -> Result<(), Error> {
        // Create a sub-aggregation
        let terms_agg = TermsAggregation::builder()
            .field("reviews.rating")
            .size(5)
            .build()?;

        // Create map of sub-aggregations
        let mut aggs = HashMap::new();
        aggs.insert("ratings".to_string(), terms_agg);

        // Create aggregations
        let aggregations = Aggregations::builder().aggs(aggs).build()?;

        // Create nested aggregation
        let nested_agg = NestedAggregation::builder()
            .path("reviews")
            .aggs(aggregations)
            .build()?;

        let serialized = serde_json::to_value(&nested_agg)?;

        // Check structure of serialized output
        assert!(serialized.get("nested").is_some());
        assert_eq!(serialized["nested"]["path"], json!("reviews"));
        assert!(serialized.get("aggs").is_some());
        assert!(serialized["aggs"].get("ratings").is_some());

        Ok(())
    }

    #[test]
    fn test_multiple_nested_aggregation_builder() -> Result<(), Error> {
        // Create a sub-aggregation
        let terms_agg = TermsAggregation::builder()
            .field("reviews.rating.numeric")
            .size(5)
            .build()?;

        // Create map of sub-aggregations
        let mut aggs = HashMap::new();
        aggs.insert("ratings".to_string(), terms_agg);

        // Create aggregations
        let aggregations = Aggregations::builder().aggs(aggs).build()?;

        // Create inner level nested aggregation
        let inner_nested_agg = NestedAggregation::builder()
            .path("reviews.ratings")
            .aggs(aggregations)
            .build()?;

        // Create outer aggregations with inner nested agg
        let mut outer_aggs = HashMap::new();
        outer_aggs.insert("nested".to_string(), inner_nested_agg);
        let outer_aggregations = Aggregations::builder().aggs(outer_aggs).build()?;

        let nested = NestedAggregation::builder()
            .path("reviews.ratings.numeric")
            .aggs(outer_aggregations)
            .build()?;

        let serialized = serde_json::to_value(&nested)?;

        let expected = json!({
          "nested": {
            "path": "reviews.ratings.numeric"
          },
          "aggs": {
            "nested": {
              "nested": {
                "path": "reviews.ratings"
              },
              "aggs": {
                "ratings": {
                  "terms": {
                    "field": "reviews.rating.numeric",
                    "size": 5
                  }
                }
              }
            }
          }
        });
        assert_eq!(serialized, expected);

        Ok(())
    }

    #[test]
    fn test_reverse_nested_aggregation_builder() -> Result<(), Error> {
        // Simple reverse nested with no path
        let simple_reverse_nested = ReverseNestedAggregation::builder().build()?;

        let serialized = serde_json::to_value(&simple_reverse_nested)?;
        assert_eq!(serialized["reverse_nested"], json!({}));

        // Reverse nested with specified path
        let reverse_nested_with_path = ReverseNestedAggregation::builder()
            .path("product")
            .build()?;

        let serialized = serde_json::to_value(&reverse_nested_with_path)?;
        assert_eq!(serialized["reverse_nested"]["path"], json!("product"));

        Ok(())
    }

    #[test]
    fn test_adjacency_matrix_aggregation_builder() -> Result<(), Error> {
        let term_query1 = Query::from(
            TermQuery::builder()
                .field("category", TermQueryRule::value(json!("electronics")))
                .build()?,
        );

        let term_query2 = Query::from(
            TermQuery::builder()
                .field("price", TermQueryRule::value(json!(100)))
                .build()?,
        );

        let mut filters = HashMap::new();
        filters.insert("electronics".to_string(), json!(term_query1));
        filters.insert("expensive".to_string(), json!(term_query2));

        let adjacency_matrix = AdjacencyMatrixAggregation::builder()
            .filters(filters)
            .build()?;

        let serialized = serde_json::to_value(&adjacency_matrix)?;

        // Check structure of filters in serialized output
        assert!(serialized.get("filters").is_some());
        assert!(serialized["filters"].get("electronics").is_some());
        assert!(serialized["filters"].get("expensive").is_some());

        Ok(())
    }
}
