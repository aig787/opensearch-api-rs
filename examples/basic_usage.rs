use opensearch_api::builder::MatchQuery;
use opensearch_api::indices::{CreateIndexRequest, IndexSettings};
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

    if !client.indices().exists(index_name).await? {
        let create_index_request = CreateIndexRequest::builder()
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
            .build()?;

        client
            .indices()
            .create(index_name, create_index_request)
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
        .index(index_name, Some(product.id.clone()), &product)
        .await?;
    println!("Indexed product: {}", product.name);

    // Get the document
    let retrieved: Option<Product> = client.documents().get(index_name, &product.id).await?;

    if let Some(doc) = retrieved {
        println!("Retrieved product: {} - ${}", doc.name, doc.price);
    }

    // Search for documents
    let search_response = client
        .search::<Product>()
        .index(index_name)
        .from(0)
        .size(10)
        .query(
            MatchQuery::builder()
                .field("category".to_string())
                .query("Electronics".to_string())
                .build_query()?,
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
