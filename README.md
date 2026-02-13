# searxng-client

[![Crates.io](https://img.shields.io/crates/v/searxng-client.svg)](https://crates.io/crates/searxng-client)
[![Docs.rs](https://docs.rs/searxng-client/badge.svg)](https://docs.rs/searxng-client)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, type-safe asynchronous SearXNG API client for Rust.

## Features

- **Type-Safe Results**: Strongly typed mapping for common SearXNG response fields.
- **Asynchronous**: Built on top of `reqwest` and compatible with modern async runtimes like `tokio`.
- **Resilient**: Designed to handle the dynamic and sometimes inconsistent nature of SearXNG API responses.
- **Ergonomic API**: Uses a fluent builder pattern to construct search queries.

## Installation

Add `searxng-client` to your `Cargo.toml`:

```toml
[dependencies]
searxng-client = "0.1"
# Required for async execution
tokio = { version = "1", features = ["full"] }
```

## Quick Start

Perform a search and process results in just a few lines of code:

```rust
use searxng_client::{SearXNGClient, ResponseFormat};
use searxng_client::response::SearchResult;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with your SearXNG instance URL
    let client = SearXNGClient::new("https://searx.be", ResponseFormat::Json);
    
    // Search for "rust programming" and fetch up to 10 results
    let results = client
        .search("rust programming")
        .send_get_num(10)
        .await?;

    println!("Found {} results.", results.len());
    
    // Print titles of the results
    for result in results {
        let title = match result {
            SearchResult::MainResult(m) => m.title,
            SearchResult::LegacyResult(l) => l.title,
        };
        println!("Title: {}", title);
    }
    
    Ok(())
}
```

## Resilience and API Stability

SearXNG aggregates results from various engines, leading to highly dynamic and sometimes inconsistent API responses. This library is built to navigate these challenges:

- **Graceful Degradation**: Most fields are wrapped in `Option<T>`, ensuring that missing fields from specific engines don't crash your application.
- **Dynamic Schemas**: For complex structures like `Infobox` attributes and URLs, we leverage `serde_json::Value` to capture arbitrary data patterns without losing the ability to parse the rest of the response.
- **Unified Interface**: The library automatically handles both `LegacyResult` and `MainResult` formats through untagged enums, providing a consistent way to access data regardless of the SearXNG instance version.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
