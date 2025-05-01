//! Query-related data types for OpenSearch

use crate::script::Script;
use crate::GeoPoint;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents query types in OpenSearch Query DSL
///
/// This is the main entry point for creating queries to be used with the OpenSearch API.
/// Each variant represents a different query type that can be used individually or combined.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use opensearch_api::{MatchQuery, Query};
///
/// let mut field_map = HashMap::new();
/// field_map.insert("title".to_string(), "document title".to_string().into());
///
/// let match_query = MatchQuery { field: field_map };
/// let query = Query::Match(match_query);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Query {
    /// Match all query to match all documents
    MatchAll(MatchAllQuery),
    /// Match none query to match no documents
    MatchNone(MatchNoneQuery),
    /// Match query for full-text search
    Match(MatchQuery),
    /// Term query for exact matching
    Term(TermQuery),
    /// Range query for range comparisons
    Range(RangeQuery),
    /// Boolean query for combining queries
    Bool(BoolQuery),
    /// Exists query to check if a field exists
    Exists(ExistsQuery),
    /// Query string query for advanced search syntax
    QueryString(QueryStringQuery),
    /// Wildcard query for pattern matching
    Wildcard(WildcardQuery),
    /// Prefix query for prefix matching
    Prefix(PrefixQuery),
    /// Generic query structure for other query types
    Generic(HashMap<String, serde_json::Value>),
}

impl Default for Query {
    fn default() -> Self {
        Self::MatchAll(MatchAllQuery::default())
    }
}

impl Query {
    /// Helper method to create a match_all query
    pub fn match_all(boost: Option<f64>) -> Self {
        Self::MatchAll(MatchAllQuery { boost })
    }

    /// Helper method to create a match_none query
    pub fn match_none() -> Self {
        Self::MatchNone(MatchNoneQuery {})
    }

    /// Helper method to create a match query
    pub fn match_query(field: &str, value: &str) -> Self {
        let mut field_map = HashMap::new();
        field_map.insert(
            field.to_string(),
            MatchQueryParams::Simple(value.to_string()),
        );

        Self::Match(MatchQuery { field: field_map })
    }

    /// Helper method to create a term query
    pub fn term_query(field: &str, value: serde_json::Value) -> Self {
        let mut field_map = HashMap::new();
        field_map.insert(field.to_string(), TermQueryParams::Simple(value));

        Self::Term(TermQuery { field: field_map })
    }

    /// Helper method to create a bool query
    pub fn bool_query() -> crate::types::builder::BoolQueryBuilder {
        crate::types::builder::BoolQuery::new()
    }
}

// BoolQueryBuilder implementation moved to builder.rs

/// Match all query to match all documents
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MatchAllQuery {
    /// Optional boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Match none query to match no documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchNoneQuery {}

/// Match query for full-text search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchQuery {
    /// Field to query
    #[serde(flatten)]
    pub field: HashMap<String, MatchQueryParams>,
}

/// Parameters for match query
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchQueryParams {
    /// Simple query with just a value
    Simple(String),
    /// Advanced query with additional parameters
    Advanced {
        /// Query text
        query: String,
        /// Operator (AND/OR)
        #[serde(skip_serializing_if = "Option::is_none")]
        operator: Option<Operator>,
        /// Analyzer to use
        #[serde(skip_serializing_if = "Option::is_none")]
        analyzer: Option<String>,
        /// Minimum should match specification
        #[serde(
            rename = "minimum_should_match",
            skip_serializing_if = "Option::is_none"
        )]
        minimum_should_match: Option<MinimumShouldMatch>,
        /// Fuzziness parameter
        #[serde(skip_serializing_if = "Option::is_none")]
        fuzziness: Option<Fuzziness>,
        /// Prefix length for fuzziness
        #[serde(rename = "prefix_length", skip_serializing_if = "Option::is_none")]
        prefix_length: Option<u32>,
        /// Maximum expansions for fuzziness
        #[serde(rename = "max_expansions", skip_serializing_if = "Option::is_none")]
        max_expansions: Option<u32>,
        /// Boost value
        #[serde(skip_serializing_if = "Option::is_none")]
        boost: Option<f64>,
    },
}

/// Term query for exact matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermQuery {
    /// Field to query
    #[serde(flatten)]
    pub field: HashMap<String, TermQueryParams>,
}

/// Parameters for term query
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TermQueryParams {
    /// Simple query with just a value
    Simple(serde_json::Value),
    /// Advanced query with additional parameters
    Advanced {
        /// Term value
        value: serde_json::Value,
        /// Boost value
        #[serde(skip_serializing_if = "Option::is_none")]
        boost: Option<f64>,
        /// Case insensitive flag
        #[serde(rename = "case_insensitive", skip_serializing_if = "Option::is_none")]
        case_insensitive: Option<bool>,
    },
}

/// Range query for range comparisons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeQuery {
    /// Field to query
    #[serde(flatten)]
    pub field: HashMap<String, RangeQueryParams>,
}

/// Parameters for range query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeQueryParams {
    /// Greater than
    #[serde(rename = "gt", skip_serializing_if = "Option::is_none")]
    pub greater_than: Option<serde_json::Value>,
    /// Greater than or equal to
    #[serde(rename = "gte", skip_serializing_if = "Option::is_none")]
    pub greater_than_or_equal_to: Option<serde_json::Value>,
    /// Less than
    #[serde(rename = "lt", skip_serializing_if = "Option::is_none")]
    pub less_than: Option<serde_json::Value>,
    /// Less than or equal to
    #[serde(rename = "lte", skip_serializing_if = "Option::is_none")]
    pub less_than_or_equal_to: Option<serde_json::Value>,
    /// Format for date values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Time zone for date values
    #[serde(rename = "time_zone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Relation for ranges (INTERSECTS, CONTAINS, WITHIN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<RangeRelation>,
    /// Boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Relation type for range queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RangeRelation {
    /// Ranges intersect
    Intersects,
    /// Range contains
    Contains,
    /// Range is within
    Within,
}

/// Boolean query for combining queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoolQuery {
    /// Queries that must match (AND)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must: Option<Vec<Query>>,
    /// Queries that must not match (NOT)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_not: Option<Vec<Query>>,
    /// Queries that should match (OR)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should: Option<Vec<Query>>,
    /// Queries that must match in a filter context (no scoring)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<Query>>,
    /// Minimum number of should clauses that must match
    #[serde(
        rename = "minimum_should_match",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_should_match: Option<MinimumShouldMatch>,
    /// Boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Exists query to check if a field exists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistsQuery {
    /// Field to check
    pub field: String,
    /// Boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Query string query with advanced query syntax
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryStringQuery {
    /// Query string
    pub query: String,
    /// Default field to search if not specified in the query
    #[serde(rename = "default_field", skip_serializing_if = "Option::is_none")]
    pub default_field: Option<String>,
    /// List of fields to search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// Default operator (AND/OR)
    #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
    pub default_operator: Option<Operator>,
    /// Analyzer to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    /// Whether to analyze wildcard terms
    #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
    pub analyze_wildcard: Option<bool>,
    /// Whether to lowercase expanded terms
    #[serde(
        rename = "lowercase_expanded_terms",
        skip_serializing_if = "Option::is_none"
    )]
    pub lowercase_expanded_terms: Option<bool>,
    /// Whether to enable position increments in result
    #[serde(
        rename = "enable_position_increments",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_position_increments: Option<bool>,
    /// Fuzzy max expansions
    #[serde(
        rename = "fuzzy_max_expansions",
        skip_serializing_if = "Option::is_none"
    )]
    pub fuzzy_max_expansions: Option<i32>,
    /// Fuzziness parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuzziness: Option<Fuzziness>,
    /// Fuzzy prefix length
    #[serde(
        rename = "fuzzy_prefix_length",
        skip_serializing_if = "Option::is_none"
    )]
    pub fuzzy_prefix_length: Option<i32>,
    /// Fuzzy rewrite method
    #[serde(rename = "fuzzy_rewrite", skip_serializing_if = "Option::is_none")]
    pub fuzzy_rewrite: Option<String>,
    /// Phrase slop
    #[serde(rename = "phrase_slop", skip_serializing_if = "Option::is_none")]
    pub phrase_slop: Option<i32>,
    /// Boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Whether to enable auto generate phrase queries
    #[serde(
        rename = "auto_generate_phrase_queries",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_generate_phrase_queries: Option<bool>,
    /// Allow leading wildcard flag
    #[serde(
        rename = "allow_leading_wildcard",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_leading_wildcard: Option<bool>,
    /// Maximum number of terms that can be created by wildcard or prefix expansion
    #[serde(
        rename = "max_determinized_states",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_determinized_states: Option<i32>,
    /// Minimum should match parameter
    #[serde(
        rename = "minimum_should_match",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_should_match: Option<MinimumShouldMatch>,
    /// Lenient flag to ignore format based failures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    /// Time zone for date values
    #[serde(rename = "time_zone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// How scores from different queries are combined
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<QueryStringType>,
}

/// Query string types for score combination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryStringType {
    /// Best fields scoring
    BestFields,
    /// Most fields scoring
    MostFields,
    /// Cross fields scoring
    CrossFields,
    /// Phrase scoring
    Phrase,
    /// Phrase prefix scoring
    PhrasePrefix,
    /// Boolean scoring
    Boolean,
}

/// Wildcard query for pattern matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WildcardQuery {
    /// Field to query
    #[serde(flatten)]
    pub field: HashMap<String, WildcardQueryParams>,
}

/// Parameters for wildcard query
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WildcardQueryParams {
    /// Simple query with just the pattern
    Simple(String),
    /// Advanced query with additional parameters
    Advanced {
        /// Wildcard pattern
        value: String,
        /// Boost value
        #[serde(skip_serializing_if = "Option::is_none")]
        boost: Option<f64>,
        /// Case insensitive flag
        #[serde(rename = "case_insensitive", skip_serializing_if = "Option::is_none")]
        case_insensitive: Option<bool>,
        /// Rewrite method
        #[serde(skip_serializing_if = "Option::is_none")]
        rewrite: Option<String>,
    },
}

/// Prefix query for prefix matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefixQuery {
    /// Field to query
    #[serde(flatten)]
    pub field: HashMap<String, PrefixQueryParams>,
}

/// Parameters for prefix query
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrefixQueryParams {
    /// Simple query with just the prefix
    Simple(String),
    /// Advanced query with additional parameters
    Advanced {
        /// Prefix value
        value: String,
        /// Boost value
        #[serde(skip_serializing_if = "Option::is_none")]
        boost: Option<f64>,
        /// Rewrite method
        #[serde(skip_serializing_if = "Option::is_none")]
        rewrite: Option<String>,
        /// Case insensitive flag
        #[serde(rename = "case_insensitive", skip_serializing_if = "Option::is_none")]
        case_insensitive: Option<bool>,
    },
}

/// Fuzziness parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Fuzziness {
    /// Auto fuzziness
    Auto(String),
    /// Specific edit distance
    Distance(i32),
}

/// Minimum should match specification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MinimumShouldMatch {
    /// Integer value (absolute number)
    Absolute(i32),
    /// String value (percentage or combination)
    Percentage(String),
}

/// Geo bounding box query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoBoundingBoxQuery {
    /// Top left corner of the bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_left: Option<GeoPoint>,

    /// Bottom right corner of the bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_right: Option<GeoPoint>,

    /// Top right corner of the bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_right: Option<GeoPoint>,

    /// Bottom left corner of the bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom_left: Option<GeoPoint>,

    /// Whether the corners can be outside the dateline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// How to validate the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,

    /// Whether to ignore unmapped fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<bool>,

    /// Boost value for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Geo polygon query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPolygonQuery {
    /// List of points that form the polygon
    pub points: Vec<GeoPoint>,

    /// How to validate the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,

    /// Whether to ignore unmapped fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<bool>,

    /// Boost value for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Geo shape query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoShapeQuery {
    /// Shape to query
    #[serde(flatten)]
    pub shape: GeoShape,

    /// Relation between query shape and indexed shapes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,

    /// Whether to ignore unmapped fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<bool>,

    /// Boost value for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Geo shape representation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeoShape {
    /// Reference to an indexed shape
    IndexedShape {
        /// Index containing the shape
        index: String,

        /// ID of the document containing the shape
        id: String,

        /// Path to the shape in the document
        path: String,
    },

    /// GeoJSON shape
    GeoJson(GeoJsonShape),
}

/// GeoJSON shape types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum GeoJsonShape {
    /// Point (single coordinate)
    Point {
        /// Coordinates [lon, lat]
        coordinates: [f64; 2],
    },

    /// LineString (array of coordinates)
    LineString {
        /// Coordinates [[lon1, lat1], [lon2, lat2], ...]
        coordinates: Vec<[f64; 2]>,
    },

    /// Polygon (array of linear rings)
    Polygon {
        /// Coordinates [[[lon1, lat1], [lon2, lat2], ...], ...]
        /// First array is exterior ring, others are holes
        coordinates: Vec<Vec<[f64; 2]>>,
    },

    /// MultiPoint (array of points)
    MultiPoint {
        /// Coordinates [[lon1, lat1], [lon2, lat2], ...]
        coordinates: Vec<[f64; 2]>,
    },

    /// MultiLineString (array of line strings)
    MultiLineString {
        /// Coordinates [[[lon1, lat1], [lon2, lat2], ...], ...]
        coordinates: Vec<Vec<[f64; 2]>>,
    },

    /// MultiPolygon (array of polygons)
    MultiPolygon {
        /// Coordinates [[[[lon1, lat1], ...]], ...]
        coordinates: Vec<Vec<Vec<[f64; 2]>>>,
    },

    /// GeometryCollection (collection of geometries)
    GeometryCollection {
        /// Array of geometries
        geometries: Vec<GeoJsonShape>,
    },

    /// Envelope (bounding box)
    Envelope {
        /// Coordinates [[min_lon, max_lat], [max_lon, min_lat]]
        coordinates: [[f64; 2]; 2],
    },

    /// Circle (center and radius)
    Circle {
        /// Coordinates [lon, lat]
        coordinates: [f64; 2],

        /// Radius with units
        radius: String,
    },
}

/// Script query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptQuery {
    /// Script to execute
    pub script: Script,

    /// Boost value for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// More like this query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoreLikeThisQuery {
    /// Fields to compare
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,

    /// Input text to find similar documents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like: Option<Vec<Like>>,

    /// Text to find documents not similar to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlike: Option<Vec<Like>>,

    /// Minimum term frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_term_freq: Option<i32>,

    /// Maximum term frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_term_freq: Option<i32>,

    /// Minimum document frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_doc_freq: Option<i32>,

    /// Maximum document frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_doc_freq: Option<i32>,

    /// Minimum word length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_word_length: Option<i32>,

    /// Maximum word length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_word_length: Option<i32>,

    /// Stop words to filter out
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<Vec<String>>,

    /// Analyzer to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,

    /// Minimum should match specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_should_match: Option<MinimumShouldMatch>,

    /// Boost terms based on frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_terms: Option<f64>,

    /// Whether to include the input documents in the results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<bool>,

    /// Boost value for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Input for more like this query
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Like {
    /// Plain text
    Text(String),

    /// Document reference
    Document {
        /// Document ID
        #[serde(rename = "_id")]
        id: String,

        /// Document index
        #[serde(rename = "_index", skip_serializing_if = "Option::is_none")]
        index: Option<String>,

        /// Per-document field mappings
        #[serde(rename = "per_field_analyzer", skip_serializing_if = "Option::is_none")]
        per_field_analyzer: Option<HashMap<String, String>>,

        /// Document routing
        #[serde(rename = "routing", skip_serializing_if = "Option::is_none")]
        routing: Option<String>,
    },

    /// Artificial document
    Doc {
        /// Document fields
        doc: HashMap<String, serde_json::Value>,
    },
}

/// Percolate query to match stored queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PercolateQuery {
    /// Field containing the query
    pub field: String,

    /// Document to match against the indexed queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<HashMap<String, serde_json::Value>>,

    /// Documents to match against the indexed queries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<HashMap<String, serde_json::Value>>>,

    /// ID of an indexed document to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Index containing the indexed document to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,

    /// Type of the indexed document to use (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    /// Routing for the indexed document to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,

    /// Preference for the indexed document to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<String>,

    /// Version for the indexed document to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,

    /// Boost value for the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Operator type for match queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operator {
    /// OR operator (default)
    Or,

    /// AND operator
    And,
}
