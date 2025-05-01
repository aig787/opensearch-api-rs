//! OpenSearch Client implementation

pub mod http;
pub mod namespaces;

use base64::Engine;
use derive_builder::Builder;
use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client as ReqwestClient};
use url::Url;

/// Configuration for the OpenSearch client
#[derive(Debug, Clone, Default, Builder)]
#[builder(pattern = "mutable", build_fn(skip))]
pub struct ClientConfig {
    /// Base URL for the OpenSearch cluster (e.g., "https://localhost:9200")
    #[builder(setter(into))]
    pub base_url: String,

    /// Username for basic authentication
    #[builder(setter(into, strip_option), default)]
    pub username: Option<String>,

    /// Password for basic authentication
    #[builder(setter(into, strip_option), default)]
    pub password: Option<String>,

    /// Request timeout in seconds
    #[builder(default = "30")]
    pub timeout_secs: u64,

    /// Whether to verify SSL certificates
    #[builder(default = "true")]
    pub verify_ssl: bool,
}

/// Client for the OpenSearch API
#[derive(Debug, Clone)]
pub struct Client {
    /// HTTP client for making requests
    pub(crate) http_client: ReqwestClient,

    /// Base URL for the OpenSearch cluster
    pub(crate) base_url: Url,

    /// Client configuration
    _config: Arc<ClientConfig>,
}

impl Client {
    /// Create a builder for configuring and creating an OpenSearch client
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::default()
    }

    /// Create a new client with the given configuration
    pub fn new(config: ClientConfig) -> Result<Self, crate::error::Error> {
        let base_url =
            Url::parse(&config.base_url).map_err(|e| crate::error::Error::UrlParseError(e))?;

        let mut client_builder = ReqwestClient::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .danger_accept_invalid_certs(!config.verify_ssl);

        // Add basic authentication as a default header if provided
        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            let auth_value = format!("{}:{}", username, password);
            let encoded = base64::engine::general_purpose::STANDARD.encode(auth_value);
            let auth_header = format!("Basic {}", encoded);

            let mut headers = HeaderMap::new();
            headers.insert(
                "Authorization",
                HeaderValue::from_str(&auth_header).unwrap(),
            );
            client_builder = client_builder.default_headers(headers);
        }

        let http_client = client_builder
            .build()
            .map_err(|e| crate::error::Error::HttpClientError(e))?;

        Ok(Self {
            http_client,
            base_url,
            _config: Arc::new(config),
        })
    }
}

impl ClientConfigBuilder {
    /// Build the client with the current configuration
    pub fn build(&self) -> Result<Client, crate::error::Error> {
        // Validate required fields
        let base_url = self
            .base_url
            .clone()
            .ok_or_else(|| crate::error::Error::BuilderError("base_url must be set".to_string()))?;

        let config = ClientConfig {
            base_url,
            username: self.username.clone().unwrap_or(None),
            password: self.password.clone().unwrap_or(None),
            timeout_secs: self.timeout_secs.unwrap_or(30),
            verify_ssl: self.verify_ssl.unwrap_or(true),
        };
        Client::new(config)
    }
}
