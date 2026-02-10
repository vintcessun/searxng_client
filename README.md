# searxng_client

A high-performance, type-safe asynchronous SearXNG API client for Rust.

## Quick Start

Add the following to your `Cargo.toml`:

```toml
[dependencies]
searxng_client = "0.1"
tokio = { version = "1", features = ["full"] }
```

Initiate a search in less than 10 lines of code:

```rust
use searxng_client::{SearXNGClient, ResponseFormat};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SearXNGClient::new("https://searx.be", ResponseFormat::Json);
    let results = client.search("rust programming").send_get_num(10).await?;

    results.iter().for_each(|res| println!("Title: {}", res.title()));
    Ok(())
}
```

## Handling API Instability

SearXNG aggregates results from various engines, leading to highly dynamic and sometimes inconsistent API responses. This library is designed to be resilient against such instability:

- **Graceful Degradation**: Most fields are wrapped in `Option<T>`, ensuring that a missing field from a specific engine doesn't crash your application.
- **Dynamic Schemas**: For complex structures like `Infobox` attributes and URLs, we utilize `serde_json::Value` to capture arbitrary data patterns without losing the ability to parse the rest of the response.
- **Legacy Support**: The library automatically handles both `LegacyResult` and `MainResult` formats through an untagged enum, providing a unified interface regardless of the SearXNG instance version.

## Core Dependencies

- **tokio**: Providing a reliable asynchronous runtime.
- **reqwest**: For high-performance, async HTTP requests.
- **serde**: Industry-standard serialization and deserialization.
- **chrono/iso8601**: Precise handling of search result timestamps and durations.

## License

This project is licensed under the MIT License.