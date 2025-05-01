//! Common data types used across the OpenSearch API
//! Type definitions for the OpenSearch client

pub mod aggregations;
pub mod builder;
pub mod common;
pub mod document;
pub mod indices;
pub mod query;
pub mod script;
pub mod search;

#[cfg(test)]
mod tests {
    //! Test suite for serializing and deserializing models in the types directory

    use crate::Error;
    use serde_json::Value;

    /// Helper function to test serialization and deserialization roundtrip
    fn test_serde_roundtrip<T>(value: &T, expected_json: &str) -> Result<(), Error>
    where
        T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
    {
        // Serialize to string
        let serialized = serde_json::to_string(&value)?;

        // Parse both as Value for comparison that ignores formatting/whitespace
        let value_json: Value = serde_json::from_str(&serialized)?;
        let expected_value: Value = serde_json::from_str(expected_json)?;

        assert_eq!(
            value_json, expected_value,
            "Serialized JSON doesn't match expected JSON"
        );

        // Deserialize back
        let deserialized: T = serde_json::from_str(&serialized)?;

        // Verify roundtrip
        assert_eq!(
            &deserialized, value,
            "Deserialized value doesn't match original"
        );

        Ok(())
    }

    #[cfg(test)]
    mod query_tests {
        use super::*;
        use crate::types::query::*;
        use serde_json::json;

        #[test]
        fn test_match_query_simple() -> Result<(), Error> {
            let query = MatchQuery {
                match_: vec![(
                    "title".to_string(),
                    MatchQueryRule::Simple("opensearch".to_string()),
                )]
                .into_iter()
                .collect(),
            };

            let expected_json = r#"{"match":{"title":"opensearch"}}"#;
            test_serde_roundtrip(&query, expected_json)
        }

        #[test]
        fn test_match_query_complete() -> Result<(), Error> {
            let query = MatchQuery {
                match_: vec![(
                    "title".to_string(),
                    MatchQueryRule::Advanced(MatchQueryRuleAdvanced {
                        query: "opensearch".to_string(),
                        operator: Some(Operator::And),
                        analyzer: Some("standard".to_string()),
                        minimum_should_match: None,
                        fuzziness: Some(Fuzziness::Auto),
                        prefix_length: Some(2),
                        max_expansions: Some(50),
                        boost: Some(1.5),
                        auto_generate_synonyms_phrase_query: None,
                        enable_position_increments: None,
                        fuzzy_rewrite: None,
                        fuzzy_transpositions: None,
                        lenient: None,
                        zero_terms_query: None,
                    }),
                )]
                .into_iter()
                .collect(),
            };

            let expected_json = r#"{
                    "match": {
                        "title": {
                            "query": "opensearch",
                            "operator": "and",
                            "fuzziness": "auto",
                            "prefix_length": 2,
                            "max_expansions": 50,
                            "boost": 1.5,
                            "analyzer": "standard"
                        }
                    }
                }"#;
            test_serde_roundtrip(&query, expected_json)
        }

        #[test]
        fn test_term_query_simple() -> Result<(), Error> {
            let query = TermQuery {
                term: vec![(
                    "status".to_string(),
                    TermQueryRule {
                        value: json!("active"),
                        case_insensitive: None,
                        boost: None,
                    },
                )]
                .into_iter()
                .collect(),
            };

            let expected_json = r#"{"term": {"status": {"value": "active"}}}"#;
            test_serde_roundtrip(&query, expected_json)
        }

        #[test]
        fn test_term_query_complete() -> Result<(), Error> {
            let query = TermQuery {
                term: vec![(
                    "status".to_string(),
                    TermQueryRule {
                        value: json!("active"),
                        boost: Some(2.0),
                        case_insensitive: Some(true),
                    },
                )]
                .into_iter()
                .collect(),
            };

            let expected_json = r#"{
                "term": {
                    "status": {
                        "value": "active",
                        "boost": 2.0,
                        "case_insensitive": true
                    }
                }
            }"#;
            test_serde_roundtrip(&query, expected_json)
        }

        #[test]
        fn test_range_query() -> Result<(), Error> {
            let query = RangeQuery {
                range: vec![(
                    "age".to_string(),
                    RangeQueryRule {
                        gt: None,
                        gte: Some(json!(25)),
                        lt: Some(json!(50)),
                        format: None,
                        relation: None,
                        time_zone: None,
                        boost: Some(1.2),
                        lte: None,
                    },
                )]
                .into_iter()
                .collect(),
            };

            let expected_json = r#"{
                "range": {
                    "age": {
                        "gte": 25,
                        "lt": 50,
                        "boost": 1.2
                    }
                }
            }"#;
            test_serde_roundtrip(&query, expected_json)
        }

        #[test]
        fn test_bool_query() -> Result<(), Error> {
            let must_query = Query::Match(MatchQuery {
                match_: vec![(
                    "title".to_string(),
                    MatchQueryRule::Simple("elasticsearch".to_string()),
                )]
                .into_iter()
                .collect(),
            });

            let should_query = Query::Term(TermQuery {
                term: vec![("status".to_string(), TermQueryRule::value("active"))]
                    .into_iter()
                    .collect(),
            });

            let bool_query = BoolQuery {
                bool: BoolQueryRule {
                    must: Some(vec![must_query]),
                    should: Some(vec![should_query]),
                    must_not: None,
                    filter: None,
                    minimum_should_match: None,
                    boost: None,
                },
            };

            let expected_json = r#"{
                "bool": {
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
                                "status": {
                                    "value": "active"
                                }
                            }
                        }
                    ]
                }
            }"#;
            test_serde_roundtrip(&bool_query, expected_json)
        }

        #[test]
        fn test_query_string_query() -> Result<(), Error> {
            let query = QueryStringQuery {
                query_string: QueryStringQueryRule {
                    query: "opensearch and (server or cloud)".to_string(),
                    default_field: Some("content".to_string()),
                    default_operator: Some(Operator::And),
                    analyzer: None,
                    allow_leading_wildcard: None,
                    enable_position_increments: None,
                    fuzzy_max_expansions: None,
                    fuzziness: None,
                    fuzzy_prefix_length: None,
                    lenient: None,
                    max_determinized_states: None,
                    minimum_should_match: None,
                    phrase_slop: None,
                    time_zone: None,
                    boost: None,
                    analyze_wildcard: None,
                    fields: None,
                    lowercase_expanded_terms: None,
                    fuzzy_rewrite: None,
                    auto_generate_phrase_queries: None,
                    type_: None,
                },
            };

            let expected_json = r#"{
                "query_string": {
                    "query": "opensearch and (server or cloud)",
                    "default_field": "content",
                    "default_operator": "and"
                }
            }"#;
            test_serde_roundtrip(&query, expected_json)
        }

        #[test]
        fn test_match_all_query() -> Result<(), Error> {
            let query = MatchAllQuery {
                match_all: MatchAllQueryRule { boost: Some(1.2) },
            };

            let expected_json = r#"{"match_all": {"boost":1.2}}"#;
            test_serde_roundtrip(&query, expected_json)
        }

        #[test]
        fn test_match_none_query() -> Result<(), Error> {
            let query = MatchNoneQuery {
                match_none: Default::default(),
            };

            let expected_json = r#"{"match_none": {}}"#;
            test_serde_roundtrip(&query, expected_json)
        }
    }

    #[cfg(test)]
    mod search_tests {
        use super::*;
        use crate::types::common::ShardStatistics;
        use crate::types::search::*;
        use serde_json::json;
        use std::collections::HashMap;

        #[test]
        fn test_search_response() -> Result<(), Error> {
            let search_response: SearchResponse = SearchResponse {
                took: 42,
                timed_out: false,
                _shards: ShardStatistics {
                    total: 5,
                    successful: 5,
                    failed: 0,
                    failures: vec![],
                },
                hits: SearchHits {
                    total: TotalHits {
                        value: 100,
                        relation: TotalHitsRelation::Equal,
                    },
                    max_score: Some(1.0),
                    hits: vec![SearchHit {
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
                    }],
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
                        "failures": []
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

            test_serde_roundtrip(&search_response, expected_json)
        }

        #[test]
        fn test_total_hits() -> Result<(), Error> {
            let exact_total = TotalHits {
                value: 42,
                relation: TotalHitsRelation::Equal,
            };

            let expected_json = r#"{"value":42,"relation":"eq"}"#;
            test_serde_roundtrip(&exact_total, expected_json)?;

            let greater_than_total = TotalHits {
                value: 10000,
                relation: TotalHitsRelation::GreaterThanOrEqual,
            };

            let expected_json = r#"{"value":10000,"relation":"gte"}"#;
            test_serde_roundtrip(&greater_than_total, expected_json)
        }

        #[test]
        fn test_aggregation_buckets() -> Result<(), Error> {
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
                    },
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

            test_serde_roundtrip(&agg, expected_json)
        }

        #[test]
        fn test_highlight_options() -> Result<(), Error> {
            let mut fields = HashMap::new();
            fields.insert(
                "content".to_string(),
                HighlightField::Config {
                    type_: Some(HighlighterType::Plain),
                    fragment_size: Some(150),
                    number_of_fragments: Some(3),
                    fragment_offset: None,
                    matched_fields: None,
                    pre_tags: None,
                    post_tags: None,
                    highlight_query: None,
                },
            );

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

            test_serde_roundtrip(&highlight_options, expected_json)
        }

        #[test]
        fn test_search_hit() -> Result<(), Error> {
            let hit: SearchHit = SearchHit {
                index: "test-index".to_string(),
                id: "123".to_string(),
                score: Some(0.8),
                source: Some(json!({
                    "title": "Test Document",
                    "content": "This is a test document"
                })),
                fields: None,
                highlight: Some(HashMap::from([(
                    "content".to_string(),
                    vec!["This is a <em>test</em> document".to_string()],
                )])),
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

            test_serde_roundtrip(&hit, expected_json)
        }
    }

    #[cfg(test)]
    mod document_tests {
        use super::*;
        use crate::types::common::ShardStatistics;
        use crate::types::document::*;
        use serde_json::json;

        #[test]
        fn test_document_metadata() -> Result<(), Error> {
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

            test_serde_roundtrip(&metadata, expected_json)
        }

        #[test]
        fn test_wait_for_active_shards() -> Result<(), Error> {
            let count = WaitForActiveShards::Count(2);
            let expected_json = "2";
            test_serde_roundtrip(&count, expected_json)?;

            let all = WaitForActiveShards::Value("all".to_string());
            let expected_json = r#""all""#;
            test_serde_roundtrip(&all, expected_json)
        }

        #[test]
        fn test_index_response() -> Result<(), Error> {
            let response = IndexResponse {
                index: "test-index".to_string(),
                id: "123".to_string(),
                version: 1,
                result: "created".to_string(),
                _shards: ShardStatistics {
                    total: 2,
                    successful: 2,
                    failed: 0,
                    failures: vec![],
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
                        "failures": []
                    },
                    "_seq_no": 0,
                    "_primary_term": 1
                }"#;

            test_serde_roundtrip(&response, expected_json)
        }

        #[test]
        fn test_get_response() -> Result<(), Error> {
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

            test_serde_roundtrip(&response, expected_json)
        }

        #[test]
        fn test_delete_response() -> Result<(), Error> {
            let response = DeleteResponse {
                index: "test-index".to_string(),
                id: "123".to_string(),
                version: 2,
                result: "deleted".to_string(),
                _shards: ShardStatistics {
                    total: 2,
                    successful: 2,
                    failed: 0,
                    failures: vec![],
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
                        "failures": []
                    },
                    "_seq_no": 43,
                    "_primary_term": 1
                }"#;

            test_serde_roundtrip(&response, expected_json)
        }

        #[test]
        fn test_bulk_response() -> Result<(), Error> {
            let index_response = IndexResponse {
                index: "test-index".to_string(),
                id: "1".to_string(),
                version: 1,
                result: "created".to_string(),
                _shards: ShardStatistics {
                    total: 2,
                    successful: 2,
                    failed: 0,
                    failures: vec![],
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
                    failures: vec![],
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
                    failures: vec![],
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
                                    "failures": []
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
                                    "failures": []
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
                        "failures": []
                    }
                }"#;

            test_serde_roundtrip(&bulk_response, expected_json)
        }
    }
}
