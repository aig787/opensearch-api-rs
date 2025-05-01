// //! Builder pattern DSL for constructing search queries
//
// use derive_builder::Builder;
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
//
// /// Match query parameters
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq)]
// #[builder(setter(strip_option))]
// pub struct Match {
//     /// Value to match
//     query: String,
//
//     /// Operator to use for matching multiple terms
//     #[builder(default)]
//     operator: Option<Operator>,
//
//     /// Fuzziness parameter
//     #[builder(default)]
//     fuzziness: Option<Fuzziness>,
//
//     /// Prefix length for fuzzy queries
//     #[builder(default)]
//     prefix_length: Option<u32>,
//
//     /// Maximum query expansion for fuzzy queries
//     #[builder(default)]
//     max_expansions: Option<u32>,
//
//     /// Boost value for this query
//     #[builder(default)]
//     boost: Option<f64>,
//
//     /// Analyzer to use
//     #[builder(default)]
//     analyzer: Option<String>,
// }
//
// /// Builder for creating a match query
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct MatchQuery {
//     /// Field to match mappings
//     #[serde(rename = "match")]
//     pub match_: HashMap<String, Match>,
// }
//
// impl Match {
//     /// Create a new match query builder
//     pub fn new() -> MatchBuilder {
//         MatchBuilder::default()
//     }
//
//     /// Create a new match builder
//     pub fn builder() -> MatchBuilder {
//         MatchBuilder::default()
//     }
// }
//
// impl MatchQuery {
//     pub fn builder() -> MatchQueryRulesBuilder {
//         MatchQueryRulesBuilder::default()
//     }
// }
//
// /// Builder for creating a match query
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq)]
// #[builder(setter(strip_option))]
// pub struct MatchQueryRules {
//     /// Field to search
//     field: String,
//
//     /// Value to match
//     #[builder(default)]
//     query: Option<String>,
//
//     /// Operator to use for matching multiple terms
//     #[builder(default)]
//     operator: Option<Operator>,
//
//     /// Fuzziness parameter
//     #[builder(default)]
//     fuzziness: Option<Fuzziness>,
//
//     /// Prefix length for fuzzy queries
//     #[builder(default)]
//     prefix_length: Option<u32>,
//
//     /// Maximum query expansion for fuzzy queries
//     #[builder(default)]
//     max_expansions: Option<u32>,
//
//     /// Boost value for this query
//     #[builder(default)]
//     boost: Option<f64>,
//
//     /// Analyzer to use
//     #[builder(default)]
//     analyzer: Option<String>,
// }
//
// impl MatchQueryRules {
//     /// Create a new match query rules builder
//     pub fn builder() -> MatchQueryRulesBuilder {
//         MatchQueryRulesBuilder::default()
//     }
// }
//
// impl MatchQueryRulesBuilder {
//     /// Build the final match query object
//     pub fn build_query(&self) -> Result<Query, String> {
//         let builder = self.clone();
//         let query = builder
//             .build()
//             .map_err(|e| format!("Failed to build match query: {}", e))?;
//
//         // Build the match parameters
//         let mut match_params_builder = MatchBuilder::default();
//
//         if let Some(query_value) = query.query {
//             match_params_builder.query(query_value);
//         } else {
//             return Err("Query is required for match query".to_string());
//         }
//
//         if let Some(op) = query.operator {
//             match_params_builder.operator(op);
//         }
//
//         if let Some(fuzz) = query.fuzziness {
//             match_params_builder.fuzziness(fuzz);
//         }
//
//         if let Some(pl) = query.prefix_length {
//             match_params_builder.prefix_length(pl);
//         }
//
//         if let Some(me) = query.max_expansions {
//             match_params_builder.max_expansions(me);
//         }
//
//         if let Some(b) = query.boost {
//             match_params_builder.boost(b);
//         }
//
//         if let Some(a) = query.analyzer {
//             match_params_builder.analyzer(a);
//         }
//
//         let match_params = match_params_builder
//             .build()
//             .map_err(|e| format!("Failed to build match parameters: {}", e))?;
//
//         let mut field_map = HashMap::new();
//
//         // Create the appropriate MatchQueryParams based on complexity
//         let params = if match_params.operator.is_none()
//             && match_params.fuzziness.is_none()
//             && match_params.prefix_length.is_none()
//             && match_params.max_expansions.is_none()
//             && match_params.boost.is_none()
//             && match_params.analyzer.is_none()
//         {
//             // Simple case - just the query text
//             query::MatchQueryParams::Simple(match_params.query)
//         } else {
//             // Advanced case - include all the optional parameters
//             query::MatchQueryParams::Advanced {
//                 query: match_params.query,
//                 operator: match_params.operator,
//                 analyzer: match_params.analyzer,
//                 minimum_should_match: None, // Not in the builder but in the params
//                 fuzziness: match_params.fuzziness,
//                 prefix_length: match_params.prefix_length,
//                 max_expansions: match_params.max_expansions,
//                 boost: match_params.boost,
//             }
//         };
//
//         // Insert into the field map
//         field_map.insert(query.field, params);
//
//         let match_query = query::MatchQuery { match_: field_map };
//
//         Ok(Query::Match(match_query))
//     }
// }
//
// /// Term query parameters
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq)]
// #[builder(setter(strip_option))]
// pub struct Term {
//     /// Value to match exactly
//     value: serde_json::Value,
//
//     /// Boost value for this query
//     #[builder(default)]
//     boost: Option<f64>,
//
//     /// Case insensitive setting
//     #[builder(default)]
//     case_insensitive: Option<bool>,
// }
//
// /// Builder for creating a term query
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct TermQuery {
//     /// Field to term mappings
//     #[serde(rename = "term")]
//     pub term: HashMap<String, Term>,
// }
//
// impl Term {
//     /// Create a new term builder
//     pub fn builder() -> TermBuilder {
//         TermBuilder::default()
//     }
//
//     /// Create a new term parameters instance
//     pub fn new() -> TermBuilder {
//         TermBuilder::default()
//     }
// }
//
// impl TermQuery {
//     /// Create a new term query builder
//     pub fn builder() -> TermQueryRulesBuilder {
//         TermQueryRulesBuilder::default()
//     }
// }
//
// /// Builder for creating a term query
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq)]
// #[builder(setter(strip_option))]
// pub struct TermQueryRules {
//     /// Field to search
//     field: String,
//
//     /// Value to match exactly
//     #[builder(default)]
//     value: Option<serde_json::Value>,
//
//     /// Boost value for this query
//     #[builder(default)]
//     boost: Option<f64>,
//
//     /// Case insensitive setting
//     #[builder(default)]
//     case_insensitive: Option<bool>,
// }
//
// impl TermQueryRules {
//     /// Create a new term query rules builder
//     pub fn builder() -> TermQueryRulesBuilder {
//         TermQueryRulesBuilder::default()
//     }
// }
//
// impl TermQueryRulesBuilder {
//     /// Build the final term query object
//     pub fn build_query(&self) -> Result<Query, String> {
//         let builder = self.clone();
//         let query = builder
//             .build()
//             .map_err(|e| format!("Failed to build term query: {}", e))?;
//
//         // Build the term parameters
//         let mut term_params_builder = TermBuilder::default();
//
//         if let Some(value) = query.value {
//             term_params_builder.value(value);
//         } else {
//             return Err("Value is required for term query".to_string());
//         }
//
//         if let Some(b) = query.boost {
//             term_params_builder.boost(b);
//         }
//
//         if let Some(ci) = query.case_insensitive {
//             term_params_builder.case_insensitive(ci);
//         }
//
//         let term_params = term_params_builder
//             .build()
//             .map_err(|e| format!("Failed to build term parameters: {}", e))?;
//
//         let mut field_map = HashMap::new();
//
//         // Create the appropriate TermQueryParams based on complexity
//         let params = if term_params.boost.is_none() && term_params.case_insensitive.is_none() {
//             // Simple case - just the value
//             query::TermQueryParams::Simple(term_params.value)
//         } else {
//             // Advanced case - include all the optional parameters
//             query::TermQueryParams::Advanced {
//                 value: term_params.value,
//                 boost: term_params.boost,
//                 case_insensitive: term_params.case_insensitive,
//             }
//         };
//
//         // Insert into the field map
//         field_map.insert(query.field, params);
//
//         let term_query = query::TermQuery { term: field_map };
//
//         Ok(Query::Term(term_query))
//     }
// }
//
// /// Builder for creating a range query
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq)]
// #[builder(setter(strip_option))]
// pub struct RangeQuery {
//     /// Field to search
//     field: String,
//
//     /// Greater than value
//     #[builder(default)]
//     gt: Option<serde_json::Value>,
//
//     /// Greater than or equal to value
//     #[builder(default)]
//     gte: Option<serde_json::Value>,
//
//     /// Less than value
//     #[builder(default)]
//     lt: Option<serde_json::Value>,
//
//     /// Less than or equal to value
//     #[builder(default)]
//     lte: Option<serde_json::Value>,
//
//     /// Boost value for this query
//     #[builder(default)]
//     boost: Option<f64>,
//
//     /// Time zone for date values
//     #[builder(default)]
//     time_zone: Option<String>,
//
//     /// Format for date values
//     #[builder(default)]
//     format: Option<String>,
//
//     /// Relation for handling ranges
//     #[builder(default)]
//     relation: Option<RangeRelation>,
// }
//
// impl RangeQuery {
//     /// Create a new range query builder for the given field
//     pub fn field(field: impl Into<String>) -> RangeQueryBuilder {
//         let mut builder = RangeQueryBuilder::default();
//         builder.field(field.into());
//         builder
//     }
//
//     /// Create a new range query builder
//     pub fn builder() -> RangeQueryBuilder {
//         RangeQueryBuilder::default()
//     }
// }
//
// impl RangeQueryBuilder {
//     /// Build the final range query object
//     pub fn build_query(&self) -> Result<Query, String> {
//         let query_builder = self.clone();
//         let query = query_builder
//             .build()
//             .map_err(|e| format!("Failed to build range query: {}", e))?;
//
//         // Create range query parameters
//         let range_params = query::RangeQueryParams {
//             greater_than: query.gt,
//             greater_than_or_equal_to: query.gte,
//             less_than: query.lt,
//             less_than_or_equal_to: query.lte,
//             format: query.format,
//             time_zone: query.time_zone,
//             relation: query.relation,
//             boost: query.boost,
//         };
//
//         // Create a HashMap for the field
//         let mut field_map = std::collections::HashMap::new();
//         field_map.insert(query.field, range_params);
//
//         // Create the RangeQuery with the field map
//         let range_query = query::RangeQuery { field: field_map };
//
//         // Create the Query with the RangeQuery
//         Ok(Query::Range(range_query))
//     }
// }
//
// /// Builder for creating a bool query
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq)]
// #[builder(setter(strip_option))]
// pub struct BoolQuery {
//     /// Must clauses - results must match these queries
//     #[builder(default)]
//     must: Option<Vec<Query>>,
//
//     /// Should clauses - results should match these queries
//     #[builder(default)]
//     should: Option<Vec<Query>>,
//
//     /// Must not clauses - results must not match these queries
//     #[builder(default)]
//     must_not: Option<Vec<Query>>,
//
//     /// Filter clauses - results must match these queries but they don't affect scoring
//     #[builder(default)]
//     filter: Option<Vec<Query>>,
//
//     /// Minimum should match parameter
//     #[builder(default)]
//     minimum_should_match: Option<MinimumShouldMatch>,
//
//     /// Boost value for this query
//     #[builder(default)]
//     boost: Option<f64>,
// }
//
// /// Builder for creating a match all query
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq)]
// #[builder(setter(strip_option))]
// pub struct MatchAllQuery {
//     /// Boost value for this query
//     #[builder(default)]
//     boost: Option<f64>,
// }
//
// impl MatchAllQuery {
//     pub fn builder() -> MatchAllQueryBuilder {
//         MatchAllQueryBuilder::default()
//     }
// }
//
// impl MatchAllQueryBuilder {
//     /// Build the final match all query object
//     pub fn build_query(&self) -> Result<Query, String> {
//         let match_all_query = query::MatchAllQuery {
//             boost: self.boost.unwrap_or_default(),
//         };
//
//         Ok(Query::MatchAll(match_all_query))
//     }
// }
//
// /// Builder for creating a match none query
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq)]
// pub struct MatchNoneQuery {}
//
// impl MatchNoneQuery {
//     /// Create a new match none query builder
//     pub fn builder() -> MatchNoneQueryBuilder {
//         MatchNoneQueryBuilder::default()
//     }
//
//     /// Build the final match none query object
//     pub fn build_query(&self) -> Result<Query, String> {
//         let match_none_query = query::MatchNoneQuery {};
//         Ok(Query::MatchNone(match_none_query))
//     }
// }
//
// impl BoolQuery {
//     /// Create a new bool query builder
//     pub fn new() -> BoolQueryBuilder {
//         BoolQueryBuilder::default()
//     }
//
//     /// Create a new bool query builder
//     pub fn builder() -> BoolQueryBuilder {
//         BoolQueryBuilder::default()
//     }
// }
//
// impl BoolQueryBuilder {
//     /// Add a query to the must clauses
//     pub fn add_must(&mut self, query: query::Query) -> &mut Self {
//         self.must = Some(Some(match self.must.take() {
//             Some(Some(mut queries)) => {
//                 queries.push(query);
//                 queries
//             }
//             _ => vec![query],
//         }));
//         self
//     }
//
//     /// Add a query to the should clauses
//     pub fn add_should(&mut self, query: query::Query) -> &mut Self {
//         // Initialize the Option<Vec<Query>> if it's None or unwrap it if it's Some
//         self.should = Some(Some(match self.should.take() {
//             Some(Some(mut queries)) => {
//                 queries.push(query);
//                 queries
//             }
//             _ => vec![query],
//         }));
//         self
//     }
//
//     /// Add a query to the must_not clauses
//     pub fn add_must_not(&mut self, query: query::Query) -> &mut Self {
//         // Initialize the Option<Vec<Query>> if it's None or unwrap it if it's Some
//         self.must_not = Some(Some(match self.must_not.take() {
//             Some(Some(mut queries)) => {
//                 queries.push(query);
//                 queries
//             }
//             _ => vec![query],
//         }));
//         self
//     }
//
//     /// Add a query to the filter clauses
//     pub fn add_filter(&mut self, query: query::Query) -> &mut Self {
//         // Initialize the Option<Vec<Query>> if it's None or unwrap it if it's Some
//         self.filter = Some(Some(match self.filter.take() {
//             Some(Some(mut queries)) => {
//                 queries.push(query);
//                 queries
//             }
//             _ => vec![query],
//         }));
//         self
//     }
//
//     /// Build the final bool query object
//     pub fn build_query(&self) -> Result<query::Query, String> {
//         let query_builder = self.clone();
//         let query = query_builder
//             .build()
//             .map_err(|e| format!("Failed to build bool query: {}", e))?;
//
//         let bool_query = query::BoolQuery {
//             must: query.must,
//             should: query.should,
//             must_not: query.must_not,
//             filter: query.filter,
//             minimum_should_match: query.minimum_should_match,
//             boost: query.boost,
//         };
//
//         Ok(Query::Bool(bool_query))
//     }
// }
//
// /// Builder for creating a query string query
// #[serde_with::skip_serializing_none]
// #[derive(Debug, Clone, Builder, Serialize, Deserialize, PartialEq)]
// #[builder(setter(strip_option))]
// pub struct QueryStringQuery {
//     /// Query string
//     pub query: String,
//
//     /// Default field to search if not specified in the query
//     #[builder(default)]
//     pub default_field: Option<String>,
//
//     /// List of fields to search
//     #[builder(default)]
//     pub fields: Option<Vec<String>>,
//
//     /// Default operator (AND/OR)
//     #[builder(default)]
//     pub default_operator: Option<Operator>,
//
//     /// Analyzer to use
//     #[builder(default)]
//     pub analyzer: Option<String>,
//
//     /// Whether to analyze wildcard terms
//     #[builder(default)]
//     pub analyze_wildcard: Option<bool>,
//
//     /// Whether to lowercase expanded terms
//     #[builder(default)]
//     pub lowercase_expanded_terms: Option<bool>,
//
//     /// Whether to enable position increments in result
//     #[builder(default)]
//     pub enable_position_increments: Option<bool>,
//
//     /// Fuzzy max expansions
//     #[builder(default)]
//     pub fuzzy_max_expansions: Option<i32>,
//
//     /// Fuzziness parameter
//     #[builder(default)]
//     pub fuzziness: Option<Fuzziness>,
//
//     /// Fuzzy prefix length
//     #[builder(default)]
//     pub fuzzy_prefix_length: Option<i32>,
//
//     /// Fuzzy rewrite method
//     #[builder(default)]
//     pub fuzzy_rewrite: Option<String>,
//
//     /// Phrase slop
//     #[builder(default)]
//     pub phrase_slop: Option<i32>,
//
//     /// Boost value
//     #[builder(default)]
//     pub boost: Option<f64>,
//
//     /// Whether to enable auto generate phrase queries
//     #[builder(default)]
//     pub auto_generate_phrase_queries: Option<bool>,
//
//     /// Allow leading wildcard flag
//     #[builder(default)]
//     pub allow_leading_wildcard: Option<bool>,
//
//     /// Maximum number of terms that can be created by wildcard or prefix expansion
//     #[builder(default)]
//     pub max_determinized_states: Option<i32>,
//
//     /// Minimum should match parameter
//     #[builder(default)]
//     pub minimum_should_match: Option<MinimumShouldMatch>,
//
//     /// Lenient flag to ignore format based failures
//     #[builder(default)]
//     pub lenient: Option<bool>,
//
//     /// Time zone for date values
//     #[builder(default)]
//     pub time_zone: Option<String>,
//
//     /// How scores from different queries are combined
//     #[builder(default)]
//     pub type_: Option<QueryStringType>,
// }
//
// impl QueryStringQuery {
//     /// Create a new query string query builder with the given query string
//     pub fn query(query: impl Into<String>) -> QueryStringQueryBuilder {
//         let mut builder = QueryStringQueryBuilder::default();
//         builder.query(query.into());
//         builder
//     }
//
//     /// Create a new query string query builder
//     pub fn builder() -> QueryStringQueryBuilder {
//         QueryStringQueryBuilder::default()
//     }
// }
//
// impl QueryStringQueryBuilder {
//     /// Build the final query string query object
//     pub fn build_query(&self) -> Result<query::Query, String> {
//         let query_builder = self.clone();
//         let builder_query = query_builder
//             .build()
//             .map_err(|e| format!("Failed to build query string query: {}", e))?;
//
//         // Create the QueryStringQuery from query.rs
//         let query_string_query = query::QueryStringQuery {
//             query: builder_query.query,
//             default_field: builder_query.default_field,
//             fields: builder_query.fields,
//             default_operator: builder_query.default_operator,
//             analyzer: builder_query.analyzer,
//             analyze_wildcard: builder_query.analyze_wildcard,
//             lowercase_expanded_terms: builder_query.lowercase_expanded_terms,
//             enable_position_increments: builder_query.enable_position_increments,
//             fuzzy_max_expansions: builder_query.fuzzy_max_expansions,
//             fuzziness: builder_query.fuzziness,
//             fuzzy_prefix_length: builder_query.fuzzy_prefix_length,
//             fuzzy_rewrite: builder_query.fuzzy_rewrite,
//             phrase_slop: builder_query.phrase_slop,
//             boost: builder_query.boost,
//             auto_generate_phrase_queries: builder_query.auto_generate_phrase_queries,
//             allow_leading_wildcard: builder_query.allow_leading_wildcard,
//             max_determinized_states: builder_query.max_determinized_states,
//             minimum_should_match: builder_query.minimum_should_match,
//             lenient: builder_query.lenient,
//             time_zone: builder_query.time_zone,
//             type_: builder_query.type_,
//         };
//
//         // Create the query with QueryType set to QueryString
//         Ok(Query::QueryString(query_string_query))
//     }
// }
//
// /// Main query DSL entry point
// pub struct QueryDsl;
//
// impl QueryDsl {
//     /// Create a match query builder
//     pub fn match_query(field: impl Into<String>) -> MatchQueryRulesBuilder {
//         let mut builder = MatchQueryRulesBuilder::default();
//         builder.field(field.into());
//         builder
//     }
//
//     /// Create a term query builder
//     pub fn term(field: impl Into<String>) -> TermQueryRulesBuilder {
//         let mut builder = TermQueryRulesBuilder::default();
//         builder.field(field.into());
//         builder
//     }
//
//     /// Create a range query builder
//     pub fn range(field: impl Into<String>) -> RangeQueryBuilder {
//         RangeQuery::field(field)
//     }
//
//     /// Create a bool query builder
//     pub fn bool() -> BoolQueryBuilder {
//         BoolQuery::new()
//     }
//
//     /// Create a query string query builder
//     pub fn query_string(query: impl Into<String>) -> QueryStringQueryBuilder {
//         QueryStringQuery::query(query)
//     }
//
//     /// Create a match all query
//     pub fn match_all(boost: Option<f64>) -> Query {
//         Query::match_all(boost)
//     }
//
//     /// Create a match none query
//     pub fn match_none() -> Query {
//         Query::match_none()
//     }
// }
