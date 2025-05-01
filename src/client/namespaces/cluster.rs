//! Cluster namespace for OpenSearch

use crate::error::Error;
use reqwest::Method;
use serde_json::Value;

/// Client namespace for cluster-related operations
#[derive(Debug, Clone)]
pub struct ClusterNamespace {
    client: crate::client::Client,
}

impl ClusterNamespace {
    /// Create a new cluster namespace with the given client
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self { client }
    }

    /// Get cluster health
    pub async fn health(&self) -> Result<Value, Error> {
        self.client.request::<(), Value>(Method::GET, "/_cluster/health", None).await
    }

    /// Get cluster stats
    pub async fn stats(&self) -> Result<Value, Error> {
        self.client.request::<(), Value>(Method::GET, "/_cluster/stats", None).await
    }

    /// Get cluster state
    pub async fn state(&self) -> Result<Value, Error> {
        self.client.request::<(), Value>(Method::GET, "/_cluster/state", None).await
    }
}

impl crate::client::Client {
    /// Access the cluster namespace
    pub fn cluster(&self) -> ClusterNamespace {
        ClusterNamespace::new(self.clone())
    }
}
