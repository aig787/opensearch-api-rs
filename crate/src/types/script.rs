//! Script-related data types for OpenSearch

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Script definition in OpenSearch
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Script {
    /// Inline script definition
    Inline(InlineScript),
    /// Stored script reference
    Stored(StoredScript),
}

/// Inline script definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineScript {
    /// Script source code
    pub source: String,

    /// Script language (painless, expression, mustache, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    /// Script parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,

    /// Script options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

/// Stored script reference
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StoredScript {
    /// ID of the stored script
    pub id: String,

    /// Script parameters
    pub params: Option<HashMap<String, serde_json::Value>>,
}
