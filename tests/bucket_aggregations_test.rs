pub mod fixture;

use crate::fixture::OpenSearchFixture;
use anyhow::Result;
use opensearch_api::types::aggregations::{Aggregation, AggregationResponse, Aggregations, NestedAggregation, RangeDefinition};
use opensearch_api::types::common::RefreshPolicy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use opensearch_api::types::query::{RangeQuery, RangeQueryRule};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct Tag {
    value: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct Review {
    user: String,
    rating: i32,
    text: String,
    tags: Vec<Tag>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct Product {
    id: String,
    name: String,
    price: f64,
    quantity: i32,
    in_stock: bool,
    tags: Vec<String>,
    rating: Option<f64>,
    created_at: String, // ISO-8601 date format (YYYY-MM-DD)
    category: String,
    reviews: Option<Vec<Review>>,
}

async fn setup_test_data(fixture: &OpenSearchFixture, index_name: &str) -> Result<()> {
    let index = fixture.namespaced_index(index_name);

    // Check if index exists and delete it
    if fixture
        .client
        .indices()
        .exists(vec![&index])
        .build()?
        .send()
        .await?
    {
        fixture
            .client
            .indices()
            .delete(vec![&index])
            .build()?
            .send()
            .await?;
    }

    fixture
        .client
        .indices()
        .create(&index)
        .mappings(serde_json::json!({
            "properties": {
                "created_at": {
                    "type": "date",
                    "format": "yyyy-MM-dd"
                },
                "category": {
                    "type": "keyword"
                },
                "price": {
                    "type": "double"
                },
                "rating": {
                    "type": "double"
                },
                "reviews": {
                    "type": "nested",
                    "properties": {
                        "user": { "type": "keyword" },
                        "rating": { "type": "integer" },
                        "text": { "type": "text" },
                        "tags": {
                            "type": "nested",
                            "properties": {
                                "value": { "type": "keyword" }
                            }
                        }
                    }
                }
            }
        }))
        .build()?
        .send()
        .await?;

    // Sample product data with various metrics
    let products = vec![
        Product {
            id: "1".to_string(),
            name: "Laptop".to_string(),
            price: 1299.99,
            quantity: 5,
            in_stock: true,
            tags: vec!["electronics".to_string(), "computers".to_string()],
            rating: Some(4.8),
            created_at: "2023-01-15".to_string(),
            category: "Computers".to_string(),
            reviews: Some(vec![
                Review {
                    user: "user3".to_string(),
                    rating: 5,
                    text: "Excellent performance".to_string(),
                    tags: vec![
                        Tag {
                            value: "performance".to_string(),
                        },
                        Tag {
                            value: "reliability".to_string(),
                        },
                    ],
                },
                Review {
                    user: "user1".to_string(),
                    rating: 4,
                    text: "Great machine but a bit expensive".to_string(),
                    tags: vec![
                        Tag {
                            value: "expensive".to_string(),
                        },
                        Tag {
                            value: "quality".to_string(),
                        },
                    ],
                },
            ]),
        },
        Product {
            id: "2".to_string(),
            name: "Smartphone".to_string(),
            price: 899.99,
            quantity: 10,
            in_stock: true,
            tags: vec!["electronics".to_string(), "mobile".to_string()],
            rating: Some(4.5),
            created_at: "2023-02-10".to_string(),
            category: "Mobile Devices".to_string(),
            reviews: Some(vec![
                Review {
                    user: "user2".to_string(),
                    rating: 5,
                    text: "Amazing camera quality".to_string(),
                    tags: vec![
                        Tag {
                            value: "camera".to_string(),
                        },
                        Tag {
                            value: "quality".to_string(),
                        },
                    ],
                },
                Review {
                    user: "user3".to_string(),
                    rating: 4,
                    text: "Good value for money".to_string(),
                    tags: vec![
                        Tag {
                            value: "value".to_string(),
                        },
                        Tag {
                            value: "budget".to_string(),
                        },
                    ],
                },
            ]),
        },
        Product {
            id: "3".to_string(),
            name: "Headphones".to_string(),
            price: 199.99,
            quantity: 15,
            in_stock: true,
            tags: vec!["electronics".to_string(), "audio".to_string()],
            rating: Some(4.2),
            created_at: "2023-02-25".to_string(),
            category: "Audio".to_string(),
            reviews: Some(vec![
                Review {
                    user: "user3".to_string(),
                    rating: 5,
                    text: "Excellent performance".to_string(),
                    tags: vec![
                        Tag {
                            value: "performance".to_string(),
                        },
                        Tag {
                            value: "sound".to_string(),
                        },
                    ],
                },
                Review {
                    user: "user4".to_string(),
                    rating: 3,
                    text: "Good but uncomfortable for long sessions".to_string(),
                    tags: vec![
                        Tag {
                            value: "comfort".to_string(),
                        },
                        Tag {
                            value: "durability".to_string(),
                        },
                    ],
                },
            ]),
        },
        Product {
            id: "4".to_string(),
            name: "Monitor".to_string(),
            price: 499.99,
            quantity: 3,
            in_stock: true,
            tags: vec!["electronics".to_string(), "displays".to_string()],
            rating: Some(4.6),
            created_at: "2023-03-12".to_string(),
            category: "Displays".to_string(),
            reviews: Some(vec![
                Review {
                    user: "user5".to_string(),
                    rating: 5,
                    text: "Crystal clear display".to_string(),
                    tags: vec![
                        Tag {
                            value: "display".to_string(),
                        },
                        Tag {
                            value: "clarity".to_string(),
                        },
                    ],
                },
                Review {
                    user: "user3".to_string(),
                    rating: 5,
                    text: "Excellent performance".to_string(),
                    tags: vec![
                        Tag {
                            value: "performance".to_string(),
                        },
                        Tag {
                            value: "precision".to_string(),
                        },
                    ],
                },
            ]),
        },
        Product {
            id: "5".to_string(),
            name: "Keyboard".to_string(),
            price: 89.99,
            quantity: 20,
            in_stock: true,
            tags: vec!["electronics".to_string(), "accessories".to_string()],
            rating: Some(4.1),
            created_at: "2023-04-05".to_string(),
            category: "Accessories".to_string(),
            reviews: Some(vec![
                Review {
                    user: "user1".to_string(),
                    rating: 4,
                    text: "Nice typing experience".to_string(),
                    tags: vec![
                        Tag {
                            value: "typing".to_string(),
                        },
                        Tag {
                            value: "ergonomic".to_string(),
                        },
                    ],
                },
                Review {
                    user: "user3".to_string(),
                    rating: 5,
                    text: "Excellent performance".to_string(),
                    tags: vec![
                        Tag {
                            value: "performance".to_string(),
                        },
                        Tag {
                            value: "graphics".to_string(),
                        },
                    ],
                },
            ]),
        },
        Product {
            id: "6".to_string(),
            name: "Mouse".to_string(),
            price: 49.99,
            quantity: 25,
            in_stock: true,
            tags: vec!["electronics".to_string(), "accessories".to_string()],
            rating: Some(4.0),
            created_at: "2023-04-20".to_string(),
            category: "Accessories".to_string(),
            reviews: Some(vec![
                Review {
                    user: "user3".to_string(),
                    rating: 5,
                    text: "Excellent performance".to_string(),
                    tags: vec![
                        Tag {
                            value: "performance".to_string(),
                        },
                        Tag {
                            value: "satisfied".to_string(),
                        },
                    ],
                },
                Review {
                    user: "user2".to_string(),
                    rating: 3,
                    text: "A bit small for my hand".to_string(),
                    tags: vec![
                        Tag {
                            value: "size".to_string(),
                        },
                        Tag {
                            value: "ergonomics".to_string(),
                        },
                    ],
                },
            ]),
        },
        Product {
            id: "7".to_string(),
            name: "Tablet".to_string(),
            price: 1001.10,
            quantity: 8,
            in_stock: true,
            tags: vec!["electronics".to_string(), "mobile".to_string()],
            rating: Some(4.3),
            created_at: "2023-05-08".to_string(),
            category: "Mobile Devices".to_string(),
            reviews: Some(vec![
                Review {
                    user: "user3".to_string(),
                    rating: 5,
                    text: "Excellent performance".to_string(),
                    tags: vec![
                        Tag {
                            value: "performance".to_string(),
                        },
                        Tag {
                            value: "battery".to_string(),
                        },
                    ],
                },
                Review {
                    user: "user6".to_string(),
                    rating: 4,
                    text: "Great for drawing and note-taking".to_string(),
                    tags: vec![
                        Tag {
                            value: "drawing".to_string(),
                        },
                        Tag {
                            value: "note-taking".to_string(),
                        },
                    ],
                },
            ]),
        },
        Product {
            id: "8".to_string(),
            name: "Smart Watch".to_string(),
            price: 299.99,
            quantity: 12,
            in_stock: true,
            tags: vec!["electronics".to_string(), "wearables".to_string()],
            rating: Some(4.4),
            created_at: "2023-06-14".to_string(),
            category: "Wearables".to_string(),
            reviews: Some(vec![
                Review {
                    user: "user7".to_string(),
                    rating: 4,
                    text: "Great fitness features".to_string(),
                    tags: vec![
                        Tag {
                            value: "fitness".to_string(),
                        },
                        Tag {
                            value: "health".to_string(),
                        },
                    ],
                },
                Review {
                    user: "user3".to_string(),
                    rating: 5,
                    text: "Excellent performance".to_string(),
                    tags: vec![
                        Tag {
                            value: "performance".to_string(),
                        },
                        Tag {
                            value: "durable".to_string(),
                        },
                    ],
                },
            ]),
        },
        Product {
            id: "9".to_string(),
            name: "Camera".to_string(),
            price: 799.99,
            quantity: 6,
            in_stock: true,
            tags: vec!["electronics".to_string(), "photography".to_string()],
            rating: Some(4.7),
            created_at: "2023-07-22".to_string(),
            category: "Photography".to_string(),
            reviews: Some(vec![
                Review {
                    user: "user3".to_string(),
                    rating: 5,
                    text: "Excellent performance".to_string(),
                    tags: vec![
                        Tag {
                            value: "performance".to_string(),
                        },
                        Tag {
                            value: "speed".to_string(),
                        },
                    ],
                },
                Review {
                    user: "user8".to_string(),
                    rating: 5,
                    text: "Professional quality photos".to_string(),
                    tags: vec![
                        Tag {
                            value: "professional".to_string(),
                        },
                        Tag {
                            value: "photos".to_string(),
                        },
                    ],
                },
            ]),
        },
        Product {
            id: "10".to_string(),
            name: "Speaker".to_string(),
            price: 149.99,
            quantity: 18,
            in_stock: true,
            tags: vec!["electronics".to_string(), "audio".to_string()],
            rating: None, // Intentionally missing rating
            created_at: "2023-08-30".to_string(),
            category: "Audio".to_string(),
            reviews: Some(vec![
                Review {
                    user: "user3".to_string(),
                    rating: 5,
                    text: "Excellent performance".to_string(),
                    tags: vec![
                        Tag {
                            value: "performance".to_string(),
                        },
                        Tag {
                            value: "audio".to_string(),
                        },
                    ],
                },
                Review {
                    user: "user9".to_string(),
                    rating: 4,
                    text: "Great sound quality".to_string(),
                    tags: vec![
                        Tag {
                            value: "sound".to_string(),
                        },
                        Tag {
                            value: "quality".to_string(),
                        },
                    ],
                },
            ]),
        },
    ];

    // Bulk index the products
    let mut bulk = fixture.client.bulk();
    for product in products {
        bulk = bulk.index(&index, Some(&product.id), &product);
    }
    bulk.refresh(RefreshPolicy::Wait).send().await?;

    Ok(())
}

async fn cleanup_test_index(fixture: &OpenSearchFixture, index_name: &str) -> Result<()> {
    let index = fixture.namespaced_index(index_name);
    fixture
        .client
        .indices()
        .delete(vec![&index])
        .build()?
        .send()
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_terms_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_terms_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let aggregations: HashMap<String, Aggregation> = HashMap::from([(
        "category_terms".to_string(),
        Aggregation::terms()
            .field("category")
            .size(10)
            .build()?
            .into(),
    )]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify terms aggregation results
    let aggs = response.aggregations.unwrap();
    let category_terms = aggs.get("category_terms").unwrap();

    if let AggregationResponse::Terms(terms_result) = category_terms {
        // Check buckets
        assert!(!terms_result.buckets.is_empty(), "Should have buckets");

        // Verify the categories exist in the buckets
        let categories = terms_result
            .buckets
            .iter()
            .map(|bucket| bucket.key.as_str())
            .collect::<Vec<_>>();

        assert!(
            categories.contains(&"Computers"),
            "Should have Computers category"
        );

        assert!(
            categories.contains(&"Mobile Devices"),
            "Should have Mobile Devices category"
        );

        // Check doc counts for categories
        for bucket in &terms_result.buckets {
            if bucket.key == "Computers" {
                assert!(bucket.doc_count > 0, "Computers should have documents");
            }
            if bucket.key == "Mobile Devices" {
                assert!(bucket.doc_count > 0, "Mobile Devices should have documents");
            }
        }
    } else {
        panic!("Terms aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_range_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_range_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let aggregations = HashMap::from([(
        "price_ranges".to_string(),
        Aggregation::range()
            .field("price")
            .ranges(vec![
                RangeDefinition::builder().to(100.0).build()?,
                RangeDefinition::builder().from(100.0).to(500.0).build()?,
                RangeDefinition::builder().from(500.0).build()?,
            ])
            .build()?
    )]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify range aggregation results
    let aggs = response.aggregations.unwrap();
    let price_ranges = aggs.get("price_ranges").unwrap();

    if let AggregationResponse::Range(range_result) = price_ranges {
        // Check buckets
        assert_eq!(range_result.buckets.len(), 3, "Should have 3 range buckets");

        // Verify the buckets have expected ranges and counts
        for bucket in &range_result.buckets {
            match (bucket.from, bucket.to) {
                (None, Some(to)) if to < 100.1 => {
                    // Low price range (< 100)
                    assert!(
                        bucket.doc_count > 0,
                        "Low price range should have documents"
                    );
                }
                (Some(from), Some(to)) if from > 99.9 && to < 500.1 => {
                    // Medium price range (100-500)
                    assert!(
                        bucket.doc_count > 0,
                        "Medium price range should have documents"
                    );
                }
                (Some(from), None) if from > 499.9 => {
                    // High price range (> 500)
                    assert!(
                        bucket.doc_count > 0,
                        "High price range should have documents"
                    );
                }
                _ => panic!(
                    "Unexpected range found: {:?} to {:?}",
                    bucket.from, bucket.to
                ),
            }
        }
    } else {
        panic!("Range aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_histogram_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_histogram_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let aggregations: HashMap<String, Aggregation> = HashMap::from([(
        "price_histogram".to_string(),
        Aggregation::histogram()
            .field("price")
            .interval(200.0)
            .build()?
            .into(),
    )]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify histogram aggregation results
    let aggs = response.aggregations.unwrap();
    let price_histogram = aggs.get("price_histogram").unwrap();

    if let AggregationResponse::Histogram(histogram_result) = price_histogram {
        // Check buckets
        assert!(
            !histogram_result.buckets.is_empty(),
            "Should have histogram buckets"
        );

        // Verify intervals and doc counts
        let mut found_intervals = 0;
        for bucket in &histogram_result.buckets {
            // Verify we have expected intervals
            let key = bucket.key.as_f64().unwrap();
            if key == 0.0
                || key == 200.0
                || key == 400.0
                || key == 600.0
                || key == 800.0
                || key == 1000.0
                || key == 1200.0
            {
                found_intervals += 1;
                assert!(bucket.doc_count > 0, "Bucket should have documents");
            }
        }

        assert!(
            found_intervals > 0,
            "Should have found at least one expected interval"
        );
    } else {
        panic!("Histogram aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_date_histogram_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_date_histogram_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let aggregations: HashMap<String, Aggregation> = HashMap::from([(
        "creation_over_time".to_string(),
        Aggregation::date_histogram()
            .field("created_at")
            .calendar_interval("month")
            .build()?
            .into(),
    )]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify date histogram aggregation results
    let aggs = response.aggregations.unwrap();
    let creation_over_time = aggs.get("creation_over_time").unwrap();

    if let AggregationResponse::Histogram(date_histogram_result) = creation_over_time {
        // Check buckets
        assert!(
            !date_histogram_result.buckets.is_empty(),
            "Should have date histogram buckets"
        );

        // Verify all buckets have timestamps and doc counts
        for bucket in &date_histogram_result.buckets {
            assert!(bucket.key_as_string.is_some(), "Should have key_as_string");
            assert!(bucket.doc_count > 0, "Should have documents in the bucket");
        }
    } else {
        panic!("Date histogram aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_filter_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_filter_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create filter aggregation for expensive products
    let filter_agg = Aggregation::filter()
        .filter(
            RangeQuery::builder()
                .field("price", RangeQueryRule::builder().gt(500.0).build()?)
                .build()?,
        )
        .build()?;

    let aggregations: HashMap<String, Aggregation> = HashMap::from([(
        "expensive_products".to_string(),
        filter_agg.into(),
    )]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify filter aggregation results
    let aggs = response.aggregations.unwrap();
    let expensive_products = aggs.get("expensive_products").unwrap();

    if let AggregationResponse::Bucket(filter_result) = expensive_products {
        // Check doc count for expensive products
        assert!(
            filter_result.doc_count > 0,
            "Should have expensive products matching the filter"
        );

        // Since we have products with price >= 500 in test data
        assert!(
            filter_result.doc_count < 10,
            "Not all products should be expensive"
        );
    } else {
        panic!("Filter aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_subaggregations() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_subaggs_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create terms aggregation by category with a sub stats agg on price
    let mut aggs = HashMap::<String, Aggregation>::new();
    let stats_agg = Aggregation::stats().field("price").build()?;

    let mut sub_aggs = HashMap::new();
    sub_aggs.insert("price_stats".to_string(), stats_agg);

    let with_sub = Aggregation::terms()
        .field("category")
        .size(10)
        .aggs(sub_aggs)
        .build()?;

    aggs.insert("categories".to_string(), with_sub.into());

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggs)
        .build()?
        .send()
        .await?;

    // Verify sub-aggregation results
    let aggregations = response.aggregations.unwrap();
    let categories = aggregations.get("categories").unwrap();

    if let AggregationResponse::Terms(terms_result) = categories {
        // Check buckets
        assert!(!terms_result.buckets.is_empty(), "Should have buckets");

        // Check that each bucket has the sub price_stats aggregation
        for bucket in &terms_result.buckets {
            let nested = &bucket.aggregations;
            let price_stats = nested.get("price_stats").unwrap();

            if let AggregationResponse::Stats(stats) = price_stats {
                assert!(stats.count > 0, "Should have documents");
                assert!(stats.min.is_some(), "Should have min value");
                assert!(stats.max.is_some(), "Should have max value");
                assert!(stats.avg.is_some(), "Should have avg value");
                assert!(stats.sum.is_some(), "Should have sum value");
            } else {
                panic!("Sub stats aggregation did not return expected format");
            }
        }
    } else {
        panic!("Terms aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_nested_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_multiply_nested_field_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Add sub-aggregations to nested
    let mut nested_sub_aggs: HashMap<String, Aggregation> = HashMap::new();
    nested_sub_aggs.insert(
        "tags".to_string(),
        Aggregation::terms()
            .field("reviews.tags.value")
            .size(10)
            .build()?
            .into(),
    );
    let aggs = Aggregations::builder()
        .agg(
            "reviews_agg",
            NestedAggregation::builder()
                .path("reviews")
                .aggs([(
                    "review_tags_agg",
                    NestedAggregation::builder()
                        .path("reviews.tags")
                        .aggs([(
                            "tag_counts",
                            Aggregation::terms()
                                .field("reviews.tags.value")
                                .size(10)
                                .build()?,
                        )])
                        .build()?,
                )])
                .build()?,
        )
        .build()?;

    // Execute the search with the nested aggregation
    let response = fixture
        .client
        .search::<serde_json::Value>(index)
        .aggregations(aggs)
        .build()?
        .send()
        .await?;

    // Verify nested aggregation results
    let aggregations = response.aggregations.unwrap();
    let review_stats = aggregations.get("reviews_agg").unwrap();


    if let AggregationResponse::Bucket(nested_result) = review_stats {
        // Check doc count
        assert!(nested_result.doc_count > 0, "Should have nested documents");

        // Check first level nested agg
        let review_tags_agg = nested_result.aggregations.get("review_tags_agg").unwrap();
        if let AggregationResponse::Bucket(nested_result) = review_tags_agg {
            assert!(nested_result.doc_count > 0, "Should have nested documents");
            // Check second level nested agg
            let tag_counts = nested_result.aggregations.get("tag_counts").unwrap();
            if let AggregationResponse::Terms(terms) = tag_counts {
                assert!(terms.buckets.len() > 0, "Should have buckets");
            } else {
                panic!("Terms aggregation did not return expected format");
            }
        } else {
           panic!("Review tags agg did not return expected format");
        }
    } else {
        panic!("Nested aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}
