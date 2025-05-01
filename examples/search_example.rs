use opensearch_api::indices::IndexSettings;
use opensearch_api::types::query::{
    BoolQuery, MatchQuery, MatchQueryRule, RangeQuery, RangeQueryRule, TermQuery, TermQueryRule,
};
use opensearch_api::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct Product {
    id: String,
    name: String,
    description: String,
    price: f64,
    category: String,
    in_stock: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a client
    let client = Client::builder()
        .base_url("http://localhost:9200")
        .username("admin")
        .password("admin")
        .build()?;

    let index_name = "my-test-index";

    // Check if index already exists and delete it if it does
    if client.indices().exists(index_name).build()?.send().await? {
        println!("Index '{}' already exists, deleting...", index_name);
        let response = client.indices().delete(index_name).build()?.send().await?;
        println!(
            "Delete response: {}",
            serde_json::to_string_pretty(&response)?
        );
    }

    // Create a new index with mappings
    println!("Creating index '{}'...", index_name);
    let response = client
        .indices()
        .create(index_name)
        .settings(
            IndexSettings::builder()
                .number_of_shards(2)
                .number_of_replicas(1)
                .refresh_interval("5s")
                .build()?,
        )
        .mappings(json!({
            "properties": {
                "id": { "type": "keyword" },
                "name": { "type": "text" },
                "description": { "type": "text" },
                "price": { "type": "float" },
                "category": { "type": "keyword" },
                "in_stock": { "type": "boolean" }
            }
        }))
        .build()?
        .send()
        .await?;
    println!(
        "Create response: {}",
        serde_json::to_string_pretty(&response)?
    );

    // Index some documents
    println!("Indexing documents...");

    let products = vec![
        Product {
            id: "1".to_string(),
            name: "Mechanical Keyboard".to_string(),
            description: "Ergonomic mechanical keyboard with RGB lighting".to_string(),
            price: 149.99,
            category: "Computer Accessories".to_string(),
            in_stock: true,
        },
        Product {
            id: "2".to_string(),
            name: "Wireless Mouse".to_string(),
            description: "Bluetooth wireless mouse with adjustable DPI".to_string(),
            price: 49.99,
            category: "Computer Accessories".to_string(),
            in_stock: true,
        },
        Product {
            id: "3".to_string(),
            name: "4K Monitor".to_string(),
            description: "32-inch 4K UHD monitor with HDR".to_string(),
            price: 349.99,
            category: "Computer Accessories".to_string(),
            in_stock: false,
        },
        Product {
            id: "4".to_string(),
            name: "Laptop Stand".to_string(),
            description: "Adjustable laptop stand with cooling pad".to_string(),
            price: 39.99,
            category: "Computer Accessories".to_string(),
            in_stock: true,
        },
        Product {
            id: "5".to_string(),
            name: "Webcam HD".to_string(),
            description: "1080p webcam with microphone".to_string(),
            price: 79.99,
            category: "Computer Accessories".to_string(),
            in_stock: false,
        },
    ];

    for product in &products {
        let _response = client
            .documents()
            .index(index_name)
            .document(&product)
            .id(&product.id)
            .build()?
            .send()
            .await?;
        println!("Indexed product {}: {}", product.id, product.name);
    }

    // Refresh the index to make documents available for search
    println!("Refreshing index...");
    client.documents().refresh(index_name).send().await?;

    // Perform a match query - find products with "keyboard" in the name
    println!("\nPerforming match query for 'keyboard' in name field:");

    let search_response = client
        .search::<Product>()
        .index(index_name)
        .from(0)
        .size(10)
        .query(
            MatchQuery::builder()
                .field("name", MatchQueryRule::simple("keyboard"))
                .build()?
                .into_query(),
        )
        .build()?
        .send()
        .await?;

    println!("Found {} matches", search_response.hits.total.value);

    for hit in search_response.hits.hits {
        if let Some(product) = hit.source {
            println!("- {} (${:.2})", product.name, product.price);
        }
    }

    // Perform a term query - find products in specific category
    println!("\nPerforming term query for 'Computer Accessories' category:");
    let search_response = client
        .search::<Product>()
        .index(index_name)
        .from(0)
        .size(10)
        .query(
            TermQuery::builder()
                .field("category", TermQueryRule::value("Computer"))
                .build()?
                .into_query(),
        )
        .build()?
        .send()
        .await?;

    println!("Found {} matches", search_response.hits.total.value);

    // Perform a bool query - find in-stock products with price < 100
    println!("\nPerforming bool query for in-stock products with price < 100:");
    let query = BoolQuery::builder()
        .must(vec![
            TermQuery::builder()
                .field("in_stock", TermQueryRule::value(true))
                .build()?
                .into_query(),
            RangeQuery::builder()
                .field("price", RangeQueryRule::builder().lt(100).build()?)
                .build()?
                .into_query(),
        ])
        .build()?
        .into_query();
    let search_response = client
        .search::<Product>()
        .index(index_name)
        .from(0)
        .size(10)
        .query(query)
        .build()?
        .send()
        .await?;

    println!("Found {} matches", search_response.hits.total.value);

    for hit in search_response.hits.hits {
        if let Some(product) = hit.source {
            println!("- {} (${:.2})", product.name, product.price);
        }
    }

    Ok(())
}
