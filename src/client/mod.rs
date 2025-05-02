//! OpenSearch Client implementation

pub mod http;
pub mod namespaces;

use base64::Engine;
use derive_builder::Builder;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client as ReqwestClient, Method};
use serde::de::DeserializeOwned;
use url::Url;
use crate::Error;

/// Configuration for the OpenSearch client
#[derive(Debug, Clone, Default, Builder)]
#[builder(pattern = "mutable", build_fn(error = "crate::Error"))]
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

impl ClientConfig {
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::default()
    }
}

/// Client for the OpenSearch API
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(skip))]
pub struct Client {
    /// HTTP client for making requests
    #[builder(setter(skip))]
    pub(crate) http_client: ReqwestClient,

    /// Base URL for the OpenSearch cluster
    #[builder(setter(skip))]
    pub(crate) base_url: Url,

    /// Client configuration
    #[allow(dead_code)]
    config: ClientConfig,
}

impl ClientBuilder {
    pub fn build(self) -> Result<Client, Error> {
        Client::new(self.config.unwrap())
    }
}

impl Client {
    /// Create a builder for configuring and creating an OpenSearch client
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Create a new client with the given configuration
    pub fn new(config: ClientConfig) -> Result<Self, Error> {
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
            config
        })
    }

    /// Send a request with a string body to OpenSearch
    ///
    /// This method is particularly useful for bulk operations or other cases
    /// where the body is already a formatted string rather than a serializable object.
    ///
    /// # Arguments
    ///
    /// * `method` - HTTP method to use
    /// * `path` - API endpoint path
    /// * `body` - Optional string body to send
    ///
    /// # Returns
    ///
    /// The deserialized response
    pub async fn request_with_string_body<R>(
        &self,
        method: Method,
        path: &str,
        body: Option<String>,
    ) -> Result<R, crate::error::Error>
    where
        R: DeserializeOwned,
    {
        let url = self
            .base_url
            .join(path)
            .map_err(crate::error::Error::UrlParseError)?;

        log::debug!("Sending {} request to {}", method, url);
        if let Some(body_ref) = &body {
            log::trace!("Request body: {}", body_ref);
        }

        let mut request_builder = self.http_client.request(method, url.clone());

        if let Some(body_str) = body.clone() {
            request_builder = request_builder.header("Content-Type", "application/json");
            request_builder = request_builder.body(body_str);
        }

        let response = request_builder
            .send()
            .await
            .map_err(crate::error::Error::HttpRequestError)?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(crate::error::Error::HttpRequestError)?;

        if !status.is_success() {
            let request_body_info =
                body.map_or(String::new(), |b| format!("\nRequest body: {}", b));
            return Err(crate::error::Error::ApiError {
                status_code: status.as_u16(),
                message: response_text,
                request_body_info,
            });
        }

        match serde_json::from_str::<R>(&response_text) {
            Ok(result) => Ok(result),
            Err(err) => {
                log::error!("Failed to deserialize response: {}", err);
                Err(crate::error::Error::DeserializationErrorWithResponse {
                    error: err,
                    response_text,
                    path: path.to_string(),
                    expected_type: std::any::type_name::<R>().to_string(),
                })
            }
        }
    }
}
