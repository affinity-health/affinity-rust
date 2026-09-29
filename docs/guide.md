# Rust SDK proposal

> **Proposed interface.**
  These examples describe the SDK we plan to build. They are for review and do not run against the
  current release. Package versions and migration steps will follow approval.


Async Rust applications. Request examples belong inside a function that returns a Result. [Source repository](https://github.com/affinity-health/affinity-rust) · [All SDKs](https://docs.joinaffinityai.com/guides/reference/sdks/) · [Shared conventions](https://docs.joinaffinityai.com/guides/reference/sdks/methods/)

## Connect

Set `AFFINITY_API_KEY` to a Test API key on your server. The key selects Test or Live mode. Keep it out of browser and mobile code.

```rust
use affinity_health_sdk::{Affinity, RequestOptions, models::*};
use futures_util::TryStreamExt;

let api = Affinity::new(std::env::var("AFFINITY_API_KEY")?)?;
```

## With a practice key

The key identifies the practice. No practice ID or scoped client is needed.
The resource IDs below come from records in that practice.
Each section is a separate usage example, not one script to concatenate.

```rust
let patients = api.patients.list(PatientListParams { limit: Some(20), ..Default::default() }, None).await?;
let patient = api.patients.get(patient_id, None).await?;
let items = api.catalog.items.list(CatalogItemListParams { limit: Some(20), ..Default::default() }, None).await?;
```

## With a platform key

Pass the target practice with each practice-scoped request. Keep record data separate from request context and idempotency options.

```rust
let options = RequestOptions::new().practice_id(practice_id);
let patients = api.patients.list(
    PatientListParams { limit: Some(20), ..Default::default() },
    Some(options.clone()),
).await?;

let patient = api.patients.get(patient_id, Some(options)).await?;

api.patients.update(
    patient_id,
    PatientUpdateParams { email: Some("alex@example.com".into()), ..Default::default() },
    Some(RequestOptions::new()
        .practice_id(practice_id)),
).await?;
```

## Scope a workflow once

A scoped client remembers the practice for subsequent requests. It is immutable; the original client and other scoped clients stay independent.
A conflicting practice ID produces an error. Scoping never grants access to another practice.

```rust
let practice = api.for_practice(practice_id);

let patients = practice.patients.list(
    PatientListParams { limit: Some(20), ..Default::default() }, None,
).await?;

let items = practice.catalog.items.list(
    CatalogItemListParams { limit: Some(20), ..Default::default() }, None,
).await?;
```

The following examples use this scoped client. A practice-key client supports the same calls without the scoping step.

## Create, get, and update a patient

Use synthetic Test data. Routine writes generate a fresh idempotency key per call and preserve it during internal retries.
Supply your own persisted key when retrying across calls or process restarts.

```rust
let patient = practice.patients.create(PatientCreateParams {
    name: PatientName { first: "Alex".into(), last: "Example".into() },
    date_of_birth: "1990-01-01".into(),
    ..Default::default()
}, None).await?;

let saved = practice.patients.get(&patient.id, None).await?;
practice.patients.update(&patient.id, PatientUpdateParams {
    email: Some("alex@example.com".into()), ..Default::default()
}, None).await?;

practice.patients.update(&patient.id, PatientUpdateParams {
    status: Some("archived".into()), ..Default::default()
}, None).await?;
```

Archive patients whose records you need to retain. Permanent deletion is available only for patients without order history. No explicit idempotency key is needed.

```rust
practice.patients.delete(patient_id, None).await?;
```

## Create an order draft

`draft` is your application's prepared prescription data, using catalog and prescribing options from this practice.
An order contains 1–20 complete prescriptions for one patient. This example creates an unsigned draft.
It shows a platform call without a scoped client: practice context and the persisted key belong together in request options.

`job` is your persisted workflow record. Generate and save a unique key for each action before making its first request.

```rust
let order = api.orders.create(
    OrderCreateParams { patient_id: patient_id.into(), prescriptions: draft.prescriptions.clone(), ..Default::default() },
    Some(RequestOptions::new().practice_id(practice_id).idempotency_key(&job.create_order_key)),
).await?;
```

## Sign and submit

`review` is your saved clinician review and signing consent for this exact order.
Store the reviewed revision, authorized prescriber ID, and explicit attestation together.
Your API key needs `orders:sign`. Never infer consent or automatically replace a stale revision.

```rust
practice.orders.sign(
    order_id,
    OrderSignParams {
        prescriber: PrescriberSelector::id(&review.prescriber_id),
        expected_revision: review.order_revision.clone(),
        signature_attestation: review.signature_attestation,
    },
    Some(RequestOptions::new().idempotency_key(&job.sign_order_key)),
).await?;

let submission = practice.orders.submit(order_id,
    Some(RequestOptions::new().idempotency_key(&job.submit_order_key)),
).await?;
```

Use separate keys for creating, signing, and submitting. After an uncertain response, retry the same action with the same key and unchanged data.
A revision conflict requires renewed clinician review before another signing attempt.

Submission means queued, not accepted by the pharmacy. Inspect the result and track order events or webhooks.
After a reported partial submission failure, retry only the unconfirmed send with a new submission key.

## Read more than one page

The list method returns one page. Pass the last record's ID to request the next page.
The iterator fetches pages as you consume records; it does not load the full collection into memory.
`syncPatient` or its language equivalent represents your application's record handler.

```rust
let page = practice.patients.list(
    PatientListParams { limit: Some(20), ..Default::default() }, None,
).await?;

if page.has_more {
    if let Some(last) = page.data.last() {
        let next = practice.patients.list(PatientListParams {
            limit: Some(20), starting_after: Some(last.id.clone()), ..Default::default()
        }, None).await?;
    }
}

let mut patients = practice.patients.iterate(
    PatientListParams { limit: Some(100), ..Default::default() }, None,
);

while let Some(patient) = patients.try_next().await? {
    sync_patient(patient).await?;
}
```

## Handle errors

API failures expose status, code, request ID, retryability, and an optional retry delay in seconds.
Log those fields without logging patient data or credentials. Transport failures remain distinguishable from API responses.

```rust
if let Err(error) = practice.patients.get(patient_id, None).await {
    eprintln!("status={:?} code={:?} request={:?} retryable={} retryAfter={:?}",
        error.status(), error.code(), error.request_id(),
        error.retryable(), error.retry_after());
    return Err(error.into());
}
```

Retryability is a transport hint, not permission to repeat a clinical action with a new key.
Keep the same key and body for an uncertain write. Validation and authorization errors require a corrected request.
See [API errors](https://docs.joinaffinityai.com/errors/) for recovery guidance.

## Platform directory and webhooks

Use the root platform client to list its practices and webhook endpoints. These calls do not need a target practice or an idempotency key.
The webhook list belongs to the platform itself. Access to another organization's endpoints still requires an explicit grant.

```rust
let practices = api.practices.list(
    PracticeListParams { limit: Some(20), ..Default::default() }, None,
).await?;

let selected = api.practices.get(practice_id, None).await?;
let endpoints = api.webhooks.endpoints.list(
    WebhookEndpointListParams { limit: Some(20), ..Default::default() }, None,
).await?;
```

## More resources

Use the same conventions for addresses, allergies, locations, team members, and nested order resources.
[Resource directory](https://docs.joinaffinityai.com/guides/reference/sdks/methods/) · [API reference](https://docs.joinaffinityai.com/api/) · [Webhooks](https://docs.joinaffinityai.com/guides/webhooks/)
