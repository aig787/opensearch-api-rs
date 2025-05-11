//! HTTP client utilities for OpenSearch

use crate::error::Error;
use reqwest::{Body, Method};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::any::type_name;

/// Represents the response from the OpenSearch root endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSearchInfo {
    /// Name of the cluster or node
    pub name: String,
    /// Cluster UUID
    pub cluster_name: Option<String>,
    /// Cluster UUID
    pub cluster_uuid: Option<String>,
    /// Version information
    pub version: OpenSearchVersion,
    /// Build information (might be present in some versions)
    pub build: Option<OpenSearchBuild>,
    /// Tagline (typically "The OpenSearch Project: https://opensearch.org/")
    pub tagline: Option<String>,
}

/// Version information for OpenSearch
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSearchVersion {
    /// Version number (e.g., "2.4.0")
    pub number: String,
    /// Build type (e.g., "tar")
    pub build_type: Option<String>,
    /// Build hash
    pub build_hash: Option<String>,
    /// Build date
    pub build_date: Option<String>,
    /// Build snapshot flag
    pub build_snapshot: Option<bool>,
    /// Lucene version
    pub lucene_version: Option<String>,
    /// Minimum wire compatibility version
    pub minimum_wire_compatibility_version: Option<String>,
    /// Minimum index compatibility version
    pub minimum_index_compatibility_version: Option<String>,
}

/// Build information for OpenSearch
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenSearchBuild {
    /// Build type
    pub type_field: Option<String>,
    /// Build hash
    pub hash: Option<String>,
    /// Build date
    pub date: Option<String>,
    /// Build snapshot flag
    pub snapshot: Option<bool>,
}

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

        // Handle error responses, but treat 404 as valid for certain operations
        if !status.is_success() && status != reqwest::StatusCode::NOT_FOUND {
            let error_text = response.text().await.unwrap_or_default();

            return Err(Error::ApiError {
                status_code: status.as_u16(),
                message: error_text,
                request_body_info: serde_json::to_string(&body).ok().unwrap_or_default(),
            });
        }

        // Get response text and attempt to deserialize
        let response_text = response.text().await.map_err(Error::HttpRequestError)?;

        // Try to parse the response with enhanced error information
        let deserializer = &mut serde_json::Deserializer::from_str(&response_text);
        match serde_path_to_error::deserialize(deserializer) {
            Ok(parsed) => Ok(parsed),
            Err(path_err) => {
                // Extract path information
                let path = path_err.path().to_string();
                let err = path_err.into_inner();
                let expected_type = type_name::<R>();

                // Log the error for debugging
                log::debug!(
                    "Deserialization error at path '{}': {}. Response: {}",
                    path,
                    err,
                    response_text
                );

                // Try a more relaxed approach with serde_json::Value first to help debugging
                match serde_json::from_str::<serde_json::Value>(&response_text) {
                    Ok(_) => {
                        // If we can parse as JSON but not as the target type, provide a better error
                        Err(Error::deserialization_with_response(
                            err,
                            response_text,
                            path,
                            expected_type,
                        ))
                    }
                    Err(_) => {
                        // If we can't even parse as JSON, return the original error with path information
                        Err(Error::deserialization_with_response(
                            err,
                            response_text,
                            path,
                            expected_type,
                        ))
                    }
                }
            }
        }
    }

    /// Make a generic HTTP request to the OpenSearch API with custom headers
    pub async fn request_with_headers<B, R>(
        &self,
        method: Method,
        path: &str,
        body: Option<B>,
        headers: Option<Vec<(&str, &str)>>,
    ) -> Result<R, Error>
    where
        B: Serialize + ?Sized + Into<Body>,
        R: DeserializeOwned,
    {
        let url = self.base_url.join(path).map_err(Error::UrlParseError)?;
        let mut request_builder = self.http_client.request(method, url);

        // Add custom headers if provided
        if let Some(custom_headers) = headers {
            for (name, value) in custom_headers {
                request_builder = request_builder.header(name, value);
            }
        }

        // Add body if provided
        let body_string = if let Some(body) = body {
            // Don't automatically add Content-Type header here since it might be specified in custom headers
            let body_string = serde_json::to_string(&body).ok();
            request_builder = request_builder.body(body.into());
            body_string
        } else {
            None
        };

        // Send request
        let response = request_builder
            .send()
            .await
            .map_err(Error::HttpRequestError)?;
        let status = response.status();

        // Handle error responses, but treat 404 as valid for certain operations
        if !status.is_success() && status != reqwest::StatusCode::NOT_FOUND {
            let error_text = response.text().await.unwrap_or_default();
            // Format the request body for inclusion in the error

            return Err(Error::ApiError {
                status_code: status.as_u16(),
                message: error_text,
                request_body_info: body_string.unwrap_or_default(),
            });
        }

        // Get response text and attempt to deserialize
        let response_text = response.text().await.map_err(Error::HttpRequestError)?;

        // Try to parse the response with enhanced error information
        let deserializer = &mut serde_json::Deserializer::from_str(&response_text);
        match serde_path_to_error::deserialize(deserializer) {
            Ok(parsed) => Ok(parsed),
            Err(path_err) => {
                // Extract path information
                let path = path_err.path().to_string();
                let err = path_err.into_inner();
                let expected_type = type_name::<R>();

                // Log the error for debugging
                log::debug!(
                    "Deserialization error at path '{}': {}. Response: {}",
                    path,
                    err,
                    response_text
                );

                // Try a more relaxed approach with serde_json::Value first to help debugging
                match serde_json::from_str::<serde_json::Value>(&response_text) {
                    Ok(_) => {
                        // If we can parse as JSON but not as the target type, provide a better error
                        Err(Error::deserialization_with_response(
                            err,
                            response_text,
                            path,
                            expected_type,
                        ))
                    }
                    Err(_) => {
                        // If we can't even parse as JSON, return the original error with path information
                        Err(Error::deserialization_with_response(
                            err,
                            response_text,
                            path,
                            expected_type,
                        ))
                    }
                }
            }
        }
    }

    /// Make a HEAD request to check if a resource exists
    pub async fn exists(&self, path: &str) -> Result<bool, Error> {
        let url = self.base_url.join(path).map_err(Error::UrlParseError)?;
        log::debug!("Making HEAD request to check existence: {}", url);

        let result = self.http_client.head(url).send().await;

        match result {
            Ok(response) => {
                let status = response.status();
                log::debug!("HEAD request returned status: {}", status);
                Ok(status.is_success())
            }
            Err(err) => {
                // HTTP 404 indicates resource doesn't exist, not an error
                if let Some(status) = err.status() {
                    if status == reqwest::StatusCode::NOT_FOUND {
                        log::debug!("Resource not found (404), returning false");
                        return Ok(false);
                    }
                    log::warn!("HEAD request failed with status: {}", status);
                } else {
                    log::error!("HEAD request failed: {}", err);
                }
                Err(Error::HttpRequestError(err))
            }
        }
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
    pub async fn info(&self) -> Result<OpenSearchInfo, Error> {
        self.request::<(), _>(Method::GET, "/", None).await
    }

    /// Helper to get the version of OpenSearch
    pub async fn version(&self) -> Result<String, Error> {
        let info = self.info().await?;
        Ok(info.version.number)
    }
}
