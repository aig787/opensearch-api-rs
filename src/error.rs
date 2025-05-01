//! Error types for OpenSearch API operations

use thiserror::Error;

/// Error types that can occur when working with the OpenSearch API
#[derive(Error, Debug)]
pub enum Error {
    /// Error during serialization of data
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Error parsing YAML
    #[error("YAML parsing error: {0}")]
    YamlParsing(#[from] serde_yaml::Error),

    /// Error in URL construction or parsing
    #[error("URL error: {0}")]
    UrlParseError(#[from] url::ParseError),

    /// HTTP client creation error
    #[cfg(feature = "client")]
    #[error("Failed to create HTTP client: {0}")]
    HttpClientError(#[from] reqwest::Error),

    /// HTTP request error
    #[cfg(feature = "client")]
    #[error("HTTP request failed: {0}")]
    HttpRequestError(reqwest::Error),

    /// JSON deserialization error
    #[cfg(feature = "client")]
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(reqwest::Error),

    /// JSON deserialization error with response text
    #[cfg(feature = "client")]
    #[error("Failed to deserialize response at path '{path}': {error}\nExpected type: {expected_type}\nRaw response text: {response_text}")]
    DeserializationErrorWithResponse {
        /// The original deserialization error
        error: serde_json::Error,
        /// The raw response text that caused the error
        response_text: String,
        /// The path where the error occurred
        path: String,
        /// The expected type at that path
        expected_type: String,
    },

    /// API returned an error response
    #[error("API error (status {status_code}): {message}{request_body_info}")]
    ApiError {
        /// HTTP status code
        status_code: u16,
        /// Error message from the API
        message: String,
        /// Request body that caused the error (if available)
        request_body_info: String,
    },

    /// Index not found
    #[error("Index '{0}' not found")]
    IndexNotFound(String),

    /// Document not found
    #[error("Document with id '{0}' not found in index '{1}'")]
    DocumentNotFound(String, String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Builder error
    #[error("Builder error: {0}")]
    BuilderError(String),
    
    /// Error parsing HTTP header
    #[error("Failed to parse HTTP header: {0}")]
    HeaderParseError(String),

    /// Validation error for API parameters
    #[error("Validation error: {0}")]
    Validation(String),

    /// Missing required parameter
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    /// Query DSL error
    #[error("Query DSL error: {0}")]
    QueryDSL(String),

    /// Search error 
    #[error("Search error: {0}")]
    Search(String),
}

/// Result type for OpenSearch API operations
pub type Result<T> = std::result::Result<T, Error>;

/// Utility functions for working with errors
impl Error {
    /// Create a new validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Error::Validation(message.into())
    }

    /// Create a new missing parameter error
    pub fn missing_parameter(parameter: impl Into<String>) -> Self {
        Error::MissingParameter(parameter.into())
    }

    /// Create a new query DSL error
    pub fn query_dsl(message: impl Into<String>) -> Self {
        Error::QueryDSL(message.into())
    }

    /// Create a new search error
    pub fn search(message: impl Into<String>) -> Self {
        Error::Search(message.into())
    }

    /// Create a new deserialization error with the raw response text and path information
    #[cfg(feature = "client")]
    pub fn deserialization_with_response(
        error: serde_json::Error, 
        response_text: String,
        path: impl Into<String>,
        expected_type: impl Into<String>,
    ) -> Self {
        Error::DeserializationErrorWithResponse {
            error,
            response_text,
            path: path.into(),
            expected_type: expected_type.into(),
        }
    }
}
