//! Search namespace for OpenSearch

use crate::error::Error;
use crate::types::search::SearchResponse;
use crate::{Client, Query};
use derive_builder::Builder;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Builder for creating and executing search queries
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct SearchQuery<T>
where
    T: Default + for<'de> Deserialize<'de> + Send + Sync,
{
    /// The namespace for performing search operations
    #[builder(setter(into))]
    client: Client,

    /// The index to search (required)
    #[builder(setter(into), default)]
    index: String,

    /// The search query (required)
    #[builder(default)]
    query: Query,

    /// The starting offset for search results
    #[builder(setter(strip_option), default)]
    from: Option<u64>,

    /// Maximum number of results to return
    #[builder(setter(strip_option), default)]
    size: Option<u64>,

    /// Type marker for the document type
    #[builder(setter(skip), default = "std::marker::PhantomData")]
    _marker: std::marker::PhantomData<T>,
}

impl<T> SearchQuery<T>
where
    T: Default + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    /// Execute the search query and return results
    pub async fn send(self) -> Result<SearchResponse<T>, Error> {
        let index_str = self.index;
        let path = format!("/{}/_search", index_str);

        let mut body = json!({
            "query": self.query,
        });

        if let Some(from_val) = self.from {
            body["from"] = json!(from_val);
        }

        if let Some(size_val) = self.size {
            body["size"] = json!(size_val);
        }

        self.client
            .request::<_, SearchResponse<T>>(Method::POST, &path, Some(&body))
            .await
    }
}

impl Client {
    /// Create a search query builder
    pub fn search<T>(&self) -> SearchQueryBuilder<T>
    where
        T: Default + Clone + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        let mut builder = SearchQueryBuilder::default();
        builder.client(self.clone());
        builder
    }

    /// Execute a raw search request
    pub async fn execute<T>(
        &self,
        index: impl Into<String>,
        body: impl Serialize,
    ) -> Result<SearchResponse<T>, Error>
    where
        T: Default + for<'de> Deserialize<'de> + Send + Sync,
    {
        let index_str = index.into();
        let path = format!("/{}/_search", index_str);

        self.request::<_, SearchResponse<T>>(Method::POST, &path, Some(&body))
            .await
    }
}
