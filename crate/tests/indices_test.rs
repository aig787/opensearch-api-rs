//! Tests for indices namespace functionality

pub mod fixture;

use crate::fixture::OpenSearchFixture;
use anyhow::Result;
use opensearch_api::indices::{AddAliasAction, AliasAction, IndexSettings, RemoveAliasAction};
use serde_json::json;
use std::collections::HashMap;

/// Helper function to create a basic test index
async fn create_test_index(fixture: &OpenSearchFixture, index_name: &str) -> Result<()> {
    let settings = IndexSettings::builder()
        .number_of_shards(1)
        .number_of_replicas(0)
        .refresh_interval("1s")
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build settings: {}", e))?;

    let response = fixture
        .client
        .indices()
        .create(index_name)
        .settings(settings)
        .build()?
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create index: {}", e))?;

    assert!(response.acknowledged, "Index creation was not acknowledged");
    Ok(())
}

/// Helper function to create multiple test indices
async fn create_multiple_test_indices(
    fixture: &OpenSearchFixture,
    index_names: &[&str],
) -> Result<()> {
    for index_name in index_names {
        create_test_index(fixture, index_name).await?;
    }
    Ok(())
}

#[tokio::test]
async fn test_index_exists() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = fixture.namespaced_index("exists_test");

    // Test non-existent index
    let exists = fixture
        .client
        .indices()
        .exists(&index_name)
        .build()?
        .send()
        .await?;
    assert!(!exists, "Index should not exist initially");

    // Create the index
    create_test_index(&fixture, &index_name).await?;

    // Test existing index
    let exists = fixture
        .client
        .indices()
        .exists(&index_name)
        .build()?
        .send()
        .await?;
    assert!(exists, "Index should exist after creation");

    Ok(())
}

#[tokio::test]
async fn test_multiple_indices_exist() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let prefix = "multi_exists_test";
    let index_names: Vec<String> = (1..=3)
        .map(|i| fixture.namespaced_index(&format!("{}_{}", prefix, i)))
        .collect();

    // Test non-existent indices
    let exists = fixture
        .client
        .indices()
        .exists(&index_names)
        .build()?
        .send()
        .await?;
    assert!(!exists, "Indices should not exist initially");

    // Create indices
    create_multiple_test_indices(
        &fixture,
        &index_names
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>(),
    )
    .await?;

    // Test all indices exist
    let exists = fixture
        .client
        .indices()
        .exists(&index_names)
        .build()?
        .send()
        .await?;
    assert!(exists, "All indices should exist after creation");

    // Create a list with one real and one non-existent index
    let mixed_indices = vec![&index_names[0], "non_existent_index"];
    let exists = fixture
        .client
        .indices()
        .exists(&mixed_indices)
        .build()?
        .send()
        .await?;
    assert!(!exists, "When any index doesn't exist, should return false");

    // Clean up
    for index_name in &index_names {
        fixture
            .client
            .indices()
            .delete(index_name)
            .build()?
            .send()
            .await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_create_and_delete_index() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = fixture.namespaced_index("create_delete_test");

    // Create index with custom settings
    let mut settings_builder = IndexSettings::builder();
    settings_builder
        .number_of_shards(1)
        .number_of_replicas(0)
        .refresh_interval("5s");

    // Create mappings
    let mappings = json!({
        "properties": {
            "title": {
                "type": "text",
            },
            "content": {
                "type": "text"
            },
            "date": {
                "type": "date"
            }
        }
    });

    let alias_name = format!("{}_alias", index_name);

    // Create the index
    let create_response = fixture
        .client
        .indices()
        .create(&index_name)
        .mappings(mappings)
        .aliases([&alias_name])
        .build()?
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create index: {}", e))?;

    assert!(
        create_response.acknowledged,
        "Index creation not acknowledged"
    );

    // Verify index exists
    let exists = fixture
        .client
        .indices()
        .exists(&index_name)
        .build()?
        .send()
        .await?;
    assert!(exists, "Index should exist after creation");

    // Delete the index
    let delete_response = fixture
        .client
        .indices()
        .delete(&index_name)
        .build()?
        .send()
        .await?;

    assert!(delete_response.acknowledged);

    // Verify index no longer exists
    let exists = fixture
        .client
        .indices()
        .exists(&index_name)
        .build()?
        .send()
        .await?;
    assert!(!exists, "Index should not exist after deletion");

    Ok(())
}

#[tokio::test]
async fn test_create_and_delete_multiple_indices() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let prefix = "multi_create_delete_test";
    let index_names: Vec<String> = (1..=3)
        .map(|i| fixture.namespaced_index(&format!("{}_{}", prefix, i)))
        .collect();

    // Create multiple indices with basic settings
    for index_name in &index_names {
        create_test_index(&fixture, index_name).await?;
    }

    // Verify all indices exist
    let exists = fixture
        .client
        .indices()
        .exists(&index_names)
        .build()?
        .send()
        .await?;
    assert!(exists, "All indices should exist after creation");

    // Delete multiple indices in one call
    let delete_response = fixture
        .client
        .indices()
        .delete(&index_names)
        .build()?
        .send()
        .await?;

    assert!(
        delete_response.acknowledged,
        "Multi-index deletion should be acknowledged"
    );

    // Verify no indices exist after deletion
    let exists = fixture
        .client
        .indices()
        .exists(&index_names)
        .build()?
        .send()
        .await?;
    assert!(!exists, "No indices should exist after deletion");

    Ok(())
}

#[tokio::test]
async fn test_close_and_open_index() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = fixture.namespaced_index("close_open_test");

    // Create the index
    create_test_index(&fixture, &index_name).await?;

    // Close the index
    let close_response = fixture
        .client
        .indices()
        .close(&index_name)
        .build()?
        .send()
        .await?;

    assert!(close_response.acknowledged);

    // Open the index
    let open_response = fixture
        .client
        .indices()
        .open(&index_name)
        .build()?
        .send()
        .await?;

    assert!(open_response.acknowledged);

    // Clean up
    fixture
        .client
        .indices()
        .delete(&index_name)
        .build()?
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_close_and_open_multiple_indices() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let prefix = "multi_close_open_test";
    let index_names: Vec<String> = (1..=3)
        .map(|i| fixture.namespaced_index(&format!("{}_{}", prefix, i)))
        .collect();

    // Create multiple indices with basic settings
    create_multiple_test_indices(
        &fixture,
        &index_names
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>(),
    )
    .await?;

    // Close multiple indices in one call
    let close_response = fixture
        .client
        .indices()
        .close(&index_names)
        .build()?
        .send()
        .await?;

    assert!(
        close_response.acknowledged,
        "Closing multiple indices should be acknowledged"
    );

    // Open multiple indices in one call
    let open_response = fixture
        .client
        .indices()
        .open(&index_names)
        .build()?
        .send()
        .await?;

    assert!(
        open_response.acknowledged,
        "Opening multiple indices should be acknowledged"
    );

    // Clean up
    fixture
        .client
        .indices()
        .delete(&index_names)
        .build()?
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_get_and_update_settings() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = fixture.namespaced_index("settings_test");

    // Create the index with initial settings
    let mut settings_builder = IndexSettings::builder();
    settings_builder
        .number_of_shards(1)
        .number_of_replicas(0)
        .refresh_interval("1s");

    let create_response = fixture
        .client
        .indices()
        .create(&index_name)
        .settings(settings_builder.build().unwrap())
        .build()?
        .send()
        .await?;

    assert!(create_response.acknowledged);

    // Get settings
    let settings_response = fixture
        .client
        .indices()
        .get_settings(&index_name)
        .build()?
        .send()
        .await?;

    // Verify settings
    let index_settings = &settings_response[&index_name].settings.index;
    assert_eq!(index_settings.number_of_shards, 1);
    assert_eq!(index_settings.number_of_replicas, 0);
    assert_eq!(index_settings.refresh_interval.as_ref().unwrap(), "1s");

    // Update settings
    let mut new_settings = HashMap::new();
    new_settings.insert("number_of_replicas".to_string(), json!(1));
    new_settings.insert("refresh_interval".to_string(), json!("5s"));

    let update_response = fixture
        .client
        .indices()
        .update_settings(&index_name)
        .settings(new_settings)
        .build()?
        .send()
        .await?;

    assert!(update_response.acknowledged);

    // Get updated settings
    let updated_settings = fixture
        .client
        .indices()
        .get_settings(&index_name)
        .build()?
        .send()
        .await?;

    // Verify updated settings
    let updated_index_settings = &updated_settings[&index_name].settings.index;
    assert_eq!(updated_index_settings.number_of_replicas, 1);
    assert_eq!(
        updated_index_settings.refresh_interval.as_ref().unwrap(),
        "5s"
    );

    // Clean up
    fixture
        .client
        .indices()
        .delete(&index_name)
        .build()?
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_get_and_update_multiple_indices_settings() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let prefix = "multi_settings_test";
    let index_names: Vec<String> = (1..=3)
        .map(|i| fixture.namespaced_index(&format!("{}_{}", prefix, i)))
        .collect();

    // Create multiple indices with specific settings
    for index_name in &index_names {
        let settings = IndexSettings::builder()
            .number_of_shards(1)
            .number_of_replicas(0)
            .refresh_interval("1s")
            .build()?;

        fixture
            .client
            .indices()
            .create(index_name)
            .settings(settings)
            .build()?
            .send()
            .await?;
    }

    // Get settings for multiple indices
    let settings_response = fixture
        .client
        .indices()
        .get_settings(&index_names)
        .build()?
        .send()
        .await?;

    // Verify settings for all indices
    for index_name in &index_names {
        let index_settings = &settings_response[index_name].settings.index;
        assert_eq!(index_settings.number_of_shards, 1);
        assert_eq!(index_settings.number_of_replicas, 0);
        assert_eq!(index_settings.refresh_interval.as_ref().unwrap(), "1s");
    }

    // Update settings for multiple indices
    let mut new_settings = HashMap::new();
    new_settings.insert("number_of_replicas".to_string(), json!(1));
    new_settings.insert("refresh_interval".to_string(), json!("5s"));

    let update_response = fixture
        .client
        .indices()
        .update_settings(&index_names)
        .settings(new_settings)
        .build()?
        .send()
        .await?;

    assert!(update_response.acknowledged);

    // Get updated settings
    let updated_settings = fixture
        .client
        .indices()
        .get_settings(&index_names)
        .build()?
        .send()
        .await?;

    // Verify updated settings for all indices
    for index_name in &index_names {
        let updated_index_settings = &updated_settings[index_name].settings.index;
        assert_eq!(updated_index_settings.number_of_replicas, 1);
        assert_eq!(
            updated_index_settings.refresh_interval.as_ref().unwrap(),
            "5s"
        );
    }

    // Clean up
    fixture
        .client
        .indices()
        .delete(&index_names)
        .build()?
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_mappings() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = fixture.namespaced_index("mappings_test");

    // Create the index
    create_test_index(&fixture, &index_name).await?;

    // Create initial mapping
    let mut properties = HashMap::new();
    properties.insert(
        "title".to_string(),
        json!({
            "type": "text"
        }),
    );

    properties.insert(
        "content".to_string(),
        json!({
            "type": "text"
        }),
    );

    let put_mapping_response = fixture
        .client
        .indices()
        .put_mapping(&index_name)
        .properties(properties)
        .build()?
        .send()
        .await?;

    assert!(put_mapping_response.acknowledged);

    // Get mappings
    let mappings = fixture
        .client
        .indices()
        .get_mapping(&index_name)
        .build()?
        .send()
        .await?;

    // Verify mappings
    let index_mappings = &mappings[&index_name].mappings;
    assert_eq!(
        index_mappings["properties"]["title"]["type"]
            .as_str()
            .unwrap(),
        "text"
    );
    assert_eq!(
        index_mappings["properties"]["content"]["type"]
            .as_str()
            .unwrap(),
        "text"
    );

    // Update mappings with additional date field with proper format
    let mut updated_properties = HashMap::new();
    updated_properties.insert(
        "date".to_string(),
        json!({
            "type": "date",
            "format": "yyyy-MM-dd"
        }),
    );

    let update_mapping_response = fixture
        .client
        .indices()
        .put_mapping(&index_name)
        .properties(updated_properties)
        .build()?
        .send()
        .await?;

    assert!(update_mapping_response.acknowledged);

    // Get updated mappings
    let updated_mappings = fixture
        .client
        .indices()
        .get_mapping(&index_name)
        .build()?
        .send()
        .await?;

    // Verify updated mappings
    let updated_index_mappings = &updated_mappings[&index_name].mappings;
    assert_eq!(
        updated_index_mappings["properties"]["title"]["type"]
            .as_str()
            .unwrap(),
        "text"
    );
    assert_eq!(
        updated_index_mappings["properties"]["content"]["type"]
            .as_str()
            .unwrap(),
        "text"
    );
    assert_eq!(
        updated_index_mappings["properties"]["date"]["type"]
            .as_str()
            .unwrap(),
        "date"
    );

    // Clean up
    fixture
        .client
        .indices()
        .delete(&index_name)
        .build()?
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_aliases() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = fixture.namespaced_index("aliases_test");
    let alias_name = format!("{}_alias", index_name);

    // Create the index
    create_test_index(&fixture, &index_name).await?;

    // Add alias
    let action = AddAliasAction::builder()
        .index(index_name.clone())
        .alias(alias_name.clone())
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build alias action: {}", e))?;

    let alias_response = fixture
        .client
        .indices()
        .update_aliases()
        .actions(vec![AliasAction::Add { add: action }])
        .build()?
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to update aliases: {}", e))?;
    assert!(alias_response.acknowledged);

    // Get aliases
    let aliases = fixture
        .client
        .indices()
        .get_aliases(&index_name)
        .build()?
        .send()
        .await?;

    // Verify aliases
    assert!(aliases[&index_name].aliases.contains_key(&alias_name));

    // Remove alias
    let mut remove_alias_request = fixture.client.indices().update_aliases();

    let remove_actions = vec![AliasAction::Remove {
        remove: RemoveAliasAction {
            index: index_name.clone(),
            alias: alias_name.clone(),
        },
    }];

    let remove_response = remove_alias_request
        .actions(remove_actions)
        .build()?
        .send()
        .await?;
    assert!(remove_response.acknowledged);

    // Get aliases after removal
    let aliases_after = fixture
        .client
        .indices()
        .get_aliases(&index_name)
        .build()?
        .send()
        .await?;

    // Verify alias is removed
    assert!(aliases_after[&index_name].aliases.is_empty());

    // Clean up
    fixture
        .client
        .indices()
        .delete(&index_name)
        .build()?
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_refresh_index() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = fixture.namespaced_index("refresh_test");

    // Create the index
    create_test_index(&fixture, &index_name).await?;

    // Refresh the index
    let refresh_response = fixture
        .client
        .indices()
        .refresh(&index_name)
        .build()?
        .send()
        .await?;

    // Verify refresh response
    assert_eq!(refresh_response._shards.successful, 1);

    // Clean up
    fixture
        .client
        .indices()
        .delete(&index_name)
        .build()?
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_refresh_multiple_indices() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let prefix = "multi_refresh_test";
    let index_names: Vec<String> = (1..=3)
        .map(|i| fixture.namespaced_index(&format!("{}_{}", prefix, i)))
        .collect();

    // Create multiple indices
    create_multiple_test_indices(
        &fixture,
        &index_names
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>(),
    )
    .await?;

    // Refresh multiple indices in one call
    let refresh_response = fixture
        .client
        .indices()
        .refresh(&index_names)
        .build()?
        .send()
        .await?;

    // Verify refresh response
    assert!(
        refresh_response._shards.successful >= 3,
        "All shards should be successfully refreshed"
    );

    // Clean up
    fixture
        .client
        .indices()
        .delete(&index_names)
        .build()?
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_alias_with_filter() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = fixture.namespaced_index("filtered_alias_test");
    let alias_name = format!("{}_filtered", index_name);

    // Create the index
    create_test_index(&fixture, &index_name).await?;

    // Create mapping for the test field
    let mut properties = HashMap::new();
    properties.insert(
        "type".to_string(),
        json!({
            "type": "keyword"
        }),
    );

    fixture
        .client
        .indices()
        .put_mapping(&index_name)
        .properties(properties)
        .build()?
        .send()
        .await?;

    // Add filtered alias
    let filter = json!({
        "term": {
            "type": "document"
        }
    });

    let mut alias_request = fixture.client.indices().update_aliases();

    let actions = vec![AliasAction::Add {
        add: AddAliasAction {
            index: index_name.clone(),
            alias: alias_name.clone(),
            filter: Some(filter),
            routing: None,
            is_write_index: None,
        },
    }];

    let alias_response = alias_request.actions(actions).build()?.send().await?;
    assert!(alias_response.acknowledged);

    // Clean up
    fixture
        .client
        .indices()
        .delete(&index_name)
        .build()?
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_combined_operations() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;
    let index_name = fixture.namespaced_index("combined_ops_test");

    // 1. Create index with settings, mappings, and aliases in one call
    let settings = IndexSettings::builder()
        .number_of_shards(1)
        .number_of_replicas(0)
        .build()?;

    let mappings = json!({
        "properties": {
            "title": { "type": "text" },
            "content": { "type": "text" },
            "tags": { "type": "keyword" }
        }
    });

    let alias_name = format!("{}_main", index_name);
    let create_response = fixture
        .client
        .indices()
        .create(&index_name)
        .settings(settings)
        .mappings(mappings)
        .aliases([&alias_name])
        .build()?
        .send()
        .await?;

    assert!(create_response.acknowledged);

    // 2. Update settings
    let mut new_settings = HashMap::new();
    new_settings.insert("refresh_interval".to_string(), json!("2s"));

    fixture
        .client
        .indices()
        .update_settings(&index_name)
        .settings(new_settings)
        .build()?
        .send()
        .await?;

    // 3. Add a field to mappings
    let mut new_properties = HashMap::new();
    new_properties.insert(
        "created_at".to_string(),
        json!({
            "type": "date"
        }),
    );

    fixture
        .client
        .indices()
        .put_mapping(&index_name)
        .properties(new_properties)
        .build()?
        .send()
        .await?;

    // 4. Add another alias
    let second_alias = format!("{}_secondary", index_name);

    let actions = vec![AliasAction::Add {
        add: AddAliasAction {
            index: index_name.clone(),
            alias: second_alias.clone(),
            filter: None,
            routing: None,
            is_write_index: None,
        },
    }];

    fixture
        .client
        .indices()
        .update_aliases()
        .actions(actions)
        .build()?
        .send()
        .await?;

    // 5. Refresh the index
    fixture
        .client
        .indices()
        .refresh(&index_name)
        .build()?
        .send()
        .await?;

    // 6. Verify the final state
    let settings_response = fixture
        .client
        .indices()
        .get_settings(&index_name)
        .build()?
        .send()
        .await?;

    let mappings_response = fixture
        .client
        .indices()
        .get_mapping(&index_name)
        .build()?
        .send()
        .await?;

    let aliases_response = fixture
        .client
        .indices()
        .get_aliases(&index_name)
        .build()?
        .send()
        .await?;

    // Verify settings
    let final_settings = &settings_response[&index_name].settings.index;
    assert_eq!(final_settings.number_of_shards, 1);
    assert_eq!(final_settings.number_of_replicas, 0);
    assert_eq!(final_settings.refresh_interval.as_ref().unwrap(), "2s");

    // Verify mappings
    let final_mappings = &mappings_response[&index_name].mappings;
    let properties = final_mappings["properties"].as_object().unwrap();
    assert!(properties.contains_key("title"));
    assert!(properties.contains_key("content"));
    assert!(properties.contains_key("tags"));
    assert!(properties.contains_key("created_at"));

    // Verify aliases
    let final_aliases = &aliases_response[&index_name].aliases;
    assert!(final_aliases.contains_key(&alias_name));
    assert!(final_aliases.contains_key(&second_alias));

    // Clean up
    fixture
        .client
        .indices()
        .delete(&index_name)
        .build()?
        .send()
        .await?;

    Ok(())
}
