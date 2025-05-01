use opensearch_api::indices::IndexSettings;
use opensearch_api::types::query::{MatchQuery, MatchQueryRule};
use opensearch_api::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct Product {
    id: String,
    name: String,
    description: String,
    price: f64,
    category: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::builder()
        .base_url("http://localhost:9200")
        .username("admin")
        .password("admin")
        .build()?;

    // Check if the cluster is available
    if client.ping().await? {
        println!("Connected to OpenSearch cluster!");
    }

    // Create an index
    let index_name = "products";

    if !client.indices().exists(index_name).build()?.send().await? {
        client
            .indices()
            .create(index_name)
            .settings(
                IndexSettings::builder()
                    .number_of_shards(1)
                    .number_of_replicas(1)
                    .build()?,
            )
            .mappings(json!({
                "properties": {
                    "id": { "type": "keyword" },
                    "name": { "type": "text" },
                    "description": { "type": "text" },
                    "price": { "type": "float" },
                    "category": { "type": "keyword" }
                }
            }))
            .build()?
            .send()
            .await?;
        println!("Created index '{}'", index_name);
    }

    // Index a document
    let product = Product {
        id: "1".to_string(),
        name: "Mechanical Keyboard".to_string(),
        description: "Ergonomic mechanical keyboard with RGB lighting".to_string(),
        price: 149.99,
        category: "Electronics".to_string(),
    };

    client
        .documents()
        .index(index_name)
        .document(&product)
        .id(&product.id)
        .build()?
        .send()
        .await?;
    println!("Indexed product: {}", product.name);

    // Get the document
    let retrieved = client
        .documents()
        .get::<Product>(index_name, &product.id)
        .send()
        .await?;

    if let Some(response) = retrieved {
        println!("Retrieved product: {} - ${}", response.source_ref_required().name, response.source_ref_required().price);
    }

    // Search for documents
    let search_response = client
        .search::<Product>()
        .index(index_name)
        .from(0)
        .size(10)
        .query(
            MatchQuery::builder()
                .field("category", MatchQueryRule::simple("Electronics"))
                .build()?
                .into_query(),
        )
        .build()?
        .send()
        .await?;

    println!("Found {} products", search_response.hits.total.value);

    for hit in search_response.hits.hits {
        if let Some(product) = hit.source {
            println!("- {} (${:.2})", product.name, product.price);
        }
    }

    Ok(())
}
