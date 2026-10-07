# Affinity Rust SDK

Server-side client for the Affinity API. Requires Rust 1.90+ with an async runtime.

Version 0.3.0 targets the deployed Affinity API contract used by TypeScript SDK 1.16.0. Install this SDK from GitHub; registry publication is deferred.

## Install from source

```sh
cargo add affinity-health-sdk --git https://github.com/affinity-health/affinity-rust --tag v0.3.0
```

## Use

Set `AFFINITY_API_KEY` to a Test practice key on your server. Keep API keys out of browser and mobile code.

```rust
use affinity_health_sdk::{Affinity, models::*};

let api = Affinity::new(std::env::var("AFFINITY_API_KEY")?)?;
let patients = api.patients.list(
    PatientListParams { limit: Some(20), ..Default::default() },
    None,
).await?;
```

Put request statements inside an async function returning a `Result`. Use `futures_util::TryStreamExt` for iterators.

Practice keys identify their practice automatically. Platform keys pass a practice ID in request options or use a scoped client.

See the [SDK guide](docs/guide.md) for platform requests, patient updates, signing, submission, pagination, and errors.
Routine patient writes generate an idempotency key. Persist your own keys for order creation, signing, and submission.

Defaults: API `2026-09-28`, a 60-second timeout, and no automatic retries.

## Verify

```sh
./scripts/check.sh
```

The tests use synthetic fixtures on loopback. The fixture runner requires Python 3; Docker runs the language toolchain for the `scripts/check.sh` commands.

Generated with Cloudflare Forge, Fern, and Affinity's facade generator. [generation.json](generation.json) records the pinned inputs. Fix the generator in the Affinity monorepo before regenerating client code.
