# Affinity Rust SDK

Generated client for the Affinity API, version `2026-09-28`. This is a source preview
at `0.1.0`; the generated interface may change before a stable release.

## Install and use

Validated with Rust 1.90. Add the GitHub dependency to `Cargo.toml`:

```toml
[dependencies]
affinity-health-sdk = { git = "https://github.com/affinity-health/affinity-rust", branch = "main" }
tokio = { version = "1", features = ["full"] }
```

```rust
use affinity_health_sdk::prelude::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ApiClient::new(ClientConfig {
        api_key: Some(std::env::var("AFFINITY_API_KEY")?),
        max_retries: 0,
        custom_headers: HashMap::from([
            ("Affinity-Version".into(), "2026-09-28".into()),
        ]),
        ..Default::default()
    })?;
    let page = client.orders.list_orders(
        &ListOrdersQueryRequest { limit: Some(20), ..Default::default() },
        None,
    ).await?;
    println!("{} orders", page.data.len());
    Ok(())
}
```

Pass required endpoint headers through `RequestOptions`, including
`RequestOptions::new().additional_header("Idempotency-Key", "your-stable-key")`
for creating orders. HTTPS uses rustls. This crate is not published on crates.io.

Use a server-side API key from `AFFINITY_API_KEY`. Never embed keys in a browser or
shipped application. The default base URL is `https://api.joinaffinityai.com`.
These examples disable automatic retries. Reuse the same idempotency key when
retrying a write that requires one. List responses expose data and cursor metadata;
pass the next cursor explicitly when fetching more records.

See [the generated reference](reference.md) for resource methods and types and
[Affinity documentation](https://docs.joinaffinityai.com) for API behavior.
Generated reference examples may assume registry publication; use the installation
instructions above while this SDK is available only from GitHub.

## Development

With Docker installed:

```sh
./scripts/check.sh
```

This builds/packages the SDK locally and checks synthetic HTTP requests, authentication,
API version headers, pagination parameters, response decoding, and failed writes.
It does not call the hosted API or publish a package.

The committed [OpenAPI contract](spec/affinity.openapi.json) is the source of truth.
[generation.json](generation.json) records the pinned Cloudflare Forge and Fern
versions and source hash. Generation is maintained in Affinity's SDK pipeline.
Do not edit generated models directly.
