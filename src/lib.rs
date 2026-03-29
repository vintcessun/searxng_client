//! # SearXNG Client
//!
//! An asynchronous, type-safe client library for the SearXNG search engine API.
//!
//! This library provides a high-level API for searching across multiple engines,
//! handling the dynamic nature of SearXNG results through robust serialization
//! and a convenient builder pattern.

pub mod client;
pub mod response;
#[cfg(test)]
mod test;

pub use client::{ResponseFormat, SearXNGClient};
pub use response::SearchResponse;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_client() -> SearXNGClient {
        let url =
            std::env::var("SEARXNG_URL").unwrap_or_else(|_| "http://localhost:8089/".to_string());
        println!("url: {url}");
        SearXNGClient::new(&url, ResponseFormat::Json)
    }

    #[tokio::test]
    async fn test_search() -> anyhow::Result<()> {
        let response = test_client().search("rust programming").send().await?;
        println!("{:?}", response);
        Ok(())
    }

    #[tokio::test]
    async fn test_search_get_num() -> anyhow::Result<()> {
        let results = test_client()
            .search("rust programming")
            .send_get_num(10)
            .await?;
        println!("{:?}", results);
        Ok(())
    }
}
