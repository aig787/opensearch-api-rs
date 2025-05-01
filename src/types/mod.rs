//! Common data types used across the OpenSearch API

pub mod aggregations;
pub mod builder;
pub mod common;
pub mod document;
pub mod query;
pub mod script;
pub mod search;

pub use aggregations::*;
pub use builder::QueryDsl;
pub use common::*;
pub use document::*;
pub use query::*;

/// Statistics about shards
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct ShardStatistics {
    /// Total number of shards
    pub total: u32,

    /// Number of successful shards
    pub successful: u32,

    /// Number of failed shards
    pub failed: u32,

    /// Number of skipped shards
    #[serde(default)]
    pub skipped: u32,
}

/// Common parameters shared across API calls
#[derive(Debug, Clone, Default)]
pub struct CommonParameters {
    /// Request timeout in milliseconds
    pub timeout: Option<String>,

    /// Master timeout for the operation
    pub master_timeout: Option<String>,

    /// Pretty format the returned JSON response
    pub pretty: Option<bool>,

    /// Human-readable output for statistics
    pub human: Option<bool>,

    /// Whether specified concrete indices should be ignored when unavailable
    pub ignore_unavailable: Option<bool>,

    /// Whether to expand wildcard expressions to concrete indices
    pub expand_wildcards: Option<ExpandWildcards>,
}

/// Options for expanding wildcard expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpandWildcards {
    Open,
    Closed,
    Hidden,
    None,
    All,
}

impl ToString for ExpandWildcards {
    fn to_string(&self) -> String {
        match self {
            ExpandWildcards::Open => "open".to_string(),
            ExpandWildcards::Closed => "closed".to_string(),
            ExpandWildcards::Hidden => "hidden".to_string(),
            ExpandWildcards::None => "none".to_string(),
            ExpandWildcards::All => "all".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    //! Test suite for serializing and deserializing models in the types directory

    use serde_json::{json, Value};

    /// Helper function to test serialization and deserialization roundtrip
    fn test_serde_roundtrip<T>(value: &T, expected_json: &str)
    where
        T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
    {
        // Serialize to string
        let serialized = serde_json::to_string(&value).expect("Failed to serialize");

        // Parse both as Value for comparison that ignores formatting/whitespace
        let value_json: Value =
            serde_json::from_str(&serialized).expect("Failed to parse serialized");
        let expected_value: Value =
            serde_json::from_str(expected_json).expect("Failed to parse expected");

        assert_eq!(
            value_json, expected_value,
            "Serialized JSON doesn't match expected JSON"
        );

        // Deserialize back
        let deserialized: T = serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Verify roundtrip
        assert_eq!(
            &deserialized, value,
            "Deserialized value doesn't match original"
        );
    }

    #[cfg(test)]
    mod builder_tests {
        use super::*;
        use crate::builder::*;
        use crate::{Fuzziness, Operator};

        #[test]
        fn test_match_query_simple() {
            let query = MatchQuery::builder()
                .field("title".to_string())
                .query("elasticsearch".to_string())
                .build()
                .unwrap();

            let expected_json = r#"{"field":"title","query":"elasticsearch"}"#;
            test_serde_roundtrip(&query, expected_json);
        }

        #[test]
        fn test_match_query_complete() {
            let query = MatchQuery::builder()
                .field("title".to_string())
                .query("elasticsearch".to_string())
                .operator(Operator::And)
                .fuzziness(Fuzziness::Auto)
                .prefix_length(2)
                .max_expansions(50)
                .boost(1.5)
                .analyzer("standard".to_string())
                .build()
                .unwrap();

            let expected_json = r#"{
            "field": "title",
            "query": "elasticsearch",
            "operator": "and",
            "fuzziness": "auto",
            "prefix_length": 2,
            "max_expansions": 50,
            "boost": 1.5,
            "analyzer": "standard"
        }"#;
            test_serde_roundtrip(&query, expected_json);
        }

        #[test]
        fn test_term_query_simple() {
            let query = TermQuery::builder()
                .field("status".to_string())
                .value(json!("active"))
                .build()
                .unwrap();

            let expected_json = r#"{"field":"status","value":"active"}"#;
            test_serde_roundtrip(&query, expected_json);
        }

        #[test]
        fn test_term_query_complete() {
            let query = TermQuery::builder()
                .field("status".to_string())
                .value(json!("active"))
                .boost(2.0)
                .case_insensitive(true)
                .build()
                .unwrap();

            let expected_json = r#"{
            "field": "status",
            "value": "active",
            "boost": 2.0,
            "case_insensitive": true
        }"#;
            test_serde_roundtrip(&query, expected_json);
        }

        #[test]
        fn test_range_query() {
            let query = RangeQuery::field("age")
                .gte(json!(25))
                .lt(json!(50))
                .boost(1.2)
                .build()
                .unwrap();

            let expected_json = r#"{
            "field": "age",
            "gte": 25,
            "lt": 50,
            "boost": 1.2
        }"#;
            test_serde_roundtrip(&query, expected_json);
        }

        #[test]
        fn test_bool_query() {
            let mut bool_builder = BoolQuery::new();

            let must_query = MatchQuery::builder()
                .field("title".to_string())
                .query("elasticsearch".to_string())
                .build_query()
                .unwrap();

            let should_query = TermQuery::builder().field("status".to_string())
                .value(json!("active"))
                .build_query()
                .unwrap();

            bool_builder.add_must(must_query).add_should(should_query);

            let bool_query = bool_builder.build().unwrap();

            let expected_json = r#"{
            "must": [
                {
                    "match": {
                        "title": "elasticsearch"
                    }
                }
            ],
            "should": [
                {
                    "term": {
                        "status": "active"
                    }
                }
            ]
        }"#;
            test_serde_roundtrip(&bool_query, expected_json);
        }

        #[test]
        fn test_query_string_query() {
            let query = QueryStringQuery::query("elasticsearch and (server or cloud)")
                .default_field("content".to_string())
                .default_operator(Operator::And)
                .build()
                .unwrap();

            let expected_json = r#"{
            "query": "elasticsearch and (server or cloud)",
            "default_field": "content",
            "default_operator": "and"
        }"#;
            test_serde_roundtrip(&query, expected_json);
        }

        #[test]
        fn test_match_all_query() {
            let query = MatchAllQuery::builder().boost(1.2).build().unwrap();

            let expected_json = r#"{"boost":1.2}"#;
            test_serde_roundtrip(&query, expected_json);
        }

        #[test]
        fn test_match_none_query() {
            let query = MatchNoneQuery {};

            let expected_json = r#"{}"#;
            test_serde_roundtrip(&query, expected_json);
        }
    }

    #[cfg(test)]
    mod search_tests {
        use std::collections::HashMap;
        use crate::search::{Aggregation, Bucket, HighlightField, HighlightOptions, HighlighterType, SearchHit, SearchHits, SearchResponse, TotalHits, TotalHitsRelation};
        use crate::ShardStatistics;
        use super::*;

        #[test]
        fn test_search_response() {
            let search_response: SearchResponse = SearchResponse {
                took: 42,
                timed_out: false,
                _shards: ShardStatistics {
                    total: 5,
                    successful: 5,
                    failed: 0,
                    skipped: 0,
                },
                hits: SearchHits {
                    total: TotalHits {
                        value: 100,
                        relation: TotalHitsRelation::Equal,
                    },
                    max_score: Some(1.0),
                    hits: vec![
                        SearchHit {
                            index: "test-index".to_string(),
                            id: "1".to_string(),
                            score: Some(1.0),
                            source: Some(json!({
                                "field1": "value1",
                                "field2": 42
                            })),
                            fields: None,
                            highlight: None,
                            inner_hits: None,
                            sort: None,
                        }
                    ],
                },
                aggregations: None,
                suggest: None,
                profile: None,
                scroll_id: None,
            };

            let expected_json = r#"{
                "took": 42,
                "timed_out": false,
                "_shards": {
                    "total": 5,
                    "successful": 5,
                    "failed": 0,
                    "skipped": 0
                },
                "hits": {
                    "total": {
                        "value": 100,
                        "relation": "eq"
                    },
                    "max_score": 1.0,
                    "hits": [
                        {
                            "_index": "test-index",
                            "_id": "1",
                            "_score": 1.0,
                            "_source": {
                                "field1": "value1",
                                "field2": 42
                            }
                        }
                    ]
                }
            }"#;

            test_serde_roundtrip(&search_response, expected_json);
        }

        #[test]
        fn test_total_hits() {
            let exact_total = TotalHits {
                value: 42,
                relation: TotalHitsRelation::Equal,
            };

            let expected_json = r#"{"value":42,"relation":"eq"}"#;
            test_serde_roundtrip(&exact_total, expected_json);

            let greater_than_total = TotalHits {
                value: 10000,
                relation: TotalHitsRelation::GreaterThanOrEqual,
            };

            let expected_json = r#"{"value":10000,"relation":"gte"}"#;
            test_serde_roundtrip(&greater_than_total, expected_json);
        }

        #[test]
        fn test_aggregation_buckets() {
            let agg = Aggregation::Buckets {
                doc_count_error_upper_bound: Some(0),
                sum_other_doc_count: Some(0),
                buckets: vec![
                    Bucket {
                        key: json!("term1"),
                        key_as_string: None,
                        doc_count: 42,
                        aggregations: HashMap::new(),
                    },
                    Bucket {
                        key: json!("term2"),
                        key_as_string: None,
                        doc_count: 21,
                        aggregations: HashMap::new(),
                    }
                ],
            };

            let expected_json = r#"{
                "doc_count_error_upper_bound": 0,
                "sum_other_doc_count": 0,
                "buckets": [
                    {
                        "key": "term1",
                        "doc_count": 42
                    },
                    {
                        "key": "term2",
                        "doc_count": 21
                    }
                ]
            }"#;

            test_serde_roundtrip(&agg, expected_json);
        }

        #[test]
        fn test_highlight_options() {
            let mut fields = HashMap::new();
            fields.insert("content".to_string(), HighlightField::Config {
                type_: Some(HighlighterType::Plain),
                fragment_size: Some(150),
                number_of_fragments: Some(3),
                fragment_offset: None,
                matched_fields: None,
                pre_tags: None,
                post_tags: None,
                highlight_query: None,
            });

            let highlight_options = HighlightOptions {
                fields,
                type_: Some(HighlighterType::Unified),
                pre_tags: Some(vec!["<em>".to_string()]),
                post_tags: Some(vec!["</em>".to_string()]),
                require_field_match: Some(false),
                fragment_size: Some(100),
                number_of_fragments: Some(5),
                order: None,
                encoder: None,
            };

            let expected_json = r#"{
                "fields": {
                    "content": {
                        "type": "plain",
                        "fragment_size": 150,
                        "number_of_fragments": 3
                    }
                },
                "type": "unified",
                "pre_tags": ["<em>"],
                "post_tags": ["</em>"],
                "require_field_match": false,
                "fragment_size": 100,
                "number_of_fragments": 5
            }"#;

            test_serde_roundtrip(&highlight_options, expected_json);
        }

        #[test]
        fn test_search_hit() {
            let hit: SearchHit = SearchHit {
                index: "test-index".to_string(),
                id: "123".to_string(),
                score: Some(0.8),
                source: Some(json!({
                    "title": "Test Document",
                    "content": "This is a test document"
                })),
                fields: None,
                highlight: Some(HashMap::from([
                    ("content".to_string(), vec!["This is a <em>test</em> document".to_string()])
                ])),
                inner_hits: None,
                sort: Some(vec![json!(1)]),
            };

            let expected_json = r#"{
                "_index": "test-index",
                "_id": "123",
                "_score": 0.8,
                "_source": {
                    "title": "Test Document",
                    "content": "This is a test document"
                },
                "highlight": {
                    "content": ["This is a <em>test</em> document"]
                },
                "sort": [1]
            }"#;

            test_serde_roundtrip(&hit, expected_json);
        }
    }

    #[cfg(test)]
    mod document_tests {
        use crate::{BulkResponse, BulkResponseItem, DeleteResponse, DocumentMetadata, GetResponse, IndexResponse, ShardStatistics, WaitForActiveShards};
        use super::*;

        #[test]
        fn test_document_metadata() {
            let metadata = DocumentMetadata {
                index: "test-index".to_string(),
                id: "123".to_string(),
                version: Some(1),
                seq_no: Some(42),
                primary_term: Some(1),
            };

            let expected_json = r#"{
                "_index": "test-index",
                "_id": "123",
                "_version": 1,
                "_seq_no": 42,
                "_primary_term": 1
            }"#;

            test_serde_roundtrip(&metadata, expected_json);
        }

        #[test]
        fn test_wait_for_active_shards() {
            let count = WaitForActiveShards::Count(2);
            let expected_json = "2";
            test_serde_roundtrip(&count, expected_json);

            let all = WaitForActiveShards::Value("all".to_string());
            let expected_json = r#""all""#;
            test_serde_roundtrip(&all, expected_json);
        }

        #[test]
        fn test_index_response() {
            let response = IndexResponse {
                index: "test-index".to_string(),
                id: "123".to_string(),
                version: 1,
                result: "created".to_string(),
                _shards: ShardStatistics {
                    total: 2,
                    successful: 2,
                    failed: 0,
                    skipped: 0,
                },
                seq_no: 0,
                primary_term: 1,
            };

            let expected_json = r#"{
                "_index": "test-index",
                "_id": "123",
                "_version": 1,
                "result": "created",
                "_shards": {
                    "total": 2,
                    "successful": 2,
                    "failed": 0,
                    "skipped": 0
                },
                "_seq_no": 0,
                "_primary_term": 1
            }"#;

            test_serde_roundtrip(&response, expected_json);
        }

        #[test]
        fn test_get_response() {
            let response: GetResponse<serde_json::Value> = GetResponse {
                index: "test-index".to_string(),
                id: "123".to_string(),
                found: true,
                version: Some(1),
                seq_no: Some(42),
                primary_term: Some(1),
                source: Some(json!({
                    "title": "Test Document",
                    "content": "This is a test document"
                })),
                fields: None,
            };

            let expected_json = r#"{
                "_index": "test-index",
                "_id": "123",
                "found": true,
                "_version": 1,
                "_seq_no": 42,
                "_primary_term": 1,
                "_source": {
                    "title": "Test Document",
                    "content": "This is a test document"
                }
            }"#;

            test_serde_roundtrip(&response, expected_json);
        }

        #[test]
        fn test_delete_response() {
            let response = DeleteResponse {
                index: "test-index".to_string(),
                id: "123".to_string(),
                version: 2,
                result: "deleted".to_string(),
                _shards: ShardStatistics {
                    total: 2,
                    successful: 2,
                    failed: 0,
                    skipped: 0,
                },
                seq_no: 43,
                primary_term: 1,
            };

            let expected_json = r#"{
                "_index": "test-index",
                "_id": "123",
                "_version": 2,
                "result": "deleted",
                "_shards": {
                    "total": 2,
                    "successful": 2,
                    "failed": 0,
                    "skipped": 0
                },
                "_seq_no": 43,
                "_primary_term": 1
            }"#;

            test_serde_roundtrip(&response, expected_json);
        }

        #[test]
        fn test_bulk_response() {
            let index_response = IndexResponse {
                index: "test-index".to_string(),
                id: "1".to_string(),
                version: 1,
                result: "created".to_string(),
                _shards: ShardStatistics {
                    total: 2,
                    successful: 2,
                    failed: 0,
                    skipped: 0,
                },
                seq_no: 0,
                primary_term: 1,
            };

            let delete_response = DeleteResponse {
                index: "test-index".to_string(),
                id: "2".to_string(),
                version: 1,
                result: "deleted".to_string(),
                _shards: ShardStatistics {
                    total: 2,
                    successful: 2,
                    failed: 0,
                    skipped: 0,
                },
                seq_no: 1,
                primary_term: 1,
            };

            let bulk_response = BulkResponse {
                took: 10,
                timed_out: false,
                items: vec![
                    BulkResponseItem {
                        index: Some(index_response),
                        create: None,
                        update: None,
                        delete: None,
                    },
                    BulkResponseItem {
                        index: None,
                        create: None,
                        update: None,
                        delete: Some(delete_response),
                    },
                ],
                _shards: ShardStatistics {
                    total: 2,
                    successful: 2,
                    failed: 0,
                    skipped: 0,
                },
            };

            let expected_json = r#"{
                "took": 10,
                "timed_out": false,
                "items": [
                    {
                        "index": {
                            "_index": "test-index",
                            "_id": "1",
                            "_version": 1,
                            "result": "created",
                            "_shards": {
                                "total": 2,
                                "successful": 2,
                                "failed": 0,
                                "skipped": 0
                            },
                            "_seq_no": 0,
                            "_primary_term": 1
                        }
                    },
                    {
                        "delete": {
                            "_index": "test-index",
                            "_id": "2",
                            "_version": 1,
                            "result": "deleted",
                            "_shards": {
                                "total": 2,
                                "successful": 2,
                                "failed": 0,
                                "skipped": 0
                            },
                            "_seq_no": 1,
                            "_primary_term": 1
                        }
                    }
                ],
                "_shards": {
                    "total": 2,
                    "successful": 2,
                    "failed": 0,
                    "skipped": 0
                }
            }"#;

            test_serde_roundtrip(&bulk_response, expected_json);
        }
    }
}
