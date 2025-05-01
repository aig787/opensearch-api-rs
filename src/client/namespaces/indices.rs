//! Indices namespace for OpenSearch

use crate::error::Error;
use derive_builder::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

/// Client namespace for index-related operations
#[derive(Debug, Clone)]
pub struct IndicesNamespace {
    client: crate::client::Client,
}

impl IndicesNamespace {
    /// Create a new indices namespace with the given client
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self { client }
    }

    /// Check if an index exists
    pub fn exists(&self, index: impl Into<String>) -> ExistsIndexRequestBuilder {
        let mut builder = ExistsIndexRequestBuilder::default();
        builder.index(index.into());
        builder.client(self.client.clone());
        builder
    }
}

/// Index exists request
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct ExistsIndexRequest {
    /// The index name
    #[builder(setter(into))]
    pub index: String,

    /// Client reference
    #[builder(private)]
    client: crate::client::Client,
}

impl ExistsIndexRequest {
    /// Create a new exists index request builder
    pub fn builder() -> ExistsIndexRequestBuilder {
        ExistsIndexRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(self) -> Result<bool, Error> {
        let path = format!("/{}", self.index);

        let response = self
            .client
            .http_client
            .head(
                self.client
                    .base_url
                    .join(&path)
                    .map_err(Error::UrlParseError)?,
            )
            .send()
            .await
            .map_err(Error::HttpRequestError)?;

        Ok(response.status().is_success())
    }
}

/// Index settings builder
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(pattern = "mutable")]
#[serde(rename_all = "snake_case")]
pub struct IndexSettings {
    /// Number of shards
    #[builder(default = "1")]
    pub number_of_shards: u32,

    /// Number of replicas
    #[builder(default = "1")]
    pub number_of_replicas: u32,

    /// Refresh interval
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_interval: Option<String>,

    /// Custom analysis settings
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<Value>,
}

// Custom Deserialize implementation to handle both numeric values and string representations
impl<'de> Deserialize<'de> for IndexSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct StringOrNum<T>
        where
            T: FromStr + for<'d> Deserialize<'d>,
            T::Err: std::fmt::Display,
            <T as FromStr>::Err: std::fmt::Display,
        {
            #[serde(deserialize_with = "deserialize_string_or_number")]
            number_of_shards: T,

            #[serde(deserialize_with = "deserialize_string_or_number")]
            number_of_replicas: T,

            #[serde(skip_serializing_if = "Option::is_none")]
            refresh_interval: Option<String>,

            #[serde(skip_serializing_if = "Option::is_none")]
            analysis: Option<Value>,
        }

        let helper = StringOrNum::<u32>::deserialize(deserializer)?;

        Ok(IndexSettings {
            number_of_shards: helper.number_of_shards,
            number_of_replicas: helper.number_of_replicas,
            refresh_interval: helper.refresh_interval,
            analysis: helper.analysis,
        })
    }
}

// Helper function to deserialize a value that can be either a number or a string containing a number
fn deserialize_string_or_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + Deserialize<'de>,
    T::Err: std::fmt::Display,
    <T as FromStr>::Err: std::fmt::Display,
    D: serde::Deserializer<'de>,
{
    struct StringOrNumber<T>(std::marker::PhantomData<T>);

    impl<'de, T> serde::de::Visitor<'de> for StringOrNumber<T>
    where
        T: std::str::FromStr + Deserialize<'de>,
        T::Err: std::fmt::Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a number or a string containing a number")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            value.parse::<T>().map_err(serde::de::Error::custom)
        }

        fn visit_u64<E>(self, value: u64) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            // For numeric types that can be converted directly
            match serde_json::Number::from(value).as_u64() {
                Some(n) => {
                    let val = serde_json::Value::Number(serde_json::Number::from(n));
                    T::deserialize(val).map_err(|_| {
                        serde::de::Error::custom(format!("Failed to deserialize {}", value))
                    })
                }
                None => Err(serde::de::Error::custom(format!(
                    "Failed to convert {} to number",
                    value
                ))),
            }
        }

        fn visit_i64<E>(self, value: i64) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            // For numeric types that can be converted directly
            match serde_json::Number::from(value).as_i64() {
                Some(n) => {
                    let val = serde_json::Value::Number(serde_json::Number::from(n));
                    T::deserialize(val).map_err(|_| {
                        serde::de::Error::custom(format!("Failed to deserialize {}", value))
                    })
                }
                None => Err(serde::de::Error::custom(format!(
                    "Failed to convert {} to number",
                    value
                ))),
            }
        }
    }

    deserializer.deserialize_any(StringOrNumber(std::marker::PhantomData))
}

impl IndexSettings {
    /// Create a new index settings builder
    pub fn builder() -> IndexSettingsBuilder {
        IndexSettingsBuilder::default()
    }
}

/// Create index request builder
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(pattern = "owned")]
pub struct CreateIndexRequest {
    /// Index settings
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<IndexSettings>,

    /// Index mappings
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Value>,

    /// Index aliases
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<HashMap<String, Value>>,

    /// Client reference
    #[builder(private)]
    #[serde(skip)]
    client: Option<crate::client::Client>,

    /// Index name
    #[builder(private)]
    #[serde(skip)]
    index: Option<String>,
}

impl CreateIndexRequest {
    /// Create a new index request builder
    pub fn builder() -> CreateIndexRequestBuilder {
        CreateIndexRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(mut self) -> Result<crate::types::indices::CreateIndexResponse, Error> {
        let index_name = self.index.take().expect("Index name must be set");
        let path = format!("/{}", index_name);
        let client = self.client.take().expect("Client must be set");

        client
            .request::<CreateIndexRequest, crate::types::indices::CreateIndexResponse>(
                Method::PUT,
                &path,
                Some(&self),
            )
            .await
    }
}

impl crate::client::Client {
    /// Access the indices namespace
    pub fn indices(&self) -> IndicesNamespace {
        IndicesNamespace::new(self.clone())
    }
}

/// Delete index request
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned")]
pub struct DeleteIndexRequest {
    /// The index name
    #[builder(setter(into))]
    pub index: String,

    /// Client reference
    #[builder(private)]
    client: crate::client::Client,
}

impl DeleteIndexRequest {
    /// Create a new delete index request builder
    pub fn builder() -> DeleteIndexRequestBuilder {
        DeleteIndexRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(self) -> Result<crate::types::indices::DeleteIndexResponse, Error> {
        let path = format!("/{}", self.index);

        self.client
            .request::<(), crate::types::indices::DeleteIndexResponse>(Method::DELETE, &path, None)
            .await
    }
}

/// Close index request
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "owned")]
pub struct CloseIndexRequest {
    /// The index name
    #[builder(setter(into))]
    pub index: String,

    /// Client reference
    #[builder(private)]
    client: crate::client::Client,
}

impl CloseIndexRequest {
    /// Create a new close index request builder
    pub fn builder() -> CloseIndexRequestBuilder {
        CloseIndexRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(self) -> Result<crate::types::indices::CloseIndexResponse, Error> {
        let path = format!("/{}/_close", self.index);

        self.client
            .request::<(), crate::types::indices::CloseIndexResponse>(Method::POST, &path, None)
            .await
    }
}

/// Open index request
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct OpenIndexRequest {
    /// The index name
    #[builder(setter(into))]
    pub index: String,

    /// Client reference
    #[builder(private)]
    client: crate::client::Client,
}

impl OpenIndexRequest {
    /// Create a new open index request builder
    pub fn builder() -> OpenIndexRequestBuilder {
        OpenIndexRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(self) -> Result<crate::types::indices::OpenIndexResponse, Error> {
        let path = format!("/{}/_open", self.index);

        self.client
            .request::<(), crate::types::indices::OpenIndexResponse>(Method::POST, &path, None)
            .await
    }
}

/// Get index settings request
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct GetIndexSettingsRequest {
    /// The index name
    #[builder(setter(into))]
    pub index: String,

    /// Client reference
    #[builder(private)]
    client: crate::client::Client,
}

impl GetIndexSettingsRequest {
    /// Create a new get index settings request builder
    pub fn builder() -> GetIndexSettingsRequestBuilder {
        GetIndexSettingsRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(self) -> Result<HashMap<String, IndexSettingsResponse>, Error> {
        let path = format!("/{}/_settings", self.index);

        self.client
            .request::<(), HashMap<String, IndexSettingsResponse>>(Method::GET, &path, None)
            .await
    }
}

/// Index settings response
#[derive(Debug, Clone, Deserialize)]
pub struct IndexSettingsResponse {
    /// Index settings
    pub settings: IndexSettingsDetails,
}

/// Index settings details
#[derive(Debug, Clone, Deserialize)]
pub struct IndexSettingsDetails {
    /// Index settings
    pub index: IndexSettings,
}

/// Update index settings request
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(pattern = "mutable")]
pub struct UpdateIndexSettingsRequest {
    /// The index settings to update
    #[serde(rename = "index")]
    pub settings: HashMap<String, Value>,

    /// Client reference
    #[builder(private)]
    #[serde(skip)]
    client: Option<crate::client::Client>,

    /// Index name
    #[builder(private)]
    #[serde(skip)]
    index: Option<String>,
}

impl UpdateIndexSettingsRequest {
    /// Create a new update index settings request builder
    pub fn builder() -> UpdateIndexSettingsRequestBuilder {
        UpdateIndexSettingsRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(
        mut self,
    ) -> Result<crate::types::indices::UpdateIndexSettingsResponse, Error> {
        let index_name = self.index.take().expect("Index name must be set");
        let path = format!("/{}/_settings", index_name);
        let client = self.client.take().expect("Client must be set");

        client
            .request::<UpdateIndexSettingsRequest, crate::types::indices::UpdateIndexSettingsResponse>(Method::PUT, &path, Some(&self))
            .await
    }
}

/// Get mapping request
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct GetMappingRequest {
    /// The index name
    #[builder(setter(into))]
    pub index: String,

    /// Client reference
    #[builder(private)]
    client: crate::client::Client,
}

impl GetMappingRequest {
    /// Create a new get mapping request builder
    pub fn builder() -> GetMappingRequestBuilder {
        GetMappingRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(self) -> Result<HashMap<String, MappingResponse>, Error> {
        let path = format!("/{}/_mapping", self.index);

        self.client
            .request::<(), HashMap<String, MappingResponse>>(Method::GET, &path, None)
            .await
    }
}

/// Mapping response
#[derive(Debug, Clone, Deserialize)]
pub struct MappingResponse {
    /// Mappings
    pub mappings: Value,
}

/// Put mapping request
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(pattern = "mutable")]
pub struct PutMappingRequest {
    /// The mapping definition
    pub properties: HashMap<String, Value>,

    /// Client reference
    #[builder(private)]
    #[serde(skip)]
    client: Option<crate::client::Client>,

    /// Index name
    #[builder(private)]
    #[serde(skip)]
    index: Option<String>,
}

impl PutMappingRequest {
    /// Create a new put mapping request builder
    pub fn builder() -> PutMappingRequestBuilder {
        PutMappingRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(mut self) -> Result<crate::types::indices::PutMappingResponse, Error> {
        let index_name = self.index.take().expect("Index name must be set");
        let path = format!("/{}/_mapping", index_name);
        let client = self.client.take().expect("Client must be set");

        client
            .request::<PutMappingRequest, crate::types::indices::PutMappingResponse>(
                Method::PUT,
                &path,
                Some(&self),
            )
            .await
    }
}

/// Get aliases request
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct GetAliasesRequest {
    /// The index name
    #[builder(setter(into))]
    pub index: String,

    /// Client reference
    #[builder(private)]
    client: crate::client::Client,
}

impl GetAliasesRequest {
    /// Create a new get aliases request builder
    pub fn builder() -> GetAliasesRequestBuilder {
        GetAliasesRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(self) -> Result<HashMap<String, AliasesResponse>, Error> {
        let path = format!("/{}/_alias", self.index);

        self.client
            .request::<(), HashMap<String, AliasesResponse>>(Method::GET, &path, None)
            .await
    }
}

/// Aliases response
#[derive(Debug, Clone, Deserialize)]
pub struct AliasesResponse {
    /// Aliases
    pub aliases: HashMap<String, Value>,
}

/// Update aliases request
#[derive(Debug, Clone, Builder, Serialize)]
#[builder(pattern = "mutable")]
pub struct UpdateAliasesRequest {
    /// Actions to perform on aliases
    pub actions: Vec<AliasAction>,

    /// Client reference
    #[builder(private)]
    #[serde(skip)]
    client: Option<crate::client::Client>,
}

/// Add alias action properties
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(pattern = "mutable", setter(strip_option, into))]
pub struct AddAliasAction {
    /// Index
    pub index: String,
    /// Alias
    pub alias: String,
    /// Filter
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Value>,
    /// Routing
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,
    /// Is write index
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_write_index: Option<bool>,
}

impl AddAliasAction {
    pub fn builder() -> AddAliasActionBuilder {
        AddAliasActionBuilder::default()
    }
}

/// Remove alias action properties
#[derive(Debug, Clone, Serialize)]
pub struct RemoveAliasAction {
    /// Index
    pub index: String,
    /// Alias
    pub alias: String,
}

/// Alias action
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AliasAction {
    /// Add an alias
    Add {
        /// Add action
        add: AddAliasAction,
    },
    /// Remove an alias
    Remove {
        /// Remove action
        remove: RemoveAliasAction,
    },
}

impl UpdateAliasesRequest {
    /// Create a new update aliases request builder
    pub fn builder() -> UpdateAliasesRequestBuilder {
        UpdateAliasesRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(mut self) -> Result<crate::types::indices::UpdateAliasesResponse, Error> {
        let path = "/_aliases";
        let client = self.client.take().expect("Client must be set");

        client
            .request::<UpdateAliasesRequest, crate::types::indices::UpdateAliasesResponse>(
                Method::POST,
                path,
                Some(&self),
            )
            .await
    }
}

/// Refresh index request
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct RefreshIndexRequest {
    /// The index name
    #[builder(setter(into))]
    pub index: String,

    /// Client reference
    #[builder(private)]
    client: crate::client::Client,
}

impl RefreshIndexRequest {
    /// Create a new refresh index request builder
    pub fn builder() -> RefreshIndexRequestBuilder {
        RefreshIndexRequestBuilder::default()
    }

    /// Send the request to the server
    pub async fn send(self) -> Result<crate::types::indices::RefreshIndexResponse, Error> {
        let path = format!("/{}/_refresh", self.index);

        self.client
            .request::<(), crate::types::indices::RefreshIndexResponse>(Method::POST, &path, None)
            .await
    }
}

impl IndicesNamespace {
    /// Create an index with the given settings
    pub fn create(&self, index: impl Into<String>) -> CreateIndexRequestBuilder {
        CreateIndexRequestBuilder::default()
            .client(Some(self.client.clone()))
            .index(Some(index.into()))
    }

    /// Delete an index
    pub fn delete(&self, index: impl Into<String>) -> DeleteIndexRequestBuilder {
        DeleteIndexRequestBuilder::default()
            .index(index.into())
            .client(self.client.clone())
    }

    /// Close an index
    pub fn close(&self, index: impl Into<String>) -> CloseIndexRequestBuilder {
        CloseIndexRequestBuilder::default()
            .index(index.into())
            .client(self.client.clone())
    }

    /// Open an index
    pub fn open(&self, index: impl Into<String>) -> OpenIndexRequestBuilder {
        let mut builder = OpenIndexRequestBuilder::default();
        builder.index(index.into());
        builder.client(self.client.clone());
        builder
    }

    /// Get index settings
    pub fn get_settings(&self, index: impl Into<String>) -> GetIndexSettingsRequestBuilder {
        let mut builder = GetIndexSettingsRequestBuilder::default();
        builder.index(index.into());
        builder.client(self.client.clone());
        builder
    }

    /// Update index settings
    pub fn update_settings(&self, index: impl Into<String>) -> UpdateIndexSettingsRequestBuilder {
        let mut builder = UpdateIndexSettingsRequestBuilder::default();
        builder.client(Some(self.client.clone()));
        builder.index(Some(index.into()));
        builder
    }

    /// Get mappings
    pub fn get_mapping(&self, index: impl Into<String>) -> GetMappingRequestBuilder {
        let mut builder = GetMappingRequestBuilder::default();
        builder.index(index.into());
        builder.client(self.client.clone());
        builder
    }

    /// Put mappings
    pub fn put_mapping(&self, index: impl Into<String>) -> PutMappingRequestBuilder {
        let mut builder = PutMappingRequestBuilder::default();
        builder.client(Some(self.client.clone()));
        builder.index(Some(index.into()));
        builder
    }

    /// Get aliases
    pub fn get_aliases(&self, index: impl Into<String>) -> GetAliasesRequestBuilder {
        let mut builder = GetAliasesRequestBuilder::default();
        builder.index(index.into());
        builder.client(self.client.clone());
        builder
    }

    /// Update aliases
    pub fn update_aliases(&self) -> UpdateAliasesRequestBuilder {
        let mut builder = UpdateAliasesRequestBuilder::default();
        builder.client(Some(self.client.clone()));
        builder
    }

    /// Refresh an index
    pub fn refresh(&self, index: impl Into<String>) -> RefreshIndexRequestBuilder {
        let mut builder = RefreshIndexRequestBuilder::default();
        builder.index(index.into());
        builder.client(self.client.clone());
        builder
    }
}
