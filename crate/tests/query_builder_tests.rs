use anyhow::Error;
use opensearch_api::types::query::*;
use serde_json::json;

#[test]
fn test_match_query_builder_simple() -> Result<(), Error> {
    let query = MatchQuery::builder()
        .field(
            "title".to_string(),
            MatchQueryRule::Simple("search text".to_string()),
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "match": {
            "title": "search text"
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_match_query_builder_complete() -> Result<(), Error> {
    let query = MatchQuery::builder()
        .field(
            "title".to_string(),
            MatchQueryRule::advanced()
                .query("search text")
                .analyzer("standard")
                .auto_generate_synonyms_phrase_query(true)
                .fuzziness(Fuzziness::Auto)
                .max_expansions(10)
                .prefix_length(2)
                .fuzzy_transpositions(true)
                .fuzzy_rewrite("constant_score")
                .lenient(true)
                .operator(Operator::And)
                .minimum_should_match(MinimumShouldMatch::Complex("75%".to_string()))
                .zero_terms_query(ZeroTermsQuery::All)
                .boost(1.5)
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "match": {
            "title": {
                "query": "search text",
                "analyzer": "standard",
                "auto_generate_synonyms_phrase_query": true,
                "fuzziness": "auto",
                "max_expansions": 10,
                "prefix_length": 2,
                "fuzzy_transpositions": true,
                "fuzzy_rewrite": "constant_score",
                "lenient": true,
                "operator": "and",
                "minimum_should_match": "75%",
                "zero_terms_query": "all",
                "boost": 1.5
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_term_query_builder_simple() -> Result<(), Error> {
    let query = TermQuery::builder()
        .field("status".to_string(), TermQueryRule::value(json!("active")))
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "term": {
            "status": {
                "value": "active"
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_terms_query_builder_simple() -> Result<(), Error> {
    let query = TermsQuery::builder()
        .field(
            "status".to_string(),
            TermsQueryRule::simple(vec![json!("active"), json!("pending")]),
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "terms": {
            "status": ["active", "pending"]
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_terms_query_builder_with_boost() -> Result<(), Error> {
    let query = TermsQuery::builder()
        .field(
            "status".to_string(),
            TermsQueryRule::advanced()
                .values(vec![json!("active"), json!("pending")])
                .boost(1.5)
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "terms": {
            "status": {
                "values": ["active", "pending"],
                "boost": 1.5
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_range_query_builder() -> Result<(), Error> {
    let query = RangeQuery::builder()
        .field(
            "age".to_string(),
            RangeQueryRule::builder()
                .gte(json!(18))
                .lt(json!(65))
                .boost(1.2)
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "range": {
            "age": {
                "gte": 18,
                "lt": 65,
                "boost": 1.2
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_range_query_builder_date() -> Result<(), Error> {
    let query = RangeQuery::builder()
        .field(
            "created_at".to_string(),
            RangeQueryRule::builder()
                .gte(json!("2020-01-01"))
                .lte(json!("2020-12-31"))
                .format("yyyy-MM-dd")
                .time_zone("UTC")
                .relation(RangeRelation::Within)
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "range": {
            "created_at": {
                "gte": "2020-01-01",
                "lte": "2020-12-31",
                "format": "yyyy-MM-dd",
                "time_zone": "UTC",
                "relation": "within"
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_bool_query_builder() -> Result<(), Error> {
    let term_query = TermQuery::builder()
        .field("status".to_string(), TermQueryRule::value(json!("active")))
        .build()?;

    let range_query = RangeQuery::builder()
        .field(
            "age".to_string(),
            RangeQueryRule::builder()
                .gte(json!(18))
                .lt(json!(65))
                .build()?,
        )
        .build()?;

    let query = BoolQuery::builder()
        .must(vec![term_query.into()])
        .filter(vec![range_query.into()])
        .minimum_should_match(MinimumShouldMatch::Absolute(1))
        .boost(1.5)
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "bool": {
            "must": [
                {
                    "term": {
                        "status": {
                            "value": "active"
                        }
                    }
                }
            ],
            "filter": [
                {
                    "range": {
                        "age": {
                            "gte": 18,
                            "lt": 65
                        }
                    }
                }
            ],
            "minimum_should_match": 1,
            "boost": 1.5
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_exists_query_builder() -> Result<(), Error> {
    let query = ExistsQuery::builder()
        .field("email".to_string())
        .boost(1.2)
        .build()?;

    assert_eq!(query.exists.field, "email");
    assert_eq!(query.exists.boost, Some(1.2));

    let json = Query::from(query).json()?;
    let expected = json!({
        "exists": {
            "field": "email",
            "boost": 1.2
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_query_string_query_builder() -> Result<(), Error> {
    let query = QueryStringQuery::builder()
        .query("title:elasticsearch or description:search".to_string())
        .default_field("content".to_string())
        .fields(vec!["title".to_string(), "description".to_string()])
        .default_operator(Operator::And)
        .analyzer("standard".to_string())
        .analyze_wildcard(true)
        .lenient(true)
        .boost(1.5)
        .type_(QueryStringType::BestFields)
        .build()?;

    assert_eq!(
        query.query_string.query,
        "title:elasticsearch or description:search"
    );
    assert_eq!(
        query.query_string.default_field,
        Some("content".to_string())
    );
    assert_eq!(
        query.query_string.fields,
        Some(vec!["title".to_string(), "description".to_string()])
    );
    assert_eq!(query.query_string.default_operator, Some(Operator::And));
    assert_eq!(query.query_string.analyzer, Some("standard".to_string()));
    assert_eq!(query.query_string.analyze_wildcard, Some(true));
    assert_eq!(query.query_string.lenient, Some(true));
    assert_eq!(query.query_string.boost, Some(1.5));
    assert_eq!(query.query_string.type_, Some(QueryStringType::BestFields));

    let json = Query::from(query).json()?;
    let expected = json!({
        "query_string": {
            "query": "title:elasticsearch or description:search",
            "default_field": "content",
            "fields": ["title", "description"],
            "default_operator": "and",
            "analyzer": "standard",
            "analyze_wildcard": true,
            "lenient": true,
            "boost": 1.5,
            "type": "best_fields"
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_wildcard_query_builder() -> Result<(), Error> {
    let query = WildcardQuery::builder()
        .field(
            "name".to_string(),
            WildcardQueryRule::advanced()
                .value("jo*n".to_string())
                .boost(1.5)
                .case_insensitive(true)
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "wildcard": {
            "name": {
                "value": "jo*n",
                "boost": 1.5,
                "case_insensitive": true
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_prefix_query_builder() -> Result<(), Error> {
    let query = PrefixQuery::builder()
        .field(
            "name".to_string(),
            PrefixQueryRule::advanced()
                .value("jo".to_string())
                .rewrite("constant_score".to_string())
                .boost(1.5)
                .case_insensitive(true)
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "prefix": {
            "name": {
                "value": "jo",
                "rewrite": "constant_score",
                "boost": 1.5,
                "case_insensitive": true
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_match_bool_prefix_query_builder_simple() -> Result<(), Error> {
    let query = MatchBoolPrefixQuery::builder()
        .field(
            "title".to_string(),
            MatchBoolPrefixQueryRule::Simple("quick brown f".to_string()),
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "match_bool_prefix": {
            "title": "quick brown f"
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_match_bool_prefix_query_builder_advanced() -> Result<(), Error> {
    let query = MatchBoolPrefixQuery::builder()
        .field(
            "title".to_string(),
            MatchBoolPrefixQueryRule::advanced()
                .query("quick brown f".to_string())
                .boost(1.5)
                .operator("AND".to_string())
                .minimum_should_match("75%".to_string())
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "match_bool_prefix": {
            "title": {
                "query": "quick brown f",
                "boost": 1.5,
                "operator": "AND",
                "minimum_should_match": "75%"
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_match_all_query_builder() -> Result<(), Error> {
    let query = MatchAllQuery::builder().boost(1.5).build()?;

    assert_eq!(query.match_all.boost, Some(1.5));

    let json = Query::from(query).json()?;
    let expected = json!({
        "match_all": {
            "boost": 1.5
        }
    });

    assert_eq!(json, expected);

    // Test default case without boost
    let query_default = MatchAllQuery::builder().build()?;
    let json_default = Query::from(query_default).json()?;
    let expected_default = json!({
        "match_all": {}
    });

    assert_eq!(json_default, expected_default);

    Ok(())
}

#[test]
fn test_match_none_query_builder() -> Result<(), Error> {
    let query = MatchNoneQuery::simple();
    let json = Query::from(query).json()?;
    let expected_default = json!({
        "match_none": {}
    });

    assert_eq!(json, expected_default);

    Ok(())
}

#[test]
fn test_match_phrase_query_builder() -> Result<(), Error> {
    let query = Query::match_phrase()
        .field("title", "quick brown fox")
        .field(
            "content",
            MatchPhraseQueryRule::advanced()
                .query("quick brown fox".to_string())
                .analyzer("standard".to_string())
                .slop(2)
                .boost(1.5)
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "match_phrase": {
            "title": "quick brown fox",
            "content": {
                "query": "quick brown fox",
                "analyzer": "standard",
                "slop": 2,
                "boost": 1.5
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_match_phrase_prefix_query_builder() -> Result<(), Error> {
    let query = MatchPhrasePrefixQuery::builder()
        .field(
            "title".to_string(),
            MatchPhrasePrefixQueryRule::advanced()
                .query("quick brown f".to_string())
                .analyzer("standard".to_string())
                .slop(2)
                .max_expansions(10)
                .boost(1.5)
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "match_phrase_prefix": {
            "title": {
                "query": "quick brown f",
                "analyzer": "standard",
                "slop": 2,
                "max_expansions": 10,
                "boost": 1.5
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_multi_match_query_builder() -> Result<(), Error> {
    let query = MultiMatchQuery::builder()
        .query("search text".to_string())
        .fields(vec!["title".to_string(), "description".to_string()])
        .type_(MatchType::BestFields)
        .operator(Operator::And)
        .analyzer("standard".to_string())
        .minimum_should_match(MinimumShouldMatch::Complex("75%".to_string()))
        .fuzziness(Fuzziness::Auto)
        .prefix_length(2)
        .max_expansions(10)
        .slop(2)
        .zero_terms_query(ZeroTermsQuery::All)
        .auto_generate_synonyms_phrase_query(true)
        .lenient(true)
        .boost(1.5)
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "multi_match": {
            "query": "search text",
            "fields": ["title", "description"],
            "type": "best_fields",
            "operator": "and",
            "analyzer": "standard",
            "minimum_should_match": "75%",
            "fuzziness": "auto",
            "prefix_length": 2,
            "max_expansions": 10,
            "slop": 2,
            "zero_terms_query": "all",
            "auto_generate_synonyms_phrase_query": true,
            "lenient": true,
            "boost": 1.5
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_term_query_builder() -> Result<(), Error> {
    // Test simple value creation
    let simple_rule = TermQueryRule::value(json!("test"));
    assert_eq!(simple_rule.value, json!("test"));
    assert_eq!(simple_rule.boost, None);
    assert_eq!(simple_rule.case_insensitive, None);

    // Test complete configuration
    let complete_rule = TermQueryRule::builder()
        .value(json!("test"))
        .boost(2.0)
        .case_insensitive(true)
        .build()?;
    assert_eq!(complete_rule.value, json!("test"));
    assert_eq!(complete_rule.boost, Some(2.0));
    assert_eq!(complete_rule.case_insensitive, Some(true));

    // Test error handling
    let result = TermQueryRule::builder().build();
    assert!(result.is_err(), "Building without value should fail");
    Ok(())
}
#[test]
fn test_ids_query_builder() -> Result<(), Error> {
    let query = IdsQuery::builder()
        .values(vec!["1".to_string(), "2".to_string(), "3".to_string()])
        .boost(1.5)
        .build()?;

    assert_eq!(
        query.ids.values,
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );
    assert_eq!(query.ids.boost, Some(1.5));

    let json = Query::from(query).json()?;
    let expected = json!({
        "ids": {
            "values": ["1", "2", "3"],
            "boost": 1.5
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_fuzzy_query_builder() -> Result<(), Error> {
    let query = FuzzyQuery::builder()
        .field(
            "name".to_string(),
            FuzzyQueryRule::builder()
                .value("kimchy".to_string())
                .fuzziness(Fuzziness::Auto)
                .prefix_length(2)
                .max_expansions(50)
                .transpositions(true)
                .rewrite("constant_score".to_string())
                .boost(1.5)
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "fuzzy": {
            "name": {
                "value": "kimchy",
                "fuzziness": "auto",
                "prefix_length": 2,
                "max_expansions": 50,
                "transpositions": true,
                "rewrite": "constant_score",
                "boost": 1.5
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_nested_query_builder() -> Result<(), Error> {
    let match_query = MatchQuery::builder()
        .field("reviews.rating", MatchQueryRule::Simple("5".to_string()))
        .build()?;

    let nested_query = NestedQuery::builder()
        .path("reviews")
        .query(Box::new(Query::from(match_query)))
        .score_mode(NestedScoreMode::Avg)
        .ignore_unmapped(false)
        .boost(1.5)
        .build()?;

    let json = Query::from(nested_query).json()?;
    let expected = json!({
        "nested": {
            "path": "reviews",
            "query": {
                "match": {
                    "reviews.rating": "5"
                }
            },
            "score_mode": "avg",
            "ignore_unmapped": false,
            "boost": 1.5
        }
    });

    assert_eq!(json, expected);
    Ok(())
}

#[test]
fn test_regexp_query_builder() -> Result<(), Error> {
    let query = RegexpQuery::builder()
        .field(
            "name".to_string(),
            RegexpQueryRule::advanced()
                .value("j.*n".to_string())
                .flags("ALL".to_string())
                .max_determinized_states(10000)
                .rewrite("constant_score".to_string())
                .boost(1.5)
                .case_insensitive(true)
                .build()?,
        )
        .build()?;

    let json = Query::from(query).json()?;
    let expected = json!({
        "regexp": {
            "name": {
                "value": "j.*n",
                "flags": "ALL",
                "max_determinized_states": 10000,
                "rewrite": "constant_score",
                "boost": 1.5,
                "case_insensitive": true
            }
        }
    });

    assert_eq!(json, expected);
    Ok(())
}
