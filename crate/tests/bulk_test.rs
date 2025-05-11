//! Tests for bulk operations in OpenSearch

pub mod fixture;

use crate::fixture::OpenSearchFixture;
use anyhow::Result;
use opensearch_api::types::common::RefreshPolicy;
use opensearch_api::types::script::{InlineScript, Script};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

/// Test document structure for bulk tests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestDocument {
    title: String,
    content: String,
    tags: Vec<String>,
    published: bool,
    views: u32,
}

impl TestDocument {
    fn new_sample(index: usize) -> Self {
        Self {
            title: format!("Test Document {}", index),
            content: format!("This is test document {} for OpenSearch API", index),
            tags: vec![
                "test".to_string(),
                format!("doc{}", index),
                "opensearch".to_string(),
            ],
            published: true,
            views: index as u32,
        }
    }

    fn updated_sample(index: usize) -> Self {
        Self {
            title: format!("Updated Document {}", index),
            content: format!("Document {} has been updated via bulk API", index),
            tags: vec![
                "test".to_string(),
                format!("updated{}", index),
                "opensearch".to_string(),
            ],
            published: true,
            views: (index * 10) as u32,
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
async fn test_bulk_index_operations() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("bulk-docs");

    // Create multiple documents for bulk indexing
    let doc1 = TestDocument::new_sample(1);
    let doc2 = TestDocument::new_sample(2);
    let doc3 = TestDocument::new_sample(3);

    // Test bulk index operation with explicit IDs
    let response = fixture
        .client
        .bulk()
        .index(&index_name, Some("doc-1"), &doc1)
        .index(&index_name, Some("doc-2"), &doc2)
        .index(&index_name, Some("doc-3"), &doc3)
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    // Verify the response
    assert_eq!(
        response.errors, false,
        "Bulk index operations should not have errors"
    );
    assert_eq!(
        response.items.len(),
        3,
        "Should have 3 items in the response"
    );

    // Check each item in the response
    for (i, item) in response.items.iter().enumerate() {
        if let Some(index_op) = &item.index {
            assert_eq!(index_op.index, index_name);
            assert_eq!(index_op.id, format!("doc-{}", i + 1));
            assert_eq!(index_op.status, 201, "Status should be 201 Created");
            assert_eq!(index_op.result, Some("created".to_string()));
            assert!(index_op.primary_term.is_some());
            assert!(index_op.seq_no.is_some());
        } else {
            panic!("Expected index operation result");
        }
    }

    // Verify the documents were indexed by fetching them
    for i in 1..=3 {
        let get_response = fixture
            .client
            .documents()
            .get::<TestDocument>(&index_name, &format!("doc-{}", i))
            .build()?
            .send()
            .await?;

        assert!(get_response.is_some(), "Document doc-{} should exist", i);

        let doc = get_response.unwrap().source.unwrap();
        assert_eq!(doc.title, format!("Test Document {}", i));
    }

    Ok(())
}

#[tokio::test]
async fn test_bulk_create_operations() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("bulk-create");

    // Create multiple documents for bulk create
    let doc1 = TestDocument::new_sample(1);
    let doc2 = TestDocument::new_sample(2);

    // Test bulk create operation
    let response = fixture
        .client
        .bulk()
        .create(&index_name, Some("create-1"), &doc1)
        .create(&index_name, Some("create-2"), &doc2)
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    // Verify the response
    assert_eq!(response.errors, false);
    assert_eq!(response.items.len(), 2);

    for (i, item) in response.items.iter().enumerate() {
        if let Some(create_op) = &item.create {
            assert_eq!(create_op.index, index_name);
            assert_eq!(create_op.id, format!("create-{}", i + 1));
            assert_eq!(create_op.status, 201);
            assert_eq!(create_op.result, Some("created".to_string()));
        } else {
            panic!("Expected create operation result");
        }
    }

    // Verify the documents exist
    for i in 1..=2 {
        let exists = fixture
            .client
            .documents()
            .exists(&index_name, &format!("create-{}", i))
            .build()?
            .send()
            .await?;

        assert!(exists, "Document create-{} should exist", i);
    }

    // Test create conflict (try to create documents that already exist)
    let conflict_response = fixture
        .client
        .bulk()
        .create(&index_name, Some("create-1"), &doc1)
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    // Verify the response shows errors
    assert_eq!(
        conflict_response.errors, true,
        "Should have errors due to document already existing"
    );
    if let Some(create_op) = &conflict_response.items[0].create {
        assert_eq!(create_op.status, 409, "Status should be 409 Conflict");
        assert!(create_op.error.is_some(), "Should have error details");
    }

    Ok(())
}

#[tokio::test]
async fn test_bulk_update_operations() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("bulk-update");

    // First, create documents to update
    let doc1 = TestDocument::new_sample(1);
    let doc2 = TestDocument::new_sample(2);

    fixture
        .client
        .bulk()
        .index(&index_name, Some("update-1"), &doc1)
        .index(&index_name, Some("update-2"), &doc2)
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    // Now perform bulk update
    let updated_doc1 = TestDocument::updated_sample(1);
    let updated_doc2 = TestDocument::updated_sample(2);

    let update_response = fixture
        .client
        .bulk()
        .update(&index_name, "update-1", &updated_doc1)
        .update(&index_name, "update-2", &updated_doc2)
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    // Verify the response
    assert_eq!(update_response.errors, false);
    assert_eq!(update_response.items.len(), 2);

    for (i, item) in update_response.items.iter().enumerate() {
        if let Some(update_op) = &item.update {
            assert_eq!(update_op.index, index_name);
            assert_eq!(update_op.id, format!("update-{}", i + 1));
            assert_eq!(update_op.status, 200);
            assert_eq!(update_op.result, Some("updated".to_string()));
        } else {
            panic!("Expected update operation result");
        }
    }

    // Verify the documents were updated by fetching them
    for i in 1..=2 {
        let get_response = fixture
            .client
            .documents()
            .get::<TestDocument>(&index_name, &format!("update-{}", i))
            .build()?
            .send()
            .await?;

        let doc = get_response.unwrap().source.unwrap();
        assert_eq!(doc.title, format!("Updated Document {}", i));
        assert_eq!(doc.views, (i * 10) as u32);
    }

    Ok(())
}

#[tokio::test]
async fn test_bulk_delete_operations() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("bulk-delete");

    // First, create documents to delete
    let doc1 = TestDocument::new_sample(1);
    let doc2 = TestDocument::new_sample(2);
    let doc3 = TestDocument::new_sample(3);

    fixture
        .client
        .bulk()
        .index(&index_name, Some("delete-1"), &doc1)
        .index(&index_name, Some("delete-2"), &doc2)
        .index(&index_name, Some("delete-3"), &doc3)
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    // Verify documents exist before deletion
    for i in 1..=3 {
        let exists = fixture
            .client
            .documents()
            .exists(&index_name, &format!("delete-{}", i))
            .build()?
            .send()
            .await?;

        assert!(exists, "Document delete-{} should exist before deletion", i);
    }

    // Perform bulk delete for doc1 and doc2
    let delete_response = fixture
        .client
        .bulk::<()>()
        .delete::<TestDocument>(&index_name, "delete-1")
        .delete::<TestDocument>(&index_name, "delete-2")
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    // Verify the response
    assert_eq!(delete_response.errors, false);
    assert_eq!(delete_response.items.len(), 2);

    for (i, item) in delete_response.items.iter().enumerate() {
        if let Some(delete_op) = &item.delete {
            assert_eq!(delete_op.index, index_name);
            assert_eq!(delete_op.id, format!("delete-{}", i + 1));
            assert_eq!(delete_op.status, 200);
            assert_eq!(delete_op.result, Some("deleted".to_string()));
        } else {
            panic!("Expected delete operation result");
        }
    }

    // Verify doc1 and doc2 no longer exist, but doc3 still does
    for i in 1..=3 {
        let exists = fixture
            .client
            .documents()
            .exists(&index_name, &format!("delete-{}", i))
            .build()?
            .send()
            .await?;

        if i <= 2 {
            assert!(
                !exists,
                "Document delete-{} should not exist after deletion",
                i
            );
        } else {
            assert!(exists, "Document delete-3 should still exist");
        }
    }

    // Try to delete a non-existent document
    let nonexistent_delete = fixture
        .client
        .bulk::<()>()
        .delete::<TestDocument>(&index_name, "nonexistent")
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    // Verify the response
    if let Some(delete_op) = &nonexistent_delete.items[0].delete {
        assert_eq!(delete_op.status, 404);
        assert_eq!(delete_op.result, Some("not_found".to_string()));
    } else {
        panic!("Expected delete operation result");
    }

    Ok(())
}

#[tokio::test]
async fn test_mixed_bulk_operations() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("bulk-mixed");

    // Create a document first for the update and delete operations
    let initial_doc = TestDocument::new_sample(5);

    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&initial_doc)
        .id("existing-doc")
        .options(
            opensearch_api::types::document::IndexOptions::builder()
                .refresh("true")
                .build()?,
        )
        .build()?
        .send()
        .await?;

    // Perform a mix of operations in a single bulk request
    let new_doc1 = TestDocument::new_sample(1);
    let new_doc2 = TestDocument::new_sample(2);
    let updated_doc = TestDocument::updated_sample(5);

    let mixed_response = fixture
        .client
        .bulk()
        .index(&index_name, Some("new-doc-1"), &new_doc1)
        .create(&index_name, Some("new-doc-2"), &new_doc2)
        .update(&index_name, "existing-doc", &updated_doc)
        .delete::<TestDocument>(&index_name, "new-doc-1") // Delete a document we just indexed
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    // Verify the response
    assert_eq!(mixed_response.errors, false);
    assert_eq!(mixed_response.items.len(), 4);

    // Check that operations were processed in order
    // Index operation
    if let Some(index_op) = &mixed_response.items[0].index {
        assert_eq!(index_op.id, "new-doc-1");
        assert_eq!(index_op.result, Some("created".to_string()));
    } else {
        panic!("Expected index operation result");
    }

    // Create operation
    if let Some(create_op) = &mixed_response.items[1].create {
        assert_eq!(create_op.id, "new-doc-2");
        assert_eq!(create_op.result, Some("created".to_string()));
    } else {
        panic!("Expected create operation result");
    }

    // Update operation
    if let Some(update_op) = &mixed_response.items[2].update {
        assert_eq!(update_op.id, "existing-doc");
        assert_eq!(update_op.result, Some("updated".to_string()));
    } else {
        panic!("Expected update operation result");
    }

    // Delete operation
    if let Some(delete_op) = &mixed_response.items[3].delete {
        assert_eq!(delete_op.id, "new-doc-1");
        assert_eq!(delete_op.result, Some("deleted".to_string()));
    } else {
        panic!("Expected delete operation result");
    }

    // Verify final state of documents
    assert!(
        !fixture
            .client
            .documents()
            .exists(&index_name, "new-doc-1")
            .build()?
            .send()
            .await?,
        "new-doc-1 should be deleted"
    );

    assert!(
        fixture
            .client
            .documents()
            .exists(&index_name, "new-doc-2")
            .build()?
            .send()
            .await?,
        "new-doc-2 should exist"
    );

    let updated = fixture
        .client
        .documents()
        .get::<TestDocument>(&index_name, "existing-doc")
        .build()?
        .send()
        .await?
        .unwrap()
        .source
        .unwrap();

    assert_eq!(updated.title, "Updated Document 5");

    Ok(())
}

#[tokio::test]
async fn test_bulk_with_script_update() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("bulk-script");

    // First create a document to update
    let doc = TestDocument::new_sample(1);
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id("script-doc")
        .options(
            opensearch_api::types::document::IndexOptions::builder()
                .refresh("true")
                .build()?,
        )
        .build()?
        .send()
        .await?;

    // Create script to increment views
    let mut params = HashMap::new();
    params.insert("count".to_string(), json!(15));

    let script = Script::Inline(InlineScript {
        source: "ctx._source.views += params.count".to_string(),
        lang: Some("painless".to_string()),
        params: Some(params),
        options: None,
    });

    // Update using script
    let response = fixture
        .client
        .bulk::<()>()
        .update_with_script(&index_name, "script-doc", script)
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    assert_eq!(response.errors, false);
    if let Some(update_op) = &response.items[0].update {
        assert_eq!(update_op.result, Some("updated".to_string()));
    }

    // Verify the update
    let updated_doc = fixture
        .client
        .documents()
        .get::<TestDocument>(&index_name, "script-doc")
        .build()?
        .send()
        .await?
        .unwrap()
        .source
        .unwrap();

    assert_eq!(updated_doc.views, 16, "Views should be incremented by 15");

    Ok(())
}

#[tokio::test]
async fn test_bulk_with_update_document() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("bulk-update-doc");

    // First create a document to update
    let doc = TestDocument::new_sample(1);
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id("update-doc")
        .options(
            opensearch_api::types::document::IndexOptions::builder()
                .refresh("true")
                .build()?,
        )
        .build()?
        .send()
        .await?;

    // Update with partial document and upsert
    let partial_update = json!({
        "title": "Partially Updated Document",
        "views": 25
    });

    let response = fixture
        .client
        .bulk::<Value>()
        .update_with_document(&index_name, "update-doc", |doc| {
            doc.doc(partial_update.clone())
        })
        .update_with_document(&index_name, "nonexistent-doc", |doc| {
            doc.doc(partial_update.clone()).doc_as_upsert(true)
        })
        .refresh(RefreshPolicy::Wait)
        .send()
        .await?;

    assert_eq!(response.errors, false);

    // Verify the partial update
    let updated_doc = fixture
        .client
        .documents()
        .get::<Value>(&index_name, "update-doc")
        .build()?
        .send()
        .await?
        .unwrap()
        .source
        .unwrap();

    assert_eq!(updated_doc["title"], "Partially Updated Document");
    assert_eq!(updated_doc["views"], 25);
    // Original content should be preserved
    assert_eq!(
        updated_doc["content"],
        "This is test document 1 for OpenSearch API"
    );

    // Verify the upsert
    let upserted_doc = fixture
        .client
        .documents()
        .get::<Value>(&index_name, "nonexistent-doc")
        .build()?
        .send()
        .await?
        .unwrap()
        .source
        .unwrap();

    assert_eq!(upserted_doc, partial_update);

    Ok(())
}

#[tokio::test]
async fn test_bulk_with_error_handling() -> Result<()> {
    let fixture = setup_fixture().await?;
    let index_name = fixture.namespaced_index("bulk-errors");

    // Create a document
    let doc = TestDocument::new_sample(1);
    fixture
        .client
        .documents()
        .index(&index_name)
        .document(&doc)
        .id("error-doc")
        .options(
            opensearch_api::types::document::IndexOptions::builder()
                .refresh("true")
                .build()?,
        )
        .build()?
        .send()
        .await?;

    // Create a bulk request with some expected failures
    let response = fixture
        .client
        .bulk()
        // This should succeed
        .index(&index_name, Some("success-doc"), &doc)
        // This should fail (trying to create a document that already exists)
        .create(&index_name, Some("error-doc"), &doc)
        // This should fail (trying to update a non-existent document)
        .update(&index_name, "nonexistent-doc", &doc)
        .refresh(RefreshPolicy::True)
        .send()
        .await?;

    // Verify the response contains errors
    assert_eq!(response.errors, true, "Should have errors in the response");
    assert_eq!(response.items.len(), 3);

    // First operation should succeed
    if let Some(index_op) = &response.items[0].index {
        assert_eq!(index_op.status, 201);
        assert_eq!(index_op.result, Some("created".to_string()));
        assert!(index_op.error.is_none());
    }

    // Second operation should fail - duplicate document
    if let Some(create_op) = &response.items[1].create {
        assert_eq!(create_op.status, 409);
        assert!(create_op.error.is_some());
    }

    // Third operation should fail - document not found
    if let Some(update_op) = &response.items[2].update {
        assert_eq!(update_op.status, 404);
        assert!(update_op.error.is_some());
    }

    Ok(())
}
