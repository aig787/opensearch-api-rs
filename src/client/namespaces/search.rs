//! Search namespace for OpenSearch

use crate::error::Error;
use crate::types::query::*;
use crate::types::search::*;
use crate::Client;
use derive_builder::Builder;
use reqwest::Method;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

/// Builder for creating and executing search queries
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable", setter(into, strip_option))]
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
    from: Option<i64>,

    /// Maximum number of results to return
    #[builder(setter(strip_option), default)]
    size: Option<i64>,

    /// Sorting criteria for search results
    #[builder(setter(strip_option), default)]
    sort: Option<Vec<Sort>>,

    /// Fields to include in the result
    #[builder(setter(strip_option), default)]
    source: Option<SourceFilter>,

    /// Highlighting options
    #[builder(setter(strip_option), default)]
    highlight: Option<HighlightOptions>,

    /// Aggregations to perform
    #[builder(setter(strip_option), default)]
    aggregations: Option<HashMap<String, Aggregation>>,

    /// Search after for pagination
    #[builder(setter(strip_option), default)]
    search_after: Option<Vec<serde_json::Value>>,

    /// Script fields to compute
    #[builder(setter(strip_option), default)]
    script_fields: Option<HashMap<String, ScriptField>>,

    /// Fields to retrieve
    #[builder(setter(strip_option), default)]
    stored_fields: Option<Vec<String>>,

    /// Whether to explain the scoring
    #[builder(setter(strip_option), default)]
    explain: Option<bool>,

    /// Whether to include version information
    #[builder(setter(strip_option), default)]
    version: Option<bool>,

    /// Minimum score to include in results
    #[builder(setter(strip_option), default)]
    min_score: Option<f64>,

    /// Scroll parameter for cursor-based pagination
    #[builder(setter(strip_option), default)]
    scroll: Option<String>,

    /// Type marker for the document type
    #[builder(setter(skip), default = "std::marker::PhantomData")]
    _marker: std::marker::PhantomData<T>,
}

/// Builder for scroll requests
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct ScrollQuery<T>
where
    T: Default + for<'de> Deserialize<'de> + Send + Sync,
{
    /// The namespace for performing search operations
    #[builder(setter(into))]
    client: Client,

    /// The scroll ID from a previous scroll or search request
    #[builder(setter(into), default)]
    scroll_id: String,

    /// How long to keep the search context alive
    #[builder(setter(into), default)]
    scroll: String,

    /// Type marker for the document type
    #[builder(setter(skip), default = "std::marker::PhantomData")]
    _marker: std::marker::PhantomData<T>,
}

/// Builder for clearing one or more scroll contexts
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct ClearScrollQuery {
    /// The namespace for performing search operations
    #[builder(setter(into))]
    client: Client,

    /// List of scroll IDs to clear
    #[builder(default)]
    scroll_ids: Vec<String>,
}

/// Builder for multi-search queries
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct MSearchQuery<T>
where
    T: Default + for<'de> Deserialize<'de> + Send + Sync,
{
    /// The namespace for performing search operations
    #[builder(setter(into))]
    client: Client,

    /// Search requests to execute
    #[builder(default)]
    searches: Vec<MSearchItem>,

    /// Type marker for the document type
    #[builder(setter(skip), default = "std::marker::PhantomData")]
    _marker: std::marker::PhantomData<T>,
}

/// Builder for creating point-in-time search contexts
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct PointInTimeQuery {
    /// The namespace for performing search operations
    #[builder(setter(into))]
    client: Client,

    /// The index to create point-in-time for (required)
    #[builder(setter(into), default)]
    index: String,

    /// How long to keep the search context alive
    #[builder(setter(into), default)]
    keep_alive: String,
}

/// Builder for deleting point-in-time search contexts
#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct DeletePointInTimeQuery {
    /// The namespace for performing search operations
    #[builder(setter(into))]
    client: Client,

    /// The point-in-time ID to delete
    #[builder(setter(into), default)]
    pit_id: String,
}

impl<T> SearchQuery<T>
where
    T: Default + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    /// Execute the search query and return results
    pub async fn send(self) -> Result<SearchResponse<T>, Error> {
        let index_str = self.index;
        let mut path = format!("/{}/_search", index_str);

        // Add scroll parameter as query parameter if provided
        if let Some(scroll_val) = &self.scroll {
            path = format!("{}?scroll={}", path, scroll_val);
        }

        let mut body = json!({
            "query": self.query,
        });

        if let Some(from_val) = self.from {
            body["from"] = json!(from_val);
        }

        if let Some(size_val) = self.size {
            body["size"] = json!(size_val);
        }

        if let Some(sort_val) = self.sort {
            body["sort"] = json!(sort_val);
        }

        if let Some(source_val) = self.source {
            body["_source"] = json!(source_val);
        }

        if let Some(highlight_val) = self.highlight {
            body["highlight"] = json!(highlight_val);
        }

        if let Some(aggs_val) = self.aggregations {
            body["aggs"] = json!(aggs_val);
        }

        if let Some(search_after_val) = self.search_after {
            body["search_after"] = json!(search_after_val);
        }

        if let Some(script_fields_val) = self.script_fields {
            body["script_fields"] = json!(script_fields_val);
        }

        if let Some(stored_fields_val) = self.stored_fields {
            body["stored_fields"] = json!(stored_fields_val);
        }

        if let Some(explain_val) = self.explain {
            body["explain"] = json!(explain_val);
        }

        if let Some(version_val) = self.version {
            body["version"] = json!(version_val);
        }

        if let Some(min_score_val) = self.min_score {
            body["min_score"] = json!(min_score_val);
        }

        self.client
            .request::<_, SearchResponse<T>>(Method::POST, &path, Some(&body))
            .await
    }
}

impl<T> ScrollQuery<T>
where
    T: Default + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    /// Execute the scroll query and return results
    pub async fn send(self) -> Result<ScrollResponse<T>, Error> {
        let path = "/_search/scroll";

        let body = json!({
            "scroll_id": self.scroll_id,
            "scroll": self.scroll
        });

        self.client
            .request::<_, ScrollResponse<T>>(Method::POST, path, Some(&body))
            .await
    }
}

impl ClearScrollQuery {
    /// Execute the clear scroll request
    pub async fn send(self) -> Result<ClearScrollResponse, Error> {
        let path = "/_search/scroll";

        let body = json!({
            "scroll_id": self.scroll_ids
        });

        self.client
            .request::<_, ClearScrollResponse>(Method::DELETE, path, Some(&body))
            .await
    }

    /// Add a scroll ID to the list of scroll IDs to clear
    pub fn add_scroll_id(mut self, scroll_id: impl Into<String>) -> Self {
        self.scroll_ids.push(scroll_id.into());
        self
    }

    /// Add multiple scroll IDs to the list of scroll IDs to clear
    pub fn add_scroll_ids<I, S>(mut self, scroll_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for id in scroll_ids {
            self.scroll_ids.push(id.into());
        }
        self
    }
}

impl<T> MSearchQuery<T>
where
    T: Default + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    /// Add a search item to the multi-search query
    pub fn add_search(mut self, search_item: MSearchItem) -> Self {
        self.searches.push(search_item);
        self
    }

    /// Execute the multi-search query and return results
    pub async fn send(self) -> Result<MSearchResponse<T>, Error> {
        let path = "/_msearch";

        // MSearch uses a special format with newline-delimited JSON
        let mut body = String::new();
        for item in self.searches {
            // Header line (contains index and other metadata)
            body.push_str(&serde_json::to_string(&item.header)?);
            body.push('\n');

            // Body line (contains the actual query)
            body.push_str(&serde_json::to_string(&item.body)?);
            body.push('\n');
        }
        body.push('\n');

        self.client
            .request_with_headers::<String, MSearchResponse<T>>(
                Method::POST,
                path,
                Some(body),
                Some(vec![("Content-Type", "application/x-ndjson")]),
            )
            .await
    }
}

impl PointInTimeQuery {
    /// Execute the point-in-time creation request
    pub async fn send(self) -> Result<PointInTimeResponse, Error> {
        let path = format!("/{}/_pit", self.index);

        let body = json!({
            "keep_alive": self.keep_alive
        });

        self.client
            .request::<_, PointInTimeResponse>(Method::POST, &path, Some(&body))
            .await
    }
}

impl DeletePointInTimeQuery {
    /// Execute the point-in-time deletion request
    pub async fn send(self) -> Result<DeletePointInTimeResponse, Error> {
        let path = "/_pit";

        let body = json!({
            "id": self.pit_id
        });

        self.client
            .request::<_, DeletePointInTimeResponse>(Method::DELETE, path, Some(&body))
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

    /// Create a multi-search query builder
    pub fn msearch<T>(&self) -> MSearchQueryBuilder<T>
    where
        T: Default + Clone + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        let mut builder = MSearchQueryBuilder::default();
        builder.client(self.clone());
        builder
    }

    /// Create a scroll query builder
    pub fn scroll<T>(&self) -> ScrollQueryBuilder<T>
    where
        T: Default + Clone + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        let mut builder = ScrollQueryBuilder::default();
        builder.client(self.clone());
        builder
    }

    /// Create a clear scroll query builder
    pub fn clear_scroll(&self) -> ClearScrollQueryBuilder {
        let mut builder = ClearScrollQueryBuilder::default();
        builder.client(self.clone());
        builder
    }
}
