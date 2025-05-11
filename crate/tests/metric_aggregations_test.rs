use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod fixture;
use fixture::OpenSearchFixture;
use opensearch_api::types::aggregations::{
    Aggregation, AggregationResponse, AvgAggregation, CardinalityAggregation,
    ExtendedStatsAggregation, MaxAggregation, MedianAbsoluteDeviationAggregation, MinAggregation,
    PercentilesAggregation, StatsAggregation, SumAggregation,
};
use opensearch_api::types::common::RefreshPolicy;

// Test document structure
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct Product {
    id: String,
    name: String,
    price: f64,
    quantity: i32,
    in_stock: bool,
    tags: Vec<String>,
    rating: Option<f64>,
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

    // Create index
    fixture
        .client
        .indices()
        .create(&index)
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
        },
        Product {
            id: "2".to_string(),
            name: "Smartphone".to_string(),
            price: 899.99,
            quantity: 10,
            in_stock: true,
            tags: vec!["electronics".to_string(), "mobile".to_string()],
            rating: Some(4.5),
        },
        Product {
            id: "3".to_string(),
            name: "Headphones".to_string(),
            price: 199.99,
            quantity: 15,
            in_stock: true,
            tags: vec!["electronics".to_string(), "audio".to_string()],
            rating: Some(4.2),
        },
        Product {
            id: "4".to_string(),
            name: "Monitor".to_string(),
            price: 499.99,
            quantity: 3,
            in_stock: true,
            tags: vec!["electronics".to_string(), "displays".to_string()],
            rating: Some(4.6),
        },
        Product {
            id: "5".to_string(),
            name: "Keyboard".to_string(),
            price: 89.99,
            quantity: 20,
            in_stock: true,
            tags: vec!["electronics".to_string(), "accessories".to_string()],
            rating: Some(4.1),
        },
        Product {
            id: "6".to_string(),
            name: "Mouse".to_string(),
            price: 49.99,
            quantity: 25,
            in_stock: true,
            tags: vec!["electronics".to_string(), "accessories".to_string()],
            rating: Some(4.0),
        },
        Product {
            id: "7".to_string(),
            name: "Tablet".to_string(),
            price: 649.99,
            quantity: 8,
            in_stock: true,
            tags: vec!["electronics".to_string(), "mobile".to_string()],
            rating: Some(4.3),
        },
        Product {
            id: "8".to_string(),
            name: "Smart Watch".to_string(),
            price: 299.99,
            quantity: 12,
            in_stock: true,
            tags: vec!["electronics".to_string(), "wearables".to_string()],
            rating: Some(4.4),
        },
        Product {
            id: "9".to_string(),
            name: "Camera".to_string(),
            price: 799.99,
            quantity: 6,
            in_stock: true,
            tags: vec!["electronics".to_string(), "photography".to_string()],
            rating: Some(4.7),
        },
        Product {
            id: "10".to_string(),
            name: "Speaker".to_string(),
            price: 149.99,
            quantity: 18,
            in_stock: true,
            tags: vec!["electronics".to_string(), "audio".to_string()],
            rating: None, // Intentionally missing rating
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
async fn test_avg_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_avg_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create avg aggregation
    let avg_agg = AvgAggregation::builder().field("price").build()?;

    let aggregations = HashMap::from([("avg_price".to_string(), avg_agg)]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify avg aggregation results
    let aggs = response.aggregations.unwrap();
    let avg_price = aggs.get("avg_price").unwrap();

    if let AggregationResponse::NumericFloat { value, .. } = avg_price {
        // Calculate expected average manually
        let expected_avg = 493.99;
        assert!(
            (value - expected_avg).abs() < 0.01,
            "Average should be approximately 493.99"
        );
    } else {
        panic!("average aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_min_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_min_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create min aggregation
    let min_agg = MinAggregation::builder().field("price").build()?;

    let aggregations = HashMap::from([("min_price".to_string(), min_agg)]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify min aggregation results
    let aggs = response.aggregations.unwrap();
    let min_price = aggs.get("min_price").unwrap();

    if let AggregationResponse::NumericFloat { value, .. } = min_price {
        // The lowest price is 49.99
        assert!(
            (value - 49.99).abs() < 0.01,
            "Minimum price should be 49.99"
        );
    } else {
        panic!("Min aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_max_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_max_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create max aggregation
    let max_agg = MaxAggregation::builder().field("price").build()?;

    let aggregations = HashMap::from([("max_price".to_string(), max_agg)]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify max aggregation results
    let aggs = response.aggregations.unwrap();
    let max_price = aggs.get("max_price").unwrap();

    if let AggregationResponse::NumericFloat { value, .. } = max_price {
        // The highest price is 1299.99
        assert!(
            (value - 1299.99).abs() < 0.01,
            "Maximum price should be 1299.99"
        );
    } else {
        panic!("Max aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_sum_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_sum_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create sum aggregation
    let sum_agg = SumAggregation::builder().field("quantity").build()?;

    let aggregations = HashMap::from([("total_quantity".to_string(), sum_agg)]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify sum aggregation results
    let aggs = response.aggregations.unwrap();
    let total_quantity = aggs.get("total_quantity").unwrap();

    if let AggregationResponse::NumericFloat { value, .. } = total_quantity {
        // Sum of all quantities is 122
        assert_eq!(*value, 122.0, "Total quantity should be 122");
    } else {
        panic!("Sum aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_value_count_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_count_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    let aggregations = HashMap::from([(
        "rating_count".to_string(),
        Aggregation::value_count().field("rating").build()?,
    )]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify value_count aggregation results
    let aggs = response.aggregations.unwrap();
    let rating_count = aggs.get("rating_count").unwrap();

    if let AggregationResponse::NumericInt { value, .. } = rating_count {
        // 9 products have ratings, 1 has none
        assert_eq!(*value, 9, "Rating count should be 9");
    } else {
        panic!("Value count aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_stats_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_stats_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create stats aggregation
    let stats_agg = StatsAggregation::builder().field("price").build()?;

    let aggregations = HashMap::from([("price_stats".to_string(), stats_agg)]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify stats aggregation results
    let aggs = response.aggregations.unwrap();
    let price_stats = aggs.get("price_stats").unwrap();

    if let AggregationResponse::Stats(stats) = price_stats {
        // Check count
        assert_eq!(stats.count, 10, "Count should be 10");

        // Check min
        let min_val = stats.min.unwrap();
        assert!((min_val - 49.99).abs() < 0.01, "Min should be 49.99");

        // Check max
        let max_val = stats.max.unwrap();
        assert!((max_val - 1299.99).abs() < 0.01, "Max should be 1299.99");

        // Check avg
        let avg_val = stats.avg.unwrap();
        assert!(
            (avg_val - 493.99).abs() < 1.0,
            "Avg should be approximately 493.99"
        );

        // Check sum
        let sum_val = stats.sum.unwrap();
        assert!(
            (sum_val - 4939.9).abs() < 1.0,
            "Sum should be approximately 4939.9"
        );
    } else {
        panic!("Stats aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_extended_stats_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_extended_stats_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create extended_stats aggregation
    let extended_stats_agg = ExtendedStatsAggregation::builder()
        .field("price")
        .sigma(2.0) // Standard deviation parameter
        .build()?;

    let aggregations = HashMap::from([("price_extended_stats".to_string(), extended_stats_agg)]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify extended_stats aggregation results
    let aggs = response.aggregations.unwrap();
    let extended_stats = aggs.get("price_extended_stats").unwrap();

    if let AggregationResponse::Stats(stats) = extended_stats {
        // Basic stats checks
        assert_eq!(stats.count, 10, "Count should be 10");

        // Check extended stats fields
        assert!(
            stats.sum_of_squares.is_some(),
            "Should contain sum_of_squares"
        );
        assert!(stats.variance.is_some(), "Should contain variance");
        assert!(
            stats.std_deviation.is_some(),
            "Should contain std_deviation"
        );
        assert!(
            stats.std_deviation_bounds.is_some(),
            "Should contain std_deviation_bounds"
        );
    } else {
        panic!("Extended stats aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_cardinality_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_cardinality_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create cardinality aggregation to count unique tags
    let cardinality_agg = CardinalityAggregation::builder()
        .field("tags.keyword") // Using keyword field for exact matching
        .precision_threshold(100) // Higher precision for small datasets
        .build()?;

    let aggregations = HashMap::from([("unique_categories".to_string(), cardinality_agg)]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify cardinality aggregation results
    let aggs = response.aggregations.unwrap();
    let unique_categories = aggs.get("unique_categories").unwrap();

    if let AggregationResponse::NumericInt { value, .. } = unique_categories {
        // In our test data, we have several unique tag values
        // Note: The cardinality is approximate, so we're checking for a range
        assert!(*value > 0, "Should have found some unique tags");
    } else {
        panic!("Cardinality aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_percentiles_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_percentiles_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create percentiles aggregation
    let percentiles_agg = PercentilesAggregation::builder()
        .field("price")
        .percents(vec![25.0, 50.0, 75.0, 95.0, 99.0])
        .build()?;

    let aggregations = vec![("price_percentiles".to_string(), percentiles_agg)]
        .into_iter()
        .collect::<HashMap<_, _>>();

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify percentiles aggregation results
    let aggs = response.aggregations.unwrap();
    let percentiles = aggs.get("price_percentiles").unwrap();

    if let AggregationResponse::Percentile(percentiles_response) = percentiles {
        // Check each percentile
        let values = &percentiles_response.values;
        assert!(
            values.contains_key("25.0"),
            "Should contain 25th percentile"
        );
        assert!(
            values.contains_key("50.0"),
            "Should contain 50th percentile"
        );
        assert!(
            values.contains_key("75.0"),
            "Should contain 75th percentile"
        );
        assert!(
            values.contains_key("95.0"),
            "Should contain 95th percentile"
        );
        assert!(
            values.contains_key("99.0"),
            "Should contain 99th percentile"
        );

        // Verify percentiles are in ascending order
        let p25 = values.get("25.0").unwrap();
        let p50 = values.get("50.0").unwrap();
        let p75 = values.get("75.0").unwrap();

        assert!(p25 <= p50, "25th percentile should be <= 50th percentile");
        assert!(p50 <= p75, "50th percentile should be <= 75th percentile");
    } else {
        panic!("Percentiles aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_median_absolute_deviation_aggregation() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_mad_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create median_absolute_deviation aggregation
    let mad_agg = MedianAbsoluteDeviationAggregation::builder()
        .field("price")
        .build()?;

    let aggregations = HashMap::from([("price_mad".to_string(), mad_agg)]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify median_absolute_deviation aggregation results
    let aggs = response.aggregations.unwrap();
    let mad = aggs.get("price_mad").unwrap();

    if let AggregationResponse::NumericFloat { value, .. } = mad {
        assert!(*value >= 0.0, "MAD should be non-negative");
    } else {
        panic!("MAD aggregation did not return expected format");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}

#[tokio::test]
async fn test_multiple_metric_aggregations() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = "products_multi_metric_test";
    setup_test_data(&fixture, index_name).await?;
    let index = fixture.namespaced_index(index_name);

    // Create multiple metric aggregations
    let min_agg = MinAggregation::builder().field("price").build()?;
    let max_agg = MaxAggregation::builder().field("price").build()?;
    let avg_agg = AvgAggregation::builder().field("price").build()?;
    let sum_agg = SumAggregation::builder().field("quantity").build()?;

    let aggregations: HashMap<String, Aggregation> = HashMap::from([
        ("min_price".to_string(), min_agg.into()),
        ("max_price".to_string(), max_agg.into()),
        ("avg_price".to_string(), avg_agg.into()),
        ("total_quantity".to_string(), sum_agg.into()),
    ]);

    let response = fixture
        .client
        .search::<Product>(index)
        .aggregations(aggregations)
        .build()?
        .send()
        .await?;

    // Verify all aggregation results
    let aggs = response.aggregations.unwrap();

    // Check min price
    let min_price = aggs.get("min_price").unwrap();
    if let AggregationResponse::NumericFloat { value, .. } = min_price {
        assert!(
            (value - 49.99).abs() < 0.01,
            "Minimum price should be 49.99"
        );
    }

    // Check max price
    let max_price = aggs.get("max_price").unwrap();
    if let AggregationResponse::NumericFloat { value, .. } = max_price {
        assert!(
            (value - 1299.99).abs() < 0.01,
            "Maximum price should be 1299.99"
        );
    }

    // Check avg price
    let avg_price = aggs.get("avg_price").unwrap();
    if let AggregationResponse::NumericFloat { value, .. } = avg_price {
        assert!(
            (value - 493.99).abs() < 1.0,
            "Avg price should be approximately 493.99"
        );
    }

    // Check sum quantity
    let total_quantity = aggs.get("total_quantity").unwrap();
    if let AggregationResponse::NumericInt { value, .. } = total_quantity {
        assert_eq!(*value, 122, "Total quantity should be 122");
    }

    cleanup_test_index(&fixture, index_name).await?;
    Ok(())
}
