//! Tests for document-related operations in OpenSearch

pub mod fixture;

use crate::fixture::OpenSearchFixture;
use anyhow::Result;
use opensearch_api::types::document::{DeleteOptions, IndexOptions, UpdateOptions, WaitForActiveShards};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;

/// Test document structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestDocument {
    title: String,
    content: String,
    tags: Vec<String>,
    published: bool,
    views: u32,
}

impl TestDocument {
    fn new_sample() -> Self {
        Self {
            title: "Test Document".to_string(),
            content: "This is a test document for OpenSearch API".to_string(),
            tags: vec![
                "test".to_string(),
                "sample".to_string(),
                "opensearch".to_string(),
            ],
            published: true,
            views: 0,
        }
    }

    fn updated_sample() -> Self {
        Self {
            title: "Updated Document".to_string(),
            content: "This document has been updated through the API".to_string(),
            tags: vec![
                "test".to_string(),
                "updated".to_string(),
                "opensearch".to_string(),
            ],
            published: true,
            views: 10,
        }
    }
}

async fn setup_fixture() -> Result<OpenSearchFixture> {
    let fixture = OpenSearchFixture::new().await?;
    // Add a short delay to ensure the cluster is fully ready
    sleep(Duration::from_millis(500)).await;
    Ok(fixture)
}

#[tokio::test]
async fn test_index_document() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");
    let doc = TestDocument::new_sample();
    let doc_id = "test-doc-1";

    // Index a document with explicit ID
    let response = fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id(doc_id)
        .options(IndexOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    assert_eq!(response.index, index_name);
    assert_eq!(response.id, doc_id);
    assert_eq!(response.result, "created");
    assert!(response.primary_term > 0);
    assert_eq!(response._shards.successful, 1);
    assert_eq!(response._shards.failed, 0);
    assert!(response._shards.total >= 1);

    // Test auto-generated ID (no ID specified)
    let response = fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .options(IndexOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    assert_eq!(response.index, index_name);
    assert!(
        !response.id.is_empty(),
        "Auto-generated ID should not be empty"
    );
    assert_eq!(response.result, "created");

    Ok(())
}

#[tokio::test]
async fn test_get_document() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");
    let doc = TestDocument::new_sample();
    let doc_id = "test-doc-get";

    // First index a document
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id(doc_id)
        .options(IndexOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    // Then retrieve it
    let response = fixture
        .client
        .documents()
        .get::<TestDocument>(&index_name, doc_id)
        .send()
        .await?;

    assert!(response.is_some(), "Document should exist");

    let get_response = response.unwrap();
    assert_eq!(get_response.id, doc_id);
    assert_eq!(get_response.index, index_name);
    assert!(get_response.version.unwrap() > 0);
    assert!(get_response.primary_term.unwrap() > 0);
    assert_eq!(get_response.found, true);
    assert_eq!(get_response.source, Some(doc));

    // Test with nonexistent document
    let missing_response = fixture
        .client
        .documents()
        .get::<TestDocument>(&index_name, "nonexistent-doc")
        .send()
        .await?;

    assert!(
        missing_response.is_none(),
        "Nonexistent document should return None"
    );

    // Test with source filtering
    let filtered_response = fixture
        .client
        .documents()
        .get::<Value>(&index_name, doc_id)
        .source_includes(vec!["title".to_string(), "published".to_string()])
        .send()
        .await?
        .unwrap();

    let source = filtered_response.source.unwrap();
    assert!(
        source.get("title").is_some(),
        "Title field should be included"
    );
    assert!(
        source.get("published").is_some(),
        "Published field should be included"
    );
    assert!(
        source.get("content").is_none(),
        "Content field should be excluded"
    );
    assert!(
        source.get("tags").is_none(),
        "Tags field should be excluded"
    );

    Ok(())
}

#[tokio::test]
async fn test_update_document() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");
    let doc = TestDocument::new_sample();
    let doc_id = "test-doc-update";

    // First index a document
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id(doc_id)
        .options(IndexOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    // Update the document
    let updated_doc = TestDocument::updated_sample();
    let update_response = fixture
        .client
        .documents()
        .update(&index_name, doc_id, &updated_doc)
        .options(UpdateOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    assert_eq!(update_response.index, index_name);
    assert_eq!(update_response.id, doc_id);
    assert_eq!(update_response.result, "updated");
    assert!(update_response.primary_term > 0);
    assert!(update_response.seq_no > 0);
    assert_eq!(update_response._shards.successful, 1);
    assert_eq!(update_response._shards.failed, 0);
    assert!(update_response._shards.total >= 1);

    // Verify the update by getting the document
    let get_response = fixture
        .client
        .documents()
        .get::<TestDocument>(&index_name, doc_id)
        .send()
        .await?
        .unwrap();

    assert_eq!(get_response.source, Some(updated_doc));
    assert!(
        get_response.version.unwrap() > 1,
        "Version should be incremented after update"
    );

    // Test partial update with JSON
    let partial_update = json!({
        "title": "Partially Updated Document",
        "views": 25
    });

    fixture
        .client
        .documents()
        .update(&index_name, doc_id, &partial_update)
        .options(UpdateOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    let partial_response = fixture
        .client
        .documents()
        .get::<Value>(&index_name, doc_id)
        .send()
        .await?
        .unwrap();

    let source = partial_response.source.unwrap();
    assert_eq!(
        source.get("title").and_then(|v| v.as_str()),
        Some("Partially Updated Document")
    );
    assert_eq!(source.get("views").and_then(|v| v.as_u64()), Some(25));

    // Content should not be changed by partial update
    assert_eq!(
        source.get("content").and_then(|v| v.as_str()),
        Some("This document has been updated through the API")
    );

    // Test upsert (update or insert)
    let upsert_id = "nonexistent-doc-upsert";
    let upsert_doc = TestDocument::new_sample();

    fixture
        .client
        .documents()
        .update(&index_name, upsert_id, &upsert_doc)
        .doc_as_upsert(true)
        .options(UpdateOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    // Verify the upsert created a new document
    let upsert_response = fixture
        .client
        .documents()
        .get::<TestDocument>(&index_name, upsert_id)
        .send()
        .await?;

    assert!(
        upsert_response.is_some(),
        "Document should be created by upsert"
    );
    assert_eq!(upsert_response.unwrap().source, Some(upsert_doc));

    Ok(())
}

#[tokio::test]
async fn test_delete_document() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");
    let doc = TestDocument::new_sample();
    let doc_id = "test-doc-delete";

    // First index a document
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id(doc_id)
        .options(IndexOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    // Verify document exists
    let exists = fixture
        .client
        .documents()
        .exists(&index_name, doc_id)
        .send()
        .await?;

    assert!(exists, "Document should exist before deletion");

    // Delete the document
    let delete_response = fixture
        .client
        .documents()
        .delete(&index_name, doc_id)
        .options(DeleteOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    assert_eq!(delete_response.index, index_name);
    assert_eq!(delete_response.id, doc_id);
    assert_eq!(delete_response.result, "deleted");
    assert!(delete_response.primary_term > 0);
    assert!(delete_response.seq_no > 0);
    assert_eq!(delete_response._shards.successful, 1);
    assert_eq!(delete_response._shards.failed, 0);
    assert!(delete_response._shards.total >= 1);

    // Verify document no longer exists
    let exists_after_delete = fixture
        .client
        .documents()
        .exists(&index_name, doc_id)
        .send()
        .await?;

    assert!(
        !exists_after_delete,
        "Document should not exist after deletion"
    );

    // Delete a nonexistent document
    let missing_delete = fixture
        .client
        .documents()
        .delete(&index_name, "nonexistent-doc-delete")
        .options(DeleteOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    assert_eq!(missing_delete.result, "not_found");

    Ok(())
}

#[tokio::test]
async fn test_document_exists() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");
    let doc = TestDocument::new_sample();
    let doc_id = "test-doc-exists";

    // Check before document exists
    let exists_before = fixture
        .client
        .documents()
        .exists(&index_name, doc_id)
        .send()
        .await?;

    assert!(!exists_before, "Document should not exist initially");

    // Create document
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id(doc_id)
        .options(IndexOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    // Check after document exists
    let exists_after = fixture
        .client
        .documents()
        .exists(&index_name, doc_id)
        .send()
        .await?;

    assert!(exists_after, "Document should exist after indexing");

    // Test exists with routing
    let doc_with_routing = "test-doc-routing";
    let routing_value = "user123";

    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id(doc_with_routing)
        .options(IndexOptions::builder().refresh("true").routing(routing_value).build()?)
        .send()
        .await?;

    // Check with correct routing
    let exists_with_routing = fixture
        .client
        .documents()
        .exists(&index_name, doc_with_routing)
        .routing(routing_value)
        .send()
        .await?;

    assert!(
        exists_with_routing,
        "Document should exist with correct routing"
    );

    Ok(())
}

#[tokio::test]
async fn test_refresh_index() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");

    // Index a document without refresh
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&TestDocument::new_sample())
        .id("test-refresh-doc")
        .send()
        .await?;

    // Issue explicit refresh
    let refresh_response = fixture
        .client
        .documents()
        .refresh(&index_name)
        .send()
        .await?;

    // Verify refresh was successful
    assert!(refresh_response.is_object());
    assert!(refresh_response.get("_shards").is_some());

    // Verify document is now visible
    let exists = fixture
        .client
        .documents()
        .exists(&index_name, "test-refresh-doc")
        .send()
        .await?;

    assert!(exists, "Document should be visible after refresh");

    Ok(())
}

#[tokio::test]
async fn test_index_options() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");
    let doc = TestDocument::new_sample();

    // Test wait_for_active_shards option
    let response = fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id("test-options-doc")
        .options(IndexOptions::builder().refresh("true").wait_for_active_shards(WaitForActiveShards::Count(1)).build()?)
        .send()
        .await?;

    assert_eq!(response.result, "created");

    // Test with timeout option
    let response = fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id("test-timeout-doc")
        .options(IndexOptions::builder().refresh("true").timeout("5s").build()?)
        .send()
        .await?;

    assert_eq!(response.result, "created");

    Ok(())
}

#[tokio::test]
async fn test_get_options() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");
    let doc = TestDocument::new_sample();
    let doc_id = "test-get-options";

    // First index a document
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id(doc_id)
        .options(IndexOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    // Test with realtime option set to false
    let realtime_response = fixture
        .client
        .documents()
        .get::<TestDocument>(&index_name, doc_id)
        .realtime(false)
        .send()
        .await?;

    assert!(realtime_response.is_some());

    // Test with preference option
    let preference_response = fixture
        .client
        .documents()
        .get::<TestDocument>(&index_name, doc_id)
        .preference("_local")
        .send()
        .await?;

    assert!(preference_response.is_some());

    Ok(())
}

#[tokio::test]
async fn test_update_options() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");
    let doc = TestDocument::new_sample();
    let doc_id = "test-update-options";

    // First index a document
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id(doc_id)
        .options(IndexOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    // Test retry_on_conflict option
    let updated_doc = TestDocument::updated_sample();
    let update_response = fixture
        .client
        .documents()
        .update(&index_name, doc_id, &updated_doc)
        .retry_on_conflict(3)
        .options(UpdateOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    assert_eq!(update_response.result, "updated");

    // Test timeout option
    let timeout_response = fixture
        .client
        .documents()
        .update(&index_name, doc_id, &json!({"views": 50}))
        .timeout("5s")
        .options(UpdateOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    assert_eq!(timeout_response.result, "updated");

    Ok(())
}

#[tokio::test]
async fn test_delete_options() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");
    let doc = TestDocument::new_sample();
    let doc_id = "test-delete-options";

    // First index a document
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id(doc_id)
        .options(IndexOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    // Test with timeout option
    let delete_response = fixture
        .client
        .documents()
        .delete(&index_name, doc_id)
        .timeout("5s")
        .options(DeleteOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    assert_eq!(delete_response.result, "deleted");

    Ok(())
}

#[tokio::test]
async fn test_document_operations_with_routing() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("docs");
    let doc = TestDocument::new_sample();
    let doc_id = "test-routing";
    let routing_value = "user456";

    // Index with routing
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id(doc_id)
        .options(IndexOptions::builder().refresh("true").routing(routing_value).build()?)
        .send()
        .await?;

    // Get with routing
    let get_response = fixture
        .client
        .documents()
        .get::<TestDocument>(&index_name, doc_id)
        .routing(routing_value)
        .send()
        .await?;

    assert!(get_response.is_some());

    // Get without routing (should still work for this test setup)
    let get_no_routing = fixture
        .client
        .documents()
        .get::<TestDocument>(&index_name, doc_id)
        .send()
        .await?;

    assert!(get_no_routing.is_some());

    // Update with routing
    fixture
        .client
        .documents()
        .update(&index_name, doc_id, &TestDocument::updated_sample())
        .routing(routing_value)
        .options(UpdateOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    // Delete with routing
    let delete_response = fixture
        .client
        .documents()
        .delete(&index_name, doc_id)
        .routing(routing_value)
        .options(DeleteOptions::builder().refresh("true").build()?)
        .send()
        .await?;

    assert_eq!(delete_response.result, "deleted");

    Ok(())
}
