use language_tags::LanguageTag;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_with::StringWithSeparator;
use serde_with::formats::CommaSeparator;
use std::sync::LazyLock;

use crate::SearchResponse;
use crate::response::SearchResult;
#[cfg(test)]
use crate::test::SmartJsonExt;

static GLOBAL_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .tcp_keepalive(std::time::Duration::from_secs(3600))
        .pool_max_idle_per_host(100)
        .build()
        .unwrap()
});

/// Supported response formats for the SearXNG API.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    /// Standard JSON response format.
    Json,
}

/// The main entry point for the SearXNG API.
///
/// This client manages the base configuration for interacting with a SearXNG instance.
/// It is recommended to reuse the client instance to benefit from connection pooling.
#[derive(Debug, Clone)]
pub struct SearXNGClient {
    base_url: String,
    format: ResponseFormat,
}

impl SearXNGClient {
    /// Creates a new `SearXNGClient` instance.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the SearXNG instance (e.g., "<https://searx.be>").
    /// * `format` - The desired [`ResponseFormat`].
    ///
    /// # Examples
    ///
    /// ```
    /// use searxng_client::{SearXNGClient, ResponseFormat};
    /// let client = SearXNGClient::new("https://searx.be", ResponseFormat::Json);
    /// ```
    pub fn new(base_url: impl Into<String>, format: ResponseFormat) -> Self {
        SearXNGClient {
            base_url: format!("{}/search", base_url.into().trim_end_matches('/')),
            format,
        }
    }

    /// Starts a new search query.
    ///
    /// Returns a [`SearchBuilder`] to configure and execute the search.
    ///
    /// # Arguments
    ///
    /// * `query` - The search terms.
    pub fn search<'a>(&'a self, query: impl Into<String>) -> SearchBuilder<'a> {
        SearchBuilder::new(self, query)
    }
}

/// Parameters for a SearXNG search request.
///
/// Reference: [SearXNG Search API Documentation](https://docs.searxng.org/dev/search_api.html)
#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParams {
    /// The search query string.
    pub q: String,
    pub format: ResponseFormat,
    pub pageno: Option<u32>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, String>>")]
    pub categories: Option<Vec<String>>,
    #[serde_as(as = "Option<StringWithSeparator<CommaSeparator, String> >")]
    pub engines: Option<Vec<String>>,
    pub language: Option<LanguageTag>,
    pub results_on_new_tab: Option<u32>,
    pub image_proxy: Option<bool>,
    pub autocomplete: Option<String>,
    pub safesearch: Option<u32>,
    pub theme: Option<String>,
}

impl SearchParams {
    pub fn new(query: impl Into<String>, format: ResponseFormat) -> Self {
        SearchParams {
            q: query.into(),
            format,
            pageno: None,
            categories: None,
            engines: None,
            language: None,
            results_on_new_tab: None,
            image_proxy: None,
            autocomplete: None,
            safesearch: None,
            theme: None,
        }
    }
}

/// A builder for configuring and executing a SearXNG search request.
#[derive(Debug, Clone)]
pub struct SearchBuilder<'a> {
    client: &'a SearXNGClient,
    params: SearchParams,
}

impl<'a> SearchBuilder<'a> {
    /// Creates a new `SearchBuilder` with default parameters.
    pub fn new(client: &'a SearXNGClient, query: impl Into<String>) -> Self {
        SearchBuilder {
            client,
            params: SearchParams::new(query, client.format),
        }
    }

    pub fn set_params(mut self, params: SearchParams) -> Self {
        self.params = params;
        self
    }

    pub fn set_pageno(mut self, pageno: u32) -> Self {
        self.params.pageno = Some(pageno);
        self
    }

    /// Executes the search request and returns the full [`SearchResponse`].
    ///
    /// # Errors
    ///
    /// Returns a [`reqwest::Error`] if:
    /// - The network request fails.
    /// - The server returns a status code that is not 2xx.
    /// - The response body cannot be parsed as a [`SearchResponse`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use searxng_client::{SearXNGClient, ResponseFormat};
    /// # tokio_test::block_on(async {
    /// # let client = SearXNGClient::new("https://searx.be", ResponseFormat::Json);
    /// let response = client.search("rust").send().await?;
    /// # Ok::<(), reqwest::Error>(())
    /// # });
    /// ```
    pub async fn send(&self) -> Result<SearchResponse, reqwest::Error> {
        let resp = GLOBAL_CLIENT
            .post(&self.client.base_url)
            .form(&self.params)
            .header("User-Agent", "searxng-rust-client/0.1")
            .send()
            .await?;

        #[cfg(not(test))]
        let resp = resp.json::<SearchResponse>().await?;
        #[cfg(test)]
        let resp = resp.json_test().await.unwrap();
        Ok(resp)
    }

    async fn send_empty_check_retry(&self) -> Result<Option<Vec<SearchResult>>, reqwest::Error> {
        for _ in 0..3 {
            let resp = self.send().await?;
            if !resp.results.is_empty() {
                return Ok(Some(resp.results));
            }
        }
        Ok(None)
    }

    /// Executes the search request and automatically fetches results across multiple pages
    /// until the specified number of results is reached.
    ///
    /// This is a convenience method that handles pagination and potential empty results.
    ///
    /// # Arguments
    ///
    /// * `num` - The minimum number of results to retrieve.
    ///
    /// # Errors
    ///
    /// Returns a [`reqwest::Error`] if any of the underlying requests fail after retries.
    pub async fn send_get_num(mut self, num: usize) -> Result<Vec<SearchResult>, reqwest::Error> {
        let mut pageno = 1;
        let mut ret = Vec::with_capacity(num + 50);
        while ret.len() < num {
            self.params.pageno = Some(pageno);
            match self.send_empty_check_retry().await {
                Ok(Some(results)) => ret.extend(results),
                Ok(None) => break,
                Err(_) => continue, // Retry on error
            }
            pageno += 1;
        }
        Ok(ret.into_iter().take(num).collect())
    }
}
