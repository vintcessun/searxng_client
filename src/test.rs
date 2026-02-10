use async_trait::async_trait;
use reqwest::Response;
use serde::de::DeserializeOwned;

#[async_trait]
pub trait SmartJsonExt {
    async fn json_test<T: DeserializeOwned>(self) -> anyhow::Result<T>;
}

#[async_trait]
impl SmartJsonExt for Response {
    async fn json_test<T: DeserializeOwned>(self) -> anyhow::Result<T> {
        let full_body = self.text().await?;

        match serde_json::from_str::<T>(&full_body) {
            Ok(val) => Ok(val),
            Err(e) => {
                println!("\n--- [DEBUG] JSON DECODE ERROR ---");
                println!("Reason: {}", e);
                println!("At: Line {}, Column {}", e.line(), e.column());
                println!("Raw Body:\n{}", full_body);
                println!("---------------------------------\n");

                Err(anyhow::anyhow!(e).context("JSON decoding failed in debug mode"))
            }
        }
    }
}
