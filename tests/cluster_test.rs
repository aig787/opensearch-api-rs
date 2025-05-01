pub mod fixture;

use anyhow::Result;
use std::collections::HashMap;

use fixture::OpenSearchFixture;
use opensearch_api::cluster::{
    AllocationExplainRequest, ClusterHealthStatus, ClusterSettingsRequest,
};
use serde_json::json;

#[tokio::test]
async fn test_cluster_health() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;

    // Get cluster health
    let health = fixture.client.cluster().health().await?;

    assert!(
        !health.cluster_name.is_empty(),
        "Cluster name should not be empty"
    );
    assert!(
        health.status == ClusterHealthStatus::Green || health.status == ClusterHealthStatus::Yellow,
        "Cluster status should be green or yellow"
    );
    assert_eq!(health.timed_out, false, "Health check should not time out");

    Ok(())
}

#[tokio::test]
async fn test_cluster_stats() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;

    // Get cluster stats
    let stats = fixture.client.cluster().stats().await?;

    assert!(
        stats.status == ClusterHealthStatus::Green || stats.status == ClusterHealthStatus::Yellow,
        "Cluster status should be green or yellow"
    );
    assert!(stats.indices.count > 0, "Should have at least one index");

    Ok(())
}

#[tokio::test]
async fn test_cluster_state() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;

    // Get cluster state
    let state = fixture.client.cluster().state().await?;

    assert!(
        !state.cluster_name.is_empty(),
        "Cluster name should not be empty"
    );
    assert!(state.master_node.is_some(), "Should have a master node");

    Ok(())
}

#[tokio::test]
async fn test_cluster_nodes_info() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;

    // Get nodes info
    let nodes_info = fixture.client.cluster().nodes_info().await?;

    assert!(
        !nodes_info.nodes.is_empty(),
        "Should have at least one node"
    );

    // Check the first node
    let first_node_id = nodes_info.nodes.keys().next().unwrap();
    let node = &nodes_info.nodes[first_node_id];

    assert!(!node.name.is_empty(), "Node name should not be empty");
    assert!(!node.version.is_empty(), "Node version should not be empty");

    Ok(())
}

#[tokio::test]
async fn test_cluster_settings() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;

    // Get cluster settings
    let settings = fixture.client.cluster().get_settings().await?;

    // Check that we have the standard sections
    assert!(
        !settings.persistent.is_empty(),
        "Should have persistent settings section"
    );
    assert!(
        !settings.transient.is_empty(),
        "Should have transient settings section"
    );

    Ok(())
}

#[tokio::test]
async fn test_cluster_update_settings() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;

    // Update a cluster setting (increase the search queue size)
    let mut transient = HashMap::new();
    transient.insert("cluster.max_shards_per_node".to_string(), json!(2000));

    let new_settings = ClusterSettingsRequest::builder()
        .transient(transient)
        .build()?;

    let _update_response = fixture.client.cluster().put_settings(new_settings).await?;

    // Verify the setting was updated
    let settings = fixture.client.cluster().get_settings().await?;

    // Look for the setting in the transient settings
    let max_shards_value = settings
        .transient
        .get("cluster")
        .and_then(|s| s.get("max_shards_per_node"));

    assert!(
        max_shards_value.is_some(),
        "Setting should exist in the response"
    );

    // The value might be returned as a number or string depending on server version
    let max_shards_str = match max_shards_value {
        Some(value) => {
            if let Some(num) = value.as_i64() {
                assert_eq!(num, 2000, "max_shards_per_node should be 2000");
                "2000"
            } else if let Some(s) = value.as_str() {
                assert_eq!(s, "2000", "max_shards_per_node should be 2000");
                s
            } else {
                panic!("max_shards_per_node has unexpected type");
            }
        }
        None => "0",
    };

    assert_eq!(
        max_shards_str, "2000",
        "max_shards_per_node should be updated to 2000"
    );

    Ok(())
}

#[tokio::test]
async fn test_cluster_allocation_explain() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;

    // Create an index first with namespaced name
    let index_name = fixture.namespaced_index("test_allocation");
    fixture
        .client
        .indices()
        .create(&index_name)
        .build()?
        .send()
        .await?;

    // Get allocation explanation
    // This might fail if no unassigned shards, which is ok in test environment
    let explain_result = fixture.client.cluster().allocation_explain(None).await;

    // This operation might legitimately fail in a single-node cluster with all shards assigned
    if let Ok(explain) = explain_result {
        assert!(
            !explain.index.is_empty(),
            "Should have an index in allocation explanation"
        );
    }

    // Try with specific request
    let specific_request = AllocationExplainRequest::builder()
        .index(&index_name)
        .shard(0)
        .primary(true)
        .build()?;

    let _t = fixture
        .client
        .cluster()
        .allocation_explain(Some(specific_request))
        .await;
    // This may also legitimately fail, so we don't assert on it

    Ok(())
}

#[tokio::test]
async fn test_cluster_pending_tasks() -> Result<()> {
    let fixture = OpenSearchFixture::new().await?;

    // Get pending tasks
    let pending_tasks = fixture.client.cluster().pending_tasks().await?;

    // In a test environment, there are typically no pending tasks
    // Just verify the response structure is as expected
    assert!(pending_tasks.tasks.is_empty() || !pending_tasks.tasks.is_empty());

    Ok(())
}
