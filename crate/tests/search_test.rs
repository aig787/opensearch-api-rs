pub mod fixture;

use crate::fixture::OpenSearchFixture;
use anyhow::Result;
use opensearch_api::types::common::GeoPoint;
use opensearch_api::types::query::*;
use opensearch_api::types::search::{MSearchHeader, MSearchItem};

mod common {
    use crate::fixture::OpenSearchFixture;
    use anyhow::Result;
    use opensearch_api::types::common::GeoPoint;
    use opensearch_api::types::query::{GeoJsonShape, GeoShape};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
    pub struct TestDocument {
        pub id: String,
        pub title: String,
        pub content: String,
        pub tags: Vec<String>,
        pub rating: Option<f64>,
        pub published: bool,
        pub created_at: String,
        pub location: Option<GeoPoint>,
        pub shape: Option<GeoShape>,
    }

    pub async fn index_test_documents(fixture: &OpenSearchFixture, index_name: &str) -> Result<()> {
        let index = fixture.namespaced_index(index_name);

        // Create index
        let create_result = fixture
            .client
            .indices()
            .create(&index)
            .mappings(json!({
               "properties": {
                    "title": {
                        "type": "keyword"
                    },
                    "content": {
                        "type": "text"
                    },
                    "location": {
                        "type": "geo_point"
                    },
                    "shape": {
                        "type": "geo_shape"
                    }
                }
            }))
            .build()?
            .send()
            .await?;
        assert!(create_result.acknowledged);

        // Add test documents
        let documents = vec![
            TestDocument {
                id: "1".to_string(),
                title: "OpenSearch Introduction".to_string(),
                content: "OpenSearch is a powerful search engine.".to_string(),
                tags: vec!["search".to_string(), "open-source".to_string()],
                rating: Some(4.5),
                published: true,
                created_at: "2023-01-15T12:00:00Z".to_string(),
                location: Some(GeoPoint {
                    lat: 40.7128,
                    lon: -74.0060,
                }),
                shape: None,
            },
            TestDocument {
                id: "2".to_string(),
                title: "Advanced Queries".to_string(),
                content: "Learn how to write complex queries in OpenSearch.".to_string(),
                tags: vec!["search".to_string(), "tutorial".to_string()],
                rating: Some(4.8),
                published: true,
                created_at: "2023-02-20T14:30:00Z".to_string(),
                location: Some(GeoPoint {
                    lat: 34.0522,
                    lon: -118.2437,
                }),
                shape: Some(GeoShape::GeoJson(GeoJsonShape::Polygon {
                    coordinates: vec![vec![
                        [73.0515, 41.5582],
                        [72.6506, 41.5623],
                        [72.6734, 41.7658],
                        [73.0515, 41.5582],
                    ]],
                })),
            },
            TestDocument {
                id: "3".to_string(),
                title: "Aggregation Framework".to_string(),
                content: "Master the aggregation framework for powerful analytics.".to_string(),
                tags: vec!["analytics".to_string(), "tutorial".to_string()],
                rating: Some(4.2),
                published: true,
                created_at: "2023-03-10T09:15:00Z".to_string(),
                location: Some(GeoPoint {
                    lat: 51.5074,
                    lon: -0.1278,
                }),
                shape: Some(GeoShape::point(40.7128, -74.0060)),
            },
            TestDocument {
                id: "4".to_string(),
                title: "Upcoming Features".to_string(),
                content: "Preview of upcoming features in the next release.".to_string(),
                tags: vec!["roadmap".to_string(), "features".to_string()],
                rating: Some(3.9),
                published: false,
                created_at: "2023-04-05T16:45:00Z".to_string(),
                location: Some(GeoPoint {
                    lat: 48.8566,
                    lon: 2.3522,
                }),
                shape: None,
            },
            TestDocument {
                id: "5".to_string(),
                title: "Performance Optimization".to_string(),
                content: "Tips and tricks for optimizing OpenSearch performance.".to_string(),
                tags: vec!["performance".to_string(), "optimization".to_string()],
                rating: Some(4.7),
                published: true,
                created_at: "2023-05-12T11:20:00Z".to_string(),
                location: Some(GeoPoint {
                    lat: 35.6762,
                    lon: 139.6503,
                }),
                shape: None,
            },
        ];

        // Index each document
        for doc in documents {
            let result = fixture
                .client
                .documents()
                .index(&index)
                .id(doc.id.to_string())
                .document(&doc)
                .send()
                .await?;
            assert!(result.result == "created" || result.result == "updated");
        }

        // Refresh the index to make documents available for search
        let refresh_result = fixture
            .client
            .indices()
            .refresh(&index)
            .build()?
            .send()
            .await?;
        assert!(refresh_result._shards.successful > 0);

        Ok(())
    }

    pub async fn cleanup_test_index(fixture: &OpenSearchFixture, index_name: &str) -> Result<()> {
        let index = fixture.namespaced_index(index_name);
        let delete_result = fixture
            .client
            .indices()
            .delete(&index)
            .build()?
            .send()
            .await?;
        assert!(delete_result.acknowledged);
        Ok(())
    }
}

// Individual test functions for each query type
#[tokio::test]
async fn test_match_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = MatchQuery::builder()
        .field(
            "content".to_string(),
            MatchQueryRule::Simple("search".to_string()),
        )
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results contain "search" in content
    for hit in response.hits.hits {
        assert!(hit
            .source
            .unwrap()
            .content
            .to_lowercase()
            .contains("search"));
    }

    Ok(())
}

#[tokio::test]
async fn test_term_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = TermQuery::builder()
        .field(
            "tags".to_string(),
            TermQueryRule::value(serde_json::Value::String("tutorial".to_string())),
        )
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results have "tutorial" tag
    for hit in response.hits.hits {
        assert!(hit
            .source_ref_required()
            .tags
            .contains(&"tutorial".to_string()));
    }

    Ok(())
}

#[tokio::test]
async fn test_range_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = RangeQuery::builder()
        .field(
            "rating".to_string(),
            RangeQueryRule::builder()
                .gte(serde_json::Value::from(4.5))
                .build()?,
        )
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results have rating >= 4.5
    for hit in response.hits.hits {
        assert!(hit.source_ref_required().rating.unwrap() >= 4.5);
    }

    Ok(())
}

#[tokio::test]
async fn test_bool_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let must_query = MatchQuery::builder()
        .field(
            "published".to_string(),
            MatchQueryRule::Simple("true".to_string()),
        )
        .build()?;

    let should_query = TermQuery::builder()
        .field(
            "tags".to_string(),
            TermQueryRule::value(serde_json::Value::String("tutorial".to_string())),
        )
        .build()?;

    let query = BoolQuery::builder()
        .must(vec![must_query.into()])
        .should(vec![should_query.into()])
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results are published
    for hit in response.hits.hits {
        assert!(hit.source_ref_required().published);
    }

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_exists_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = ExistsQuery::builder().field("rating".to_string()).build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results have rating field
    for hit in response.hits.hits {
        assert!(hit.source_ref_required().rating.is_some());
    }

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_query_string_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = QueryStringQuery::builder()
        .query("content:performance OR title:optimization".to_string())
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results match query string criteria
    for hit in response.hits.hits {
        let content_match = hit
            .source_ref_required()
            .content
            .to_lowercase()
            .contains("performance");
        let title_match = hit
            .source_ref_required()
            .title
            .to_lowercase()
            .contains("optimization");
        assert!(content_match || title_match);
    }

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_wildcard_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = WildcardQuery::builder()
        .field(
            "title".to_string(),
            WildcardQueryRule::simple("*Queries*".to_string()),
        )
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results match wildcard pattern
    for hit in response.hits.hits {
        assert!(hit.source_ref_required().title.contains("Queries"));
    }

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_prefix_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = PrefixQuery::builder()
        .field(
            "title".to_string(),
            PrefixQueryRule::simple("Open".to_string()),
        )
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results match prefix
    for hit in response.hits.hits {
        assert!(hit.source_ref_required().title.starts_with("Open"));
    }

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_match_all_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = MatchAllQuery::builder().build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert_eq!(response.hits.total.value, 5);
    assert_eq!(response.hits.hits.len(), 5);

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_match_none_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = MatchNoneQuery::simple();

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert_eq!(response.hits.total.value, 0);
    assert!(response.hits.hits.is_empty());

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_match_phrase_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = MatchPhraseQuery::builder()
        .field(
            "content".to_string(),
            MatchPhraseQueryRule::simple("powerful search".to_string()),
        )
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results contain the exact phrase
    for hit in response.hits.hits {
        assert!(hit
            .source_ref_required()
            .content
            .to_lowercase()
            .contains("powerful search"));
    }

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_match_phrase_prefix_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = MatchPhrasePrefixQuery::builder()
        .field(
            "content".to_string(),
            MatchPhrasePrefixQueryRule::advanced()
                .query("powerful search".to_string())
                .max_expansions(10)
                .build()?,
        )
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_multi_match_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = MultiMatchQuery::builder()
        .query("tutorial".to_string())
        .fields(vec![
            "title".to_string(),
            "content".to_string(),
            "tags".to_string(),
        ])
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_ids_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = IdsQuery::builder()
        .values(vec!["1".to_string(), "3".to_string()])
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert_eq!(response.hits.total.value, 2);
    assert_eq!(response.hits.hits.len(), 2);

    // Verify correct IDs are returned
    let ids: Vec<String> = response
        .hits
        .hits
        .iter()
        .map(|hit| hit.source_ref_required().id.clone())
        .collect();
    assert!(ids.contains(&"1".to_string()));
    assert!(ids.contains(&"3".to_string()));

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_fuzzy_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = FuzzyQuery::builder()
        .field(
            "content".to_string(),
            FuzzyQueryRule::builder()
                .value("optimzng".to_string())
                .build()?,
        )
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_regexp_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = RegexpQuery::builder()
        .field(
            "title".to_string(),
            RegexpQueryRule::simple(".*Optimization.*".to_string()),
        )
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results match regexp
    for hit in response.hits.hits {
        assert!(hit.source_ref_required().title.contains("Optimization"));
    }

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_terms_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = TermsQuery::builder()
        .field(
            "tags".to_string(),
            TermsQueryRule::simple(vec![
                serde_json::Value::String("tutorial".to_string()),
                serde_json::Value::String("performance".to_string()),
            ]),
        )
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);
    assert!(!response.hits.hits.is_empty());

    // Verify results have at least one of the tags
    for hit in response.hits.hits {
        let has_tutorial = hit
            .source_ref_required()
            .tags
            .contains(&"tutorial".to_string());
        let has_performance = hit
            .source_ref_required()
            .tags
            .contains(&"performance".to_string());
        assert!(has_tutorial || has_performance);
    }

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_geo_distance_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = BoolQuery::builder()
        .filter(vec![GeoDistanceQuery::builder()
            .distance("1000km".to_string())
            .point(GeoPointField::new("location", 40.7128, -74.0060))
            .build()?
            .into()])
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_geo_bounding_box_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = BoolQuery::builder()
        .filter(vec![GeoBoundingBoxQuery::builder()
            .field(
                "location",
                GeoBoundingBoxQueryRule::builder()
                    .top_left(GeoPoint::new(41.0, -75.0))
                    .bottom_right(GeoPoint::new(40.0, -73.0))
                    .build()?,
            )
            .build()?
            .into()])
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_geo_shape_query() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let query = BoolQuery::builder()
        .filter(vec![GeoShapeQuery::builder()
            .field(
                "shape".to_string(),
                GeoShapeQueryRule::builder()
                    .shape(GeoShape::GeoJson(GeoJsonShape::Envelope {
                        coordinates: [[71.0589, 42.3601], [74.006, 40.7128]],
                    }))
                    .relation(GeoShapeRelation::Intersects)
                    .build()?,
            )
            .build()?
            .into()])
        .build()?;

    let response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .build()?
        .send()
        .await?;

    assert!(response.hits.total.value > 0);

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_scroll() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // First search with scroll parameter
    let query = MatchAllQuery::builder().build()?;

    let search_response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .size(2) // Small size to ensure we have multiple pages
        .scroll("1m")
        .build()?
        .send()
        .await?;

    assert!(search_response.hits.total.value > 0);
    assert!(!search_response.hits.hits.is_empty());
    assert!(search_response.scroll_id.is_some());

    let scroll_id = search_response.scroll_id.unwrap();

    // Use scroll API to get next page
    let scroll_response = fixture
        .client
        .scroll::<common::TestDocument>()
        .scroll_id(scroll_id)
        .scroll("1m")
        .build()?
        .send()
        .await?;

    assert!(!scroll_response.hits.hits.is_empty());

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_clear_scroll() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // First search with scroll parameter
    let query = MatchAllQuery::builder().build()?;

    let search_response = fixture
        .client
        .search::<common::TestDocument>(index)
        .query(query)
        .scroll("1m")
        .build()?
        .send()
        .await?;

    assert!(search_response.scroll_id.is_some());

    let scroll_id = search_response.scroll_id.unwrap();

    // Clear the scroll
    let clear_response = fixture
        .client
        .clear_scroll()
        .scroll_ids(vec![scroll_id])
        .build()?
        .send()
        .await?;

    assert!(clear_response.succeeded);

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_msearch() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "search_test";
    common::index_test_documents(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create two different search queries
    let match_query = MatchQuery::builder()
        .field(
            "tags".to_string(),
            MatchQueryRule::Simple("tutorial".to_string()),
        )
        .build()?;

    let term_query = TermQuery::builder()
        .field(
            "published".to_string(),
            TermQueryRule::value(serde_json::Value::Bool(true)),
        )
        .build()?;

    // Create MSearch items
    let item1 = MSearchItem {
        header: MSearchHeader::builder().index(&index).build()?,
        body: serde_json::json!({ "query": match_query }),
    };

    let item2 = MSearchItem {
        header: MSearchHeader::builder().index(&index).build()?,
        body: serde_json::json!({ "query": term_query }),
    };

    // Execute multi-search
    let response = fixture
        .client
        .msearch::<common::TestDocument>()
        .searches(vec![item1, item2])
        .build()?
        .send()
        .await?;

    assert_eq!(response.responses.len(), 2);
    assert!(response.responses[0].hits.total.value > 0);
    assert!(response.responses[1].hits.total.value > 0);

    common::cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}
