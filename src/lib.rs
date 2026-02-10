//! # SearXNG Client
//!
//! An asynchronous, type-safe client library for the SearXNG search engine API.
//!
//! This library provides a high-level API for searching across multiple engines,
//! handling the dynamic nature of SearXNG results through robust serialization
//! and a convenient builder pattern.

mod client;
mod response;
#[cfg(test)]
mod test;

pub use client::{ResponseFormat, SearXNGClient};
pub use response::SearchResponse;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search() -> anyhow::Result<()> {
        let client = SearXNGClient::new("http://localhost:8089/", ResponseFormat::Json);
        let response = client.search("rust programming").send().await?;
        println!("{:?}", response);
        let results = client.search("rust programming").send_get_num(10).await?;
        println!("{:?}", results);
        Ok(())
    }
}
