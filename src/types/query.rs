//! Query-related data types for OpenSearch

use crate::types::common::GeoPoint;
use derive_builder::Builder;
use derive_more::From;
use serde::{Deserialize, Serialize};
use serde_literals::lit_str;
use serde_with::{serde_as, KeyValueMap};
use std::collections::HashMap;

/// Represents query types in OpenSearch Query DSL
///
/// This is the main entry point for creating queries to be used with the OpenSearch API.
/// Each variant represents a different query type that can be used individually or combined.
///
/// # Examples
///
/// ```
/// use opensearch_api::types::query::{MatchQuery, MatchQueryRule};
/// let query = MatchQuery::builder()
///     .field(
///         "test",
///         MatchQueryRule::simple("value")
///     )
///     .build()
///     .unwrap()
///     .into_query();
/// ```
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, From)]
#[serde(untagged)]
pub enum Query {
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
    /// Match all query to match all documents
    MatchAll(MatchAllQuery),
    /// Match none query to match no documents
    MatchNone(MatchNoneQuery),
    MatchPhrase(MatchPhraseQuery),
    MatchPhrasePrefix(MatchPhrasePrefixQuery),
    MultiMatch(MultiMatchQuery),
    Ids(IdsQuery),
    Fuzzy(FuzzyQuery),
    Regexp(RegexpQuery),
    Terms(TermsQuery),
    TermsSet(TermsSetQuery),
    Nested(NestedQuery),
    HasChild(HasChildQuery),
    HasParent(HasParentQuery),
    ParentId(ParentIdQuery),
    Script(ScriptQuery),
    MoreLikeThis(MoreLikeThisQuery),
    GeoDistance(GeoDistanceQuery),
    GeoBoundingBox(GeoBoundingBoxQuery),
    GeoPolygon(GeoPolygonQuery),
    GeoShape(GeoShapeQuery),
    /// Generic query structure for other query types
    Generic(HashMap<String, serde_json::Value>),
}

impl Default for Query {
    fn default() -> Self {
        Self::MatchAll(MatchAllQuery::simple())
    }
}

impl Query {
    pub fn json(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::to_value(self)
    }
}

/// Match all query to match all documents
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct MatchAllQuery {
    pub match_all: MatchAllQueryRule,
}

impl MatchAllQuery {
    pub fn builder() -> MatchAllQueryRuleBuilder {
        MatchAllQueryRuleBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::MatchAll(self)
    }

    pub fn simple() -> Self {
        Self {
            match_all: MatchAllQueryRule {
                ..Default::default()
            },
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
#[builder(build_fn(name = "build_rule"))]
pub struct MatchAllQueryRule {
    /// Optional boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub boost: Option<f64>,
}

impl MatchAllQueryRuleBuilder {
    pub fn build(&self) -> Result<MatchAllQuery, MatchAllQueryRuleBuilderError> {
        Ok(MatchAllQuery {
            match_all: self.build_rule()?,
        })
    }
}

/// Match none query to match no documents
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct MatchNoneQuery {
    pub match_none: MatchNoneQueryRule,
}

impl MatchNoneQuery {
    pub fn builder() -> MatchNoneQueryRuleBuilder {
        MatchNoneQueryRuleBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::MatchNone(self)
    }

    pub fn simple() -> Self {
        Self {
            match_none: MatchNoneQueryRule {
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder, Default)]
#[builder(pattern = "mutable", setter(into, strip_option))]
#[builder(build_fn(name = "build_rule"))]
pub struct MatchNoneQueryRule {}

impl MatchNoneQueryRuleBuilder {
    pub fn build(&self) -> Result<MatchNoneQuery, MatchNoneQueryRuleBuilderError> {
        Ok(MatchNoneQuery {
            match_none: self.build_rule()?,
        })
    }
}

/// Match query for full-text search
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct MatchQuery {
    /// Field to query
    #[serde(rename = "match")]
    #[builder(default)]
    pub match_: HashMap<String, MatchQueryRule>,
}

impl MatchQuery {
    /// Create a new builder for MatchQuery
    pub fn builder() -> MatchQueryBuilder {
        MatchQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Match(self)
    }
}

impl MatchQueryBuilder {
    /// Add a field to the match query
    pub fn field<S: Into<String>, V: Into<MatchQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let match_ = self.match_.get_or_insert_with(HashMap::new);
        match_.insert(field.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, From)]
#[serde(untagged)]
pub enum MatchQueryRule {
    /// Simple query with just a value
    Simple(String),
    /// Advanced query with additional parameters
    Advanced(MatchQueryRuleAdvanced),
}

impl MatchQueryRule {
    pub fn simple(value: impl Into<String>) -> Self {
        Self::Simple(value.into())
    }

    pub fn advanced() -> MatchQueryRuleAdvancedBuilder {
        MatchQueryRuleAdvancedBuilder::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct MatchQueryRuleAdvanced {
    /// Query text
    pub query: String,
    /// Operator (AND/OR)
    #[builder(default)]
    pub operator: Option<Operator>,
    /// Analyzer to use
    #[builder(default)]
    pub analyzer: Option<String>,
    /// Minimum should match specification
    #[serde(rename = "minimum_should_match")]
    #[builder(default)]
    pub minimum_should_match: Option<MinimumShouldMatch>,
    /// Fuzziness parameter
    #[builder(default)]
    pub fuzziness: Option<Fuzziness>,
    /// Prefix length for fuzziness
    #[serde(rename = "prefix_length")]
    #[builder(default)]
    pub prefix_length: Option<i32>,
    /// Maximum expansions for fuzziness
    #[serde(rename = "max_expansions")]
    #[builder(default)]
    pub max_expansions: Option<i32>,
    /// Boost value
    #[builder(default)]
    pub boost: Option<f64>,
    /// Whether to create a match phrase query for multi-term synonyms
    #[serde(rename = "auto_generate_synonyms_phrase_query")]
    #[builder(default)]
    pub auto_generate_synonyms_phrase_query: Option<bool>,
    /// Whether to enable position increments
    #[serde(rename = "enable_position_increments")]
    #[builder(default)]
    pub enable_position_increments: Option<bool>,
    /// Fuzzy rewrite method
    #[serde(rename = "fuzzy_rewrite")]
    #[builder(default)]
    pub fuzzy_rewrite: Option<String>,
    /// Whether to include transpositions for fuzziness
    #[serde(rename = "fuzzy_transpositions")]
    #[builder(default)]
    pub fuzzy_transpositions: Option<bool>,
    /// Whether to ignore data type mismatches
    #[builder(default)]
    pub lenient: Option<bool>,
    /// How to handle queries with only stop words
    #[serde(rename = "zero_terms_query")]
    #[builder(default)]
    pub zero_terms_query: Option<ZeroTermsQuery>,
}

impl MatchQueryRuleAdvanced {
    pub fn builder() -> MatchQueryRuleAdvancedBuilder {
        MatchQueryRuleAdvancedBuilder::default()
    }
}

/// How to handle queries with only stop words
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ZeroTermsQuery {
    /// Match no documents
    None,
    /// Match all documents
    All,
}

/// Term query for exact matching
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct TermQuery {
    /// Field to query
    #[serde(rename = "term")]
    pub term: HashMap<String, TermQueryRule>,
}

impl TermQuery {
    /// Create a new builder for TermQuery
    pub fn builder() -> TermQueryBuilder {
        TermQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Term(self)
    }
}

impl TermQueryBuilder {
    /// Add a field to the term query
    pub fn field<S: Into<String>, V: Into<TermQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let term = self.term.get_or_insert_with(HashMap::new);
        term.insert(field.into(), value.into());
        self
    }
}

/// Parameters for term query
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct TermQueryRule {
    /// Terms to match
    pub value: serde_json::Value,
    /// Case insensitive flag
    #[serde(rename = "case_insensitive", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub case_insensitive: Option<bool>,
    /// Boost factor for this query
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub boost: Option<f64>,
}

impl TermQueryRule {
    pub fn builder() -> TermQueryRuleBuilder {
        TermQueryRuleBuilder::default()
    }

    pub fn value(value: impl Into<serde_json::Value>) -> TermQueryRule {
        TermQueryRule {
            value: value.into(),
            ..Default::default()
        }
    }
}

/// Range query for range comparisons
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct RangeQuery {
    #[builder(default)]
    pub range: HashMap<String, RangeQueryRule>,
}

impl RangeQuery {
    /// Create a new builder for RangeQuery
    pub fn builder() -> RangeQueryBuilder {
        RangeQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Range(self)
    }
}

impl RangeQueryBuilder {
    /// Add a field to the range query
    pub fn field<S: Into<String>, V: Into<RangeQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let range = self.range.get_or_insert_with(HashMap::new);
        range.insert(field.into(), value.into());
        self
    }
}

/// Parameters for range query
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct RangeQueryRule {
    /// Greater than
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gt: Option<serde_json::Value>,
    /// Greater than or equal to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub gte: Option<serde_json::Value>,
    /// Less than
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lt: Option<serde_json::Value>,
    /// Less than or equal to
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lte: Option<serde_json::Value>,
    /// Format for date values
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub format: Option<String>,
    /// Time zone for date values
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_zone: Option<String>,
    /// Relation for ranges (INTERSECTS, CONTAINS, WITHIN)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub relation: Option<RangeRelation>,
    /// Boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub boost: Option<f64>,
}

impl RangeQueryRule {
    pub fn builder() -> RangeQueryRuleBuilder {
        RangeQueryRuleBuilder::default()
    }

    pub fn gt(value: impl Into<serde_json::Value>) -> Self {
        Self {
            gt: Some(value.into()),
            ..Default::default()
        }
    }

    pub fn gte(value: impl Into<serde_json::Value>) -> Self {
        Self {
            gte: Some(value.into()),
            ..Default::default()
        }
    }

    pub fn lt(value: impl Into<serde_json::Value>) -> Self {
        Self {
            lt: Some(value.into()),
            ..Default::default()
        }
    }

    pub fn lte(value: impl Into<serde_json::Value>) -> Self {
        Self {
            lte: Some(value.into()),
            ..Default::default()
        }
    }
}

/// Relation type for range queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RangeRelation {
    /// Ranges intersect
    Intersects,
    /// Range contains
    Contains,
    /// Range is within
    Within,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct BoolQuery {
    pub(crate) bool: BoolQueryRule,
}

impl BoolQuery {
    /// Create a new builder for BoolQuery
    pub fn builder() -> BoolQueryRuleBuilder {
        BoolQueryRuleBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Bool(self)
    }
}

/// Boolean query for combining queries
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
#[builder(build_fn(name = "build_rule"))]
pub struct BoolQueryRule {
    /// Queries that must match (AND)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub must: Option<Vec<Query>>,
    /// Queries that must not match (NOT)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub must_not: Option<Vec<Query>>,
    /// Queries that should match (OR)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub should: Option<Vec<Query>>,
    /// Queries that must match in a filter context (no scoring)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub filter: Option<Vec<Query>>,
    /// Minimum number of should clauses that must match
    #[serde(
        rename = "minimum_should_match",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub minimum_should_match: Option<MinimumShouldMatch>,
    /// Boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub boost: Option<f64>,
}

impl BoolQueryRuleBuilder {
    pub fn build(&self) -> Result<BoolQuery, BoolQueryRuleBuilderError> {
        Ok(BoolQuery {
            bool: self.build_rule()?,
        })
    }
}

/// Exists query to check if a field exists
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct ExistsQuery {
    pub exists: ExistsQueryRule,
}

impl ExistsQuery {
    /// Create a new builder for ExistsQuery
    pub fn builder() -> ExistsQueryRuleBuilder {
        ExistsQueryRuleBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Exists(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
#[builder(build_fn(name = "build_params"))]
pub struct ExistsQueryRule {
    /// Field to check
    pub field: String,
    /// Boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub boost: Option<f64>,
}

impl ExistsQueryRuleBuilder {
    pub fn build(&self) -> Result<ExistsQuery, ExistsQueryRuleBuilderError> {
        Ok(ExistsQuery {
            exists: self.build_params()?,
        })
    }
}

/// Query string query with advanced query syntax
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct QueryStringQuery {
    pub query_string: QueryStringQueryRule,
}

impl QueryStringQuery {
    /// Create a new builder for QueryStringQuery
    pub fn builder() -> QueryStringQueryRuleBuilder {
        QueryStringQueryRuleBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::QueryString(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
#[builder(build_fn(name = "build_params"))]
pub struct QueryStringQueryRule {
    /// Query string
    pub query: String,
    /// Default field to search if not specified in the query
    #[serde(rename = "default_field", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub default_field: Option<String>,
    /// List of fields to search
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fields: Option<Vec<String>>,
    /// Default operator (AND/OR)
    #[serde(rename = "default_operator", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub default_operator: Option<Operator>,
    /// Analyzer to use
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub analyzer: Option<String>,
    /// Whether to analyze wildcard terms
    #[serde(rename = "analyze_wildcard", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub analyze_wildcard: Option<bool>,
    /// Whether to lowercase expanded terms
    #[serde(
        rename = "lowercase_expanded_terms",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub lowercase_expanded_terms: Option<bool>,
    /// Whether to enable position increments in result
    #[serde(
        rename = "enable_position_increments",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub enable_position_increments: Option<bool>,
    /// Fuzzy max expansions
    #[serde(
        rename = "fuzzy_max_expansions",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub fuzzy_max_expansions: Option<i32>,
    /// Fuzziness parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fuzziness: Option<Fuzziness>,
    /// Fuzzy prefix length
    #[serde(
        rename = "fuzzy_prefix_length",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub fuzzy_prefix_length: Option<i32>,
    /// Fuzzy rewrite method
    #[serde(rename = "fuzzy_rewrite", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub fuzzy_rewrite: Option<String>,
    /// Phrase slop
    #[serde(rename = "phrase_slop", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub phrase_slop: Option<i32>,
    /// Boost value
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub boost: Option<f64>,
    /// Whether to enable auto generate phrase queries
    #[serde(
        rename = "auto_generate_phrase_queries",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub auto_generate_phrase_queries: Option<bool>,
    /// Allow leading wildcard flag
    #[serde(
        rename = "allow_leading_wildcard",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub allow_leading_wildcard: Option<bool>,
    /// Maximum number of terms that can be created by wildcard or prefix expansion
    #[serde(
        rename = "max_determinized_states",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub max_determinized_states: Option<i32>,
    /// Minimum should match parameter
    #[serde(
        rename = "minimum_should_match",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub minimum_should_match: Option<MinimumShouldMatch>,
    /// Lenient flag to ignore format based failures
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub lenient: Option<bool>,
    /// Time zone for date values
    #[serde(rename = "time_zone", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub time_zone: Option<String>,
    /// How scores from different queries are combined
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub type_: Option<QueryStringType>,
}

impl QueryStringQueryRuleBuilder {
    pub fn build(&self) -> Result<QueryStringQuery, QueryStringQueryRuleBuilderError> {
        Ok(QueryStringQuery {
            query_string: self.build_params()?,
        })
    }
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

/// Match phrase query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct MatchPhraseQuery {
    #[serde(rename = "match_phrase")]
    pub match_phrase: HashMap<String, MatchPhraseQueryRule>,
}

impl MatchPhraseQuery {
    /// Create a new builder for MatchPhraseQuery
    pub fn builder() -> MatchPhraseQueryBuilder {
        MatchPhraseQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::MatchPhrase(self)
    }
}

impl MatchPhraseQueryBuilder {
    /// Add a field to the match phrase query
    pub fn field<S: Into<String>, V: Into<MatchPhraseQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let match_phrase = self.match_phrase.get_or_insert_with(HashMap::new);
        match_phrase.insert(field.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, From)]
#[serde(untagged)]
pub enum MatchPhraseQueryRule {
    /// Simple query with just the query string
    Simple(String),
    /// Advanced query with additional parameters
    Advanced(MatchPhraseQueryRuleAdvanced),
}

impl MatchPhraseQueryRule {
    pub fn simple(value: String) -> Self {
        Self::Simple(value)
    }

    pub fn advanced() -> MatchPhraseQueryRuleAdvancedBuilder {
        MatchPhraseQueryRuleAdvancedBuilder::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct MatchPhraseQueryRuleAdvanced {
    /// Query text to match
    query: String,
    /// Analyzer to use for the query
    #[builder(default)]
    analyzer: Option<String>,
    /// Slop factor (number of words that can be skipped)
    #[builder(default)]
    slop: Option<i32>,
    /// Boost factor for this query
    #[builder(default)]
    boost: Option<f64>,
}

impl MatchPhraseQueryRuleAdvanced {
    pub fn builder() -> MatchPhraseQueryRuleAdvancedBuilder {
        MatchPhraseQueryRuleAdvancedBuilder::default()
    }
}

/// Match phrase prefix query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct MatchPhrasePrefixQuery {
    #[serde(rename = "match_phrase_prefix")]
    pub match_phrase_prefix: HashMap<String, MatchPhrasePrefixQueryRule>,
}

impl MatchPhrasePrefixQuery {
    /// Create a new builder for MatchPhrasePrefixQuery
    pub fn builder() -> MatchPhrasePrefixQueryBuilder {
        MatchPhrasePrefixQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::MatchPhrasePrefix(self)
    }
}

impl MatchPhrasePrefixQueryBuilder {
    /// Add a field to the match phrase prefix query
    pub fn field<S: Into<String>, V: Into<MatchPhrasePrefixQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let match_phrase_prefix = self.match_phrase_prefix.get_or_insert_with(HashMap::new);
        match_phrase_prefix.insert(field.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, From)]
#[serde(untagged)]
pub enum MatchPhrasePrefixQueryRule {
    /// Simple query with just the query string
    Simple(String),
    /// Advanced query with additional parameters
    Advanced(MatchPhrasePrefixQueryRuleAdvanced),
}

impl MatchPhrasePrefixQueryRule {
    pub fn simple(value: String) -> Self {
        Self::Simple(value)
    }

    pub fn advanced() -> MatchPhrasePrefixQueryRuleAdvancedBuilder {
        MatchPhrasePrefixQueryRuleAdvancedBuilder::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct MatchPhrasePrefixQueryRuleAdvanced {
    /// Query text to match
    query: String,
    /// Analyzer to use for the query
    #[builder(default)]
    analyzer: Option<String>,
    /// Slop factor (number of words that can be skipped)
    #[builder(default)]
    slop: Option<i32>,
    /// Maximum number of terms to match on the last word
    #[serde(rename = "max_expansions")]
    #[builder(default)]
    max_expansions: Option<i32>,
    /// Boost factor for this query
    #[builder(default)]
    boost: Option<f64>,
}

impl MatchPhrasePrefixQueryRuleAdvanced {
    pub fn builder() -> MatchPhrasePrefixQueryRuleAdvancedBuilder {
        MatchPhrasePrefixQueryRuleAdvancedBuilder::default()
    }
}

/// Multi-match query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct MultiMatchQuery {
    #[serde(rename = "multi_match")]
    pub multi_match: MultiMatchQueryRule,
}

impl MultiMatchQuery {
    /// Create a new builder for MultiMatchQuery
    pub fn builder() -> MultiMatchQueryRuleBuilder {
        MultiMatchQueryRuleBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::MultiMatch(self)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
#[builder(build_fn(name = "build_rule"))]
pub struct MultiMatchQueryRule {
    /// Query text to match
    query: String,
    /// Fields to search in
    #[builder(default)]
    fields: Option<Vec<String>>,
    /// Type of multi-match query
    #[serde(rename = "type")]
    #[builder(default)]
    type_: Option<MatchType>,
    /// Operator to use (OR or AND)
    #[builder(default)]
    operator: Option<Operator>,
    /// Minimum number of clauses that must match
    #[serde(rename = "minimum_should_match")]
    #[builder(default)]
    minimum_should_match: Option<MinimumShouldMatch>,
    /// Boost factor for this query
    #[builder(default)]
    boost: Option<f64>,
    /// Analyzer to use
    #[builder(default)]
    analyzer: Option<String>,
    /// Control fuzzy matching behavior
    #[builder(default)]
    fuzziness: Option<Fuzziness>,
    /// Length of common prefix required for fuzzy matching
    #[serde(rename = "prefix_length")]
    #[builder(default)]
    prefix_length: Option<i32>,
    /// Maximum number of term expansions for fuzzy matching
    #[serde(rename = "max_expansions")]
    #[builder(default)]
    max_expansions: Option<i32>,
    /// Whether to create a match phrase query for multi-term synonyms
    #[serde(rename = "auto_generate_synonyms_phrase_query")]
    #[builder(default)]
    auto_generate_synonyms_phrase_query: Option<bool>,
    /// Whether to ignore data type mismatches between query and field
    #[builder(default)]
    lenient: Option<bool>,
    /// How to handle queries with only stop words
    #[serde(rename = "zero_terms_query")]
    #[builder(default)]
    zero_terms_query: Option<ZeroTermsQuery>,
    /// Number of positions allowed between matching terms
    #[builder(default)]
    slop: Option<i32>,
    /// Factor between 0 and 1 to score across multiple fields
    #[serde(rename = "tie_breaker")]
    #[builder(default)]
    tie_breaker: Option<f64>,
}

impl MultiMatchQueryRuleBuilder {
    pub fn build(&self) -> Result<MultiMatchQuery, MultiMatchQueryRuleBuilderError> {
        Ok(MultiMatchQuery {
            multi_match: self.build_rule()?,
        })
    }
}

/// IDs query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct IdsQuery {
    pub ids: IdsQueryRule,
}

impl IdsQuery {
    /// Create a new builder for IdsQuery
    pub fn builder() -> IdsQueryRuleBuilder {
        IdsQueryRuleBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Ids(self)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
#[builder(build_fn(name = "build_rule"))]
pub struct IdsQueryRule {
    /// List of document IDs to match
    #[builder(default)]
    pub values: Vec<String>,
    /// Boost factor for this query
    #[builder(default)]
    pub boost: Option<f64>,
}

impl IdsQueryRule {
    pub fn builder() -> IdsQueryRuleBuilder {
        IdsQueryRuleBuilder::default()
    }
}

impl IdsQueryRuleBuilder {
    pub fn build(&self) -> Result<IdsQuery, IdsQueryRuleBuilderError> {
        Ok(IdsQuery {
            ids: self.build_rule()?,
        })
    }
}

/// Fuzzy query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct FuzzyQuery {
    pub fuzzy: HashMap<String, FuzzyQueryRule>,
}

impl FuzzyQuery {
    /// Create a new builder for FuzzyQuery
    pub fn builder() -> FuzzyQueryBuilder {
        FuzzyQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Fuzzy(self)
    }
}

impl FuzzyQueryBuilder {
    /// Add a field to the fuzzy query
    pub fn field<S: Into<String>, V: Into<FuzzyQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let fuzzy = self.fuzzy.get_or_insert_with(HashMap::new);
        fuzzy.insert(field.into(), value.into());
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct FuzzyQueryRule {
    /// Value to match
    value: String,
    /// Fuzziness parameter
    #[builder(default)]
    fuzziness: Option<Fuzziness>,
    /// Length of common prefix required
    #[serde(rename = "prefix_length")]
    #[builder(default)]
    prefix_length: Option<i32>,
    /// Maximum number of term expansions
    #[serde(rename = "max_expansions")]
    #[builder(default)]
    max_expansions: Option<i32>,
    /// Whether transpositions should be counted as a single edit operation
    #[builder(default)]
    transpositions: Option<bool>,
    /// Rewrite method
    #[builder(default)]
    rewrite: Option<String>,
    /// Boost factor for this query
    #[builder(default)]
    boost: Option<f64>,
}

impl FuzzyQueryRule {
    pub fn builder() -> FuzzyQueryRuleBuilder {
        FuzzyQueryRuleBuilder::default()
    }
}

/// Regular expression query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct RegexpQuery {
    #[serde(rename = "regexp")]
    pub regexp: HashMap<String, RegexpQueryRule>,
}

impl RegexpQuery {
    /// Create a new builder for RegexpQuery
    pub fn builder() -> RegexpQueryBuilder {
        RegexpQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Regexp(self)
    }
}

impl RegexpQueryBuilder {
    /// Add a field to the regexp query
    pub fn field<S: Into<String>, V: Into<RegexpQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let regexp = self.regexp.get_or_insert_with(HashMap::new);
        regexp.insert(field.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, From)]
#[serde(untagged)]
pub enum RegexpQueryRule {
    /// Simple regexp with just the value
    Simple(String),
    /// Advanced regexp with additional parameters
    Advanced(RegexpQueryRuleAdvanced),
}

impl RegexpQueryRule {
    pub fn simple(value: String) -> Self {
        Self::Simple(value)
    }

    pub fn advanced() -> RegexpQueryRuleAdvancedBuilder {
        RegexpQueryRuleAdvancedBuilder::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct RegexpQueryRuleAdvanced {
    /// Value to match
    value: String,
    /// Boost value
    #[builder(default)]
    boost: Option<f64>,
    /// Whether to use case-insensitive matching
    #[serde(rename = "case_insensitive")]
    #[builder(default)]
    case_insensitive: Option<bool>,
    /// Regular expression flags
    #[builder(default)]
    flags: Option<String>,
    /// Maximum number of automaton states the query requires
    #[serde(rename = "max_determinized_states")]
    #[builder(default)]
    max_determinized_states: Option<i32>,
    /// Rewrite method
    #[builder(default)]
    rewrite: Option<String>,
}

/// Terms query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct TermsQuery {
    pub terms: HashMap<String, TermsQueryRule>,
}

impl TermsQuery {
    pub fn builder() -> TermsQueryBuilder {
        TermsQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Terms(self)
    }
}

impl TermsQueryBuilder {
    pub fn field<S: Into<String>, V: Into<TermsQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let terms = self.terms.get_or_insert_with(HashMap::new);
        terms.insert(field.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, From)]
#[serde(untagged)]
pub enum TermsQueryRule {
    /// Simple query with just the terms
    Simple(Vec<serde_json::Value>),
    /// Advanced query with additional parameters
    Advanced(TermsQueryRuleAdvanced),
}

impl TermsQueryRule {
    pub fn simple(values: Vec<serde_json::Value>) -> Self {
        Self::Simple(values)
    }

    pub fn advanced() -> TermsQueryRuleAdvancedBuilder {
        TermsQueryRuleAdvancedBuilder::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct TermsQueryRuleAdvanced {
    /// Terms to match
    pub values: Vec<serde_json::Value>,
    /// Case insensitive flag
    #[serde(rename = "case_insensitive")]
    #[builder(default)]
    pub case_insensitive: Option<bool>,
    /// Boost value
    #[builder(default)]
    pub boost: Option<f64>,
}

impl TermsQueryRuleAdvanced {
    pub fn builder() -> TermsQueryRuleAdvancedBuilder {
        TermsQueryRuleAdvancedBuilder::default()
    }
}

/// Terms set query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct TermsSetQuery {
    #[serde(rename = "terms_set")]
    pub terms_set: HashMap<String, TermsSetQueryRule>,
}

impl TermsSetQuery {
    pub fn builder() -> TermsSetQueryBuilder {
        TermsSetQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::TermsSet(self)
    }
}

impl TermsSetQueryBuilder {
    pub fn field<S: Into<String>, V: Into<TermsSetQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let terms_set = self.terms_set.get_or_insert_with(HashMap::new);
        terms_set.insert(field.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct TermsSetQueryRule {
    /// Terms to match
    pub terms: Vec<String>,
    /// Field containing the required number of matches
    #[serde(
        rename = "minimum_should_match_field",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub minimum_should_match_field: Option<String>,
    /// Script defining the required number of matches
    #[serde(
        rename = "minimum_should_match_script",
        skip_serializing_if = "Option::is_none"
    )]
    #[builder(default)]
    pub minimum_should_match_script: Option<crate::types::script::Script>,
    /// Static value for minimum required matches
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub minimum_should_match: Option<MinimumShouldMatch>,
    /// Boost factor for this query
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub boost: Option<f64>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct GeoDistanceQuery {
    pub geo_distance: GeoDistanceQueryRule,
}

impl GeoDistanceQuery {
    /// Create a new builder for GeoDistanceQuery
    pub fn builder() -> GeoDistanceQueryRuleBuilder {
        GeoDistanceQueryRuleBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::GeoDistance(self)
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
#[builder(build_fn(name = "build_rule"))]
pub struct GeoDistanceQueryRule {
    /// Distance value (e.g., "10km")
    pub distance: String,
    /// Center point for the distance calculation
    #[serde(flatten)]
    pub points: GeoPoints,
    /// How distances are calculated
    #[serde(rename = "distance_type")]
    #[builder(default)]
    pub distance_type: Option<GeoDistanceType>,
    /// Validation mode
    #[serde(rename = "validation_method")]
    #[builder(default)]
    pub validation_method: Option<GeoValidationMethod>,
    /// Whether to ignore malformed geo points
    #[serde(rename = "ignore_unmapped")]
    #[builder(default)]
    pub ignore_unmapped: Option<bool>,
}

impl GeoDistanceQueryRule {
    pub fn builder() -> GeoDistanceQueryRuleBuilder {
        GeoDistanceQueryRuleBuilder::default()
    }
}

impl GeoDistanceQueryRuleBuilder {
    pub fn point(mut self, point: GeoPointField) -> Self {
        self.points.get_or_insert_default().0.push(point);
        self
    }

    pub fn build(self) -> Result<GeoDistanceQuery, GeoDistanceQueryRuleBuilderError> {
        Ok(GeoDistanceQuery {
            geo_distance: self.build_rule()?,
        })
    }
}

/// Geo distance calculation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeoDistanceType {
    /// Arc calculation on a sphere
    Arc,
    /// Plane calculation on a flat surface
    Plane,
}

/// Geo validation method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeoValidationMethod {
    /// No validation
    IgnoreMalformed,
    /// Coordinates are required to be valid
    Strict,
    /// Coordinates are coerced to valid values
    Coerce,
}

/// Geo shape query
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct GeoShapeQuery {
    /// Field to query
    pub geo_shape: HashMap<String, GeoShapeQueryRule>,
}

impl GeoShapeQuery {
    /// Create a new builder for GeoShapeQuery
    pub fn builder() -> GeoShapeQueryBuilder {
        GeoShapeQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::GeoShape(self)
    }
}

impl GeoShapeQueryBuilder {
    /// Add a field to the geo shape query
    pub fn field<S: Into<String>, V: Into<GeoShapeQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let geo_shape = self.geo_shape.get_or_insert_with(HashMap::new);
        geo_shape.insert(field.into(), value.into());
        self
    }
}

/// Parameters for geo_shape query
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct GeoShapeQueryRule {
    /// Shape to query
    pub shape: GeoShape,
    /// Spatial relation
    #[builder(default)]
    pub relation: Option<GeoShapeRelation>,
    /// Boost factor for this query
    #[builder(default)]
    pub boost: Option<f64>,
}

impl GeoShapeQueryRule {
    /// Create a new builder for GeoShapeQueryParams
    pub fn builder() -> GeoShapeQueryRuleBuilder {
        GeoShapeQueryRuleBuilder::default()
    }
}

impl From<GeoShape> for GeoShapeQueryRule {
    fn from(shape: GeoShape) -> Self {
        Self {
            shape,
            relation: None,
            boost: None,
        }
    }
}

/// Geo shape relation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum GeoShapeRelation {
    /// Shape contains the query
    Contains,
    /// Shape is within the query
    Within,
    /// Shape intersects the query
    Intersects,
    /// Shape disjoint from the query
    Disjoint,
}

/// Nested query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedQuery {
    pub nested: NestedQueryParams,
}

/// Parameters for nested query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NestedQueryParams {
    /// Path to the nested field
    pub path: String,
    /// Query to execute on the nested field
    pub query: Box<Query>,
    /// How scores are calculated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_mode: Option<NestedScoreMode>,
    /// Whether to consider unmapped paths as matching
    #[serde(rename = "ignore_unmapped", skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<bool>,
    /// Boost factor for this query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Nested score mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NestedScoreMode {
    /// Use average score of matching child documents
    Avg,
    /// Use sum of scores of matching child documents
    Sum,
    /// Use maximum score of matching child documents
    Max,
    /// Use minimum score of matching child documents
    Min,
    /// Use score of parent document
    None,
}

/// Has child query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HasChildQuery {
    #[serde(rename = "has_child")]
    pub has_child: HasChildQueryParams,
}

/// Parameters for has_child query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HasChildQueryParams {
    /// Type of child document
    #[serde(rename = "type")]
    pub type_: String,
    /// Query to match child documents
    pub query: Box<Query>,
    /// How scores are calculated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_mode: Option<ChildScoreMode>,
    /// Minimum number of children that match the query
    #[serde(rename = "min_children", skip_serializing_if = "Option::is_none")]
    pub min_children: Option<u32>,
    /// Maximum number of children that match the query
    #[serde(rename = "max_children", skip_serializing_if = "Option::is_none")]
    pub max_children: Option<u32>,
    /// Whether to consider unmapped types as matching
    #[serde(rename = "ignore_unmapped", skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<bool>,
    /// Boost factor for this query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Child score mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChildScoreMode {
    /// No scoring, just filter
    None,
    /// Use average score of matching child documents
    Avg,
    /// Use sum of scores of matching child documents
    Sum,
    /// Use maximum score of matching child documents
    Max,
    /// Use minimum score of matching child documents
    Min,
}

/// Has parent query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HasParentQuery {
    #[serde(rename = "has_parent")]
    pub has_parent: HasParentQueryParams,
}

/// Parameters for has_parent query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HasParentQueryParams {
    /// Type of parent document
    #[serde(rename = "parent_type")]
    pub parent_type: String,
    /// Query to match parent documents
    pub query: Box<Query>,
    /// Whether to score based on parent match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<bool>,
    /// Whether to consider unmapped types as matching
    #[serde(rename = "ignore_unmapped", skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<bool>,
    /// Boost factor for this query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Parent ID query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParentIdQuery {
    #[serde(rename = "parent_id")]
    pub parent_id: ParentIdQueryParams,
}

/// Parameters for parent_id query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParentIdQueryParams {
    /// Type of child document
    #[serde(rename = "type")]
    pub type_: String,
    /// ID of parent document
    pub id: String,
    /// Whether to consider unmapped types as matching
    #[serde(rename = "ignore_unmapped", skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<bool>,
    /// Boost factor for this query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// Script query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptQuery {
    pub script: ScriptQueryParams,
}

/// Parameters for script query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScriptQueryParams {
    /// Script to execute
    pub script: crate::types::script::Script,
    /// Boost factor for this query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
}

/// More like this query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoreLikeThisQuery {
    #[serde(rename = "more_like_this")]
    pub more_like_this: MoreLikeThisQueryParams,
}

/// Parameters for more_like_this query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoreLikeThisQueryParams {
    /// Fields to use for similarity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    /// Documents to find similar documents to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like: Option<Vec<MoreLikeThisLike>>,
    /// Terms to find similar documents to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_text: Option<String>,
    /// Minimum term frequency
    #[serde(rename = "min_term_freq", skip_serializing_if = "Option::is_none")]
    pub min_term_freq: Option<u32>,
    /// Maximum term frequency
    #[serde(rename = "max_query_terms", skip_serializing_if = "Option::is_none")]
    pub max_query_terms: Option<u32>,
    /// Terms to ignore
    #[serde(rename = "stop_words", skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<Vec<String>>,
    /// Minimum document frequency
    #[serde(rename = "min_doc_freq", skip_serializing_if = "Option::is_none")]
    pub min_doc_freq: Option<u32>,
    /// Maximum document frequency
    #[serde(rename = "max_doc_freq", skip_serializing_if = "Option::is_none")]
    pub max_doc_freq: Option<u32>,
    /// Minimum word length
    #[serde(rename = "min_word_length", skip_serializing_if = "Option::is_none")]
    pub min_word_length: Option<u32>,
    /// Maximum word length
    #[serde(rename = "max_word_length", skip_serializing_if = "Option::is_none")]
    pub max_word_length: Option<u32>,
    /// Boost factor for terms
    #[serde(rename = "boost_terms", skip_serializing_if = "Option::is_none")]
    pub boost_terms: Option<f64>,
    /// Boost factor for this query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    /// Analyzer to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    /// Minimum number of terms that should match
    #[serde(
        rename = "minimum_should_match",
        skip_serializing_if = "Option::is_none"
    )]
    pub minimum_should_match: Option<MinimumShouldMatch>,
    /// Whether to include the query term
    #[serde(rename = "include", skip_serializing_if = "Option::is_none")]
    pub include: Option<bool>,
}

/// More like this like document reference
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MoreLikeThisLike {
    /// Document referenced by its ID
    Doc {
        /// Index of the document
        #[serde(rename = "_index", skip_serializing_if = "Option::is_none")]
        index: Option<String>,
        /// ID of the document
        #[serde(rename = "_id")]
        id: String,
    },
    /// Document provided directly
    Text(String),
}

/// Wildcard query for pattern matching
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct WildcardQuery {
    pub wildcard: HashMap<String, WildcardQueryRule>,
}

impl WildcardQuery {
    /// Create a new builder for WildcardQuery
    pub fn builder() -> WildcardQueryBuilder {
        WildcardQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Wildcard(self)
    }
}

impl WildcardQueryBuilder {
    /// Add a field to the wildcard query
    pub fn field<S: Into<String>, V: Into<WildcardQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let wildcard = self.wildcard.get_or_insert_with(HashMap::new);
        wildcard.insert(field.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, From)]
#[serde(untagged)]
pub enum WildcardQueryRule {
    /// Simple query with just the pattern
    Simple(String),
    /// Advanced query with additional parameters
    Advanced(WildcardQueryRuleAdvanced),
}

impl WildcardQueryRule {
    pub fn simple(value: String) -> Self {
        Self::Simple(value)
    }

    pub fn advanced() -> WildcardQueryRuleAdvancedBuilder {
        WildcardQueryRuleAdvancedBuilder::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct WildcardQueryRuleAdvanced {
    /// Wildcard pattern
    value: String,
    /// Boost value
    #[builder(default)]
    boost: Option<f64>,
    /// Case insensitive flag
    #[serde(rename = "case_insensitive")]
    #[builder(default)]
    case_insensitive: Option<bool>,
    /// Rewrite method
    #[builder(default)]
    rewrite: Option<String>,
}

impl WildcardQueryRuleAdvanced {
    pub fn builder() -> WildcardQueryRuleAdvancedBuilder {
        WildcardQueryRuleAdvancedBuilder::default()
    }
}

/// Prefix query for prefix matching
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct PrefixQuery {
    /// Field to query
    #[serde(rename = "prefix")]
    pub prefix: HashMap<String, PrefixQueryRule>,
}

impl PrefixQuery {
    /// Create a new builder for PrefixQuery
    pub fn builder() -> PrefixQueryBuilder {
        PrefixQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::Prefix(self)
    }
}

impl PrefixQueryBuilder {
    /// Add a field to the prefix query
    pub fn field<S: Into<String>, V: Into<PrefixQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let prefix = self.prefix.get_or_insert_with(HashMap::new);
        prefix.insert(field.into(), value.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, From)]
#[serde(untagged)]
pub enum PrefixQueryRule {
    /// Simple query with just the prefix
    Simple(String),
    /// Advanced query with additional parameters
    Advanced(PrefixQueryRuleAdvanced),
}

impl PrefixQueryRule {
    pub fn simple(value: String) -> Self {
        Self::Simple(value)
    }

    pub fn advanced() -> PrefixQueryRuleAdvancedBuilder {
        PrefixQueryRuleAdvancedBuilder::default()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
pub struct PrefixQueryRuleAdvanced {
    /// Prefix value
    value: String,
    /// Boost value
    #[builder(default)]
    boost: Option<f64>,
    /// Rewrite method
    #[builder(default)]
    rewrite: Option<String>,
    /// Case insensitive flag
    #[serde(rename = "case_insensitive")]
    #[builder(default)]
    case_insensitive: Option<bool>,
}

lit_str!(LitAuto, "auto");

/// Fuzziness parameter
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Fuzziness {
    /// Auto fuzziness
    #[serde(with = "LitAuto")]
    Auto,
    /// Specific edit distance
    Distance(i32),
}

/// Minimum should match specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MinimumShouldMatch {
    /// Integer value (absolute number)
    Absolute(i32),
    /// String value (percentage or combination)
    Complex(String),
}

/// Geo bounding box query
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option), default)]
pub struct GeoBoundingBoxQuery {
    pub geo_bounding_box: HashMap<String, GeoBoundingBoxQueryRule>,
}

impl GeoBoundingBoxQuery {
    /// Create a new builder for GeoDistanceQuery
    pub fn builder() -> GeoBoundingBoxQueryBuilder {
        GeoBoundingBoxQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::GeoBoundingBox(self)
    }
}

impl GeoBoundingBoxQueryBuilder {
    pub fn field<S: Into<String>, V: Into<GeoBoundingBoxQueryRule>>(
        &mut self,
        field: S,
        value: V,
    ) -> &mut Self {
        let geo_bounding_box = self.geo_bounding_box.get_or_insert_with(HashMap::new);
        geo_bounding_box.insert(field.into(), value.into());
        self
    }
}

/// Geo bounding box query
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option), default)]
pub struct GeoBoundingBoxQueryRule {
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,

    /// How to validate the query
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,

    /// Whether to ignore unmapped fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<bool>,
}

impl GeoBoundingBoxQueryRule {
    pub fn builder() -> GeoBoundingBoxQueryRuleBuilder {
        GeoBoundingBoxQueryRuleBuilder::default()
    }
}

/// Geo polygon query
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
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

impl GeoPolygonQuery {
    /// Create a new builder for GeoDistanceQuery
    pub fn builder() -> GeoPolygonQueryBuilder {
        GeoPolygonQueryBuilder::default()
    }

    pub fn into_query(self) -> Query {
        Query::GeoPolygon(self)
    }
}

/// Geo shape representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

impl GeoShape {
    /// Create a new shape from GeoJSON
    pub fn geo_json(shape: GeoJsonShape) -> Self {
        Self::GeoJson(shape)
    }

    /// Create a new point shape
    pub fn point(lat: f64, lon: f64) -> Self {
        Self::GeoJson(GeoJsonShape::Point {
            coordinates: [lon, lat],
        })
    }

    /// Create a new polygon shape from a list of points
    pub fn polygon(coordinates: Vec<Vec<[f64; 2]>>) -> Self {
        Self::GeoJson(GeoJsonShape::Polygon { coordinates })
    }
}

/// Multi-match query types for scoring and matching behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
    /// Returns documents that match any field. Uses the _score of the best-matching field.
    BestFields,
    /// Returns documents that match any field. Uses a combined score of each matching field.
    MostFields,
    /// Treats all fields as if they were one field. Processes fields with the same analyzer and matches words in any field.
    CrossFields,
    /// Runs a match_phrase query on each field. Uses the _score of the best-matching field.
    Phrase,
    /// Runs a match_phrase_prefix query on each field. Uses the _score of the best-matching field.
    PhrasePrefix,
    /// Runs a match_bool_prefix query on each field. Uses a combined score of each matched field.
    BoolPrefix,
}

/// GeoJSON shape types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Input for more like this query
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[serde_as]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GeoPoints(#[serde_as(as = "KeyValueMap<_>")] pub Vec<GeoPointField>);

/// Geo point field representation
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoPointField {
    #[serde(rename = "$key$")]
    pub field: String,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
}

impl GeoPointField {
    pub fn new(field: impl Into<String>, lat: f64, lon: f64) -> Self {
        Self {
            field: field.into(),
            lat,
            lon,
        }
    }
}
