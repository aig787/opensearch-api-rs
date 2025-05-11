use opensearch_api::{Client, ClientConfig};
use reqwest::StatusCode;
use std::borrow::Cow;
use std::collections::HashMap;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::core::{ContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, Image, ImageExt, ReuseDirective};
use uuid::Uuid;

/// Default OpenSearch port for tests
pub const OPENSEARCH_PORT: u16 = 9200;
pub const DEFAULT_USERNAME: &str = "admin";
pub const DEFAULT_PASSWORD: &str = "*!ST8IbKo5uFRs";

/// OpenSearch Docker image configuration
#[derive(Debug, Clone)]
pub struct OpenSearchImage {
    tag: String,
    password: String,
    exposed_ports: Vec<ContainerPort>,
    env_vars: HashMap<String, String>,
}

impl Default for OpenSearchImage {
    fn default() -> Self {
        Self {
            tag: "2.19.0".to_owned(),
            password: DEFAULT_PASSWORD.to_owned(),
            exposed_ports: vec![OPENSEARCH_PORT.into()],
            env_vars: vec![
                ("OPENSEARCH_INITIAL_ADMIN_PASSWORD", DEFAULT_PASSWORD),
                ("discovery.type", "single-node"),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        }
    }
}

impl Image for OpenSearchImage {
    fn name(&self) -> &str {
        "opensearchproject/opensearch"
    }

    fn tag(&self) -> &str {
        &self.tag
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::Http(
            HttpWaitStrategy::new("/")
                .with_basic_auth(DEFAULT_USERNAME, self.password())
                .with_port(OPENSEARCH_PORT.into())
                .with_tls()
                .with_client(
                    reqwest::Client::builder()
                        .danger_accept_invalid_certs(true)
                        .build()
                        .unwrap(),
                )
                .with_expected_status_code(StatusCode::OK),
        )]
    }

    fn env_vars(
        &self,
    ) -> impl IntoIterator<Item = (impl Into<Cow<'_, str>>, impl Into<Cow<'_, str>>)> {
        Box::new(self.env_vars.iter())
    }

    fn expose_ports(&self) -> &[ContainerPort] {
        &self.exposed_ports
    }
}

impl OpenSearchImage {
    #[allow(dead_code)]
    pub fn with_password(mut self, password: &str) -> Self {
        self.password = password.to_owned();
        self.env_vars.insert(
            "OPENSEARCH_INITIAL_ADMIN_PASSWORD".to_string(),
            password.to_owned(),
        );
        self
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

/// Test fixture for running OpenSearch in a Docker container
#[derive(Debug)]
pub struct OpenSearchFixture {
    /// Docker container running OpenSearch
    _container: ContainerAsync<OpenSearchImage>,
    // The id of this test instance
    pub id: String,
    /// The OpenSearch client
    pub client: Client,
}

impl OpenSearchFixture {
    /// Create a new OpenSearch test fixture
    pub async fn new() -> Result<Self, anyhow::Error> {
        let id = Uuid::new_v4().to_string();
        // Configure the OpenSearch container
        let container = OpenSearchImage::default()
            .with_reuse(ReuseDirective::Always)
            .start()
            .await?;

        // Create a client connected to the container
        let host_port = container.get_host_port_ipv4(OPENSEARCH_PORT).await?;
        let base_url = format!("https://localhost:{}", host_port);

        // Create and configure the client
        let client = Client::builder()
            .config(
                ClientConfig::builder()
                    .base_url(base_url)
                    .username(DEFAULT_USERNAME)
                    .password(DEFAULT_PASSWORD)
                    .verify_ssl(false)
                    .build()?,
            )
            .build()?;

        // Create the fixture
        let fixture = Self {
            _container: container,
            id,
            client,
        };

        Ok(fixture)
    }

    /// Get the base URL for connecting to the OpenSearch container
    pub async fn get_base_url(&self) -> anyhow::Result<String> {
        Ok(format!(
            "https://localhost:{}",
            self._container.get_host_port_ipv4(OPENSEARCH_PORT).await?
        ))
    }

    /// Creates a namespaced index name to ensure test isolation
    pub fn namespaced_index(&self, index_name: &str) -> String {
        format!("{}-{}", index_name, self.id)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[tokio::test]
    // async fn test_opensearch_fixture() -> anyhow::Result<()> {
    //     let fixture = OpenSearchFixture::new().await?;
    //
    //     // Verify that we can ping the cluster
    //     let ping_result = fixture.client.ping().await.expect("Failed to ping cluster");
    //     assert!(ping_result, "OpenSearch cluster should be pingable");
    //
    //     Ok(())
    // }
}
