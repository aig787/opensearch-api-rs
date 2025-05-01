//! HTTP client utilities for OpenSearch

use crate::error::Error;
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

impl super::Client {
    /// Make a generic HTTP request to the OpenSearch API
    pub async fn request<B, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<R, Error>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = self.base_url.join(path).map_err(Error::UrlParseError)?;
        let mut request_builder = self.http_client.request(method, url);

        // Add body if provided
        if let Some(body) = body {
            request_builder = request_builder
                .header("Content-Type", "application/json")
                .json(body);
        }

        // Send request
        let response = request_builder
            .send()
            .await
            .map_err(Error::HttpRequestError)?;
        let status = response.status();

        // Handle error responses
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::ApiError {
                status_code: status.as_u16(),
                message: error_text,
            });
        }

        // Parse successful response
        response
            .json::<R>()
            .await
            .map_err(Error::DeserializationError)
    }

    /// Helper to check if cluster is available
    pub async fn ping(&self) -> Result<bool, Error> {
        let response = self
            .http_client
            .get(self.base_url.clone())
            .send()
            .await
            .map_err(Error::HttpRequestError)?;

        Ok(response.status().is_success())
    }

    /// Helper to get cluster info
    pub async fn info(&self) -> Result<serde_json::Value, Error> {
        self.request::<(), _>(Method::GET, "/", None).await
    }

    /// Helper to get the version of OpenSearch
    pub async fn version(&self) -> Result<String, Error> {
        let info = self.info().await?;
        info["version"]["number"]
            .as_str()
            .map(|v| v.to_string())
            .ok_or_else(|| Error::ApiError {
                status_code: 200,
                message: "Version information missing from response".to_string(),
            })
    }
}
