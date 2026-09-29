#![allow(unused)]
use affinity_health_sdk::{Affinity,RequestOptions,models::*};use futures_util::TryStreamExt;#[derive(Default)]struct Draft{prescriptions:Vec<OrderCreateParamsPrescriptionsItem>}#[derive(Default)]struct Job{create_order_key:String,sign_order_key:String,submit_order_key:String}#[derive(Default)]struct Review{prescriber_id:String,order_revision:String,signature_attestation:bool}async fn sync_patient<T>(_:T)->Result<(),Box<dyn std::error::Error>>{Ok(())}
async fn example0()->Result<(),Box<dyn std::error::Error>>{let api=Affinity::new("test")?;let practice=api.for_practice("prac_a");let practice_id="prac_a";let patient_id="pat_a";let order_id="ord_a";let draft=Draft::default();let job=Job::default();let review=Review::default();
let patients = api.patients.list(
    PatientListParams { limit: Some(20), ..Default::default() },
    None,
).await?;

let patient = api.patients.get(patient_id, None).await?;
let items = api.catalog.items.list(
    CatalogItemListParams { limit: Some(20), ..Default::default() },
    None,
).await?;


Ok(())}
async fn example1()->Result<(),Box<dyn std::error::Error>>{let api=Affinity::new("test")?;let practice=api.for_practice("prac_a");let practice_id="prac_a";let patient_id="pat_a";let order_id="ord_a";let draft=Draft::default();let job=Job::default();let review=Review::default();
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


Ok(())}
async fn example2()->Result<(),Box<dyn std::error::Error>>{let api=Affinity::new("test")?;let practice=api.for_practice("prac_a");let practice_id="prac_a";let patient_id="pat_a";let order_id="ord_a";let draft=Draft::default();let job=Job::default();let review=Review::default();
let practice = api.for_practice(practice_id);

let patients = practice.patients.list(
    PatientListParams { limit: Some(20), ..Default::default() }, None,
).await?;

let items = practice.catalog.items.list(
    CatalogItemListParams { limit: Some(20), ..Default::default() }, None,
).await?;


Ok(())}
async fn example3()->Result<(),Box<dyn std::error::Error>>{let api=Affinity::new("test")?;let practice=api.for_practice("prac_a");let practice_id="prac_a";let patient_id="pat_a";let order_id="ord_a";let draft=Draft::default();let job=Job::default();let review=Review::default();
let patient = practice.patients.create(PatientCreateParams {
    name: PatientName { first: "Alex".into(), last: "Example".into(), ..Default::default() },
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

Ok(())}
async fn example4()->Result<(),Box<dyn std::error::Error>>{let api=Affinity::new("test")?;let practice=api.for_practice("prac_a");let practice_id="prac_a";let patient_id="pat_a";let order_id="ord_a";let draft=Draft::default();let job=Job::default();let review=Review::default();
practice.patients.delete(patient_id, None).await?;

Ok(())}
async fn example5()->Result<(),Box<dyn std::error::Error>>{let api=Affinity::new("test")?;let practice=api.for_practice("prac_a");let practice_id="prac_a";let patient_id="pat_a";let order_id="ord_a";let draft=Draft::default();let job=Job::default();let review=Review::default();
let order = api.orders.create(
    OrderCreateParams {
        patient_id: patient_id.into(),
        prescriptions: draft.prescriptions.clone(),
        ..Default::default()
    },
    Some(RequestOptions::new()
        .practice_id(practice_id)
        .idempotency_key(&job.create_order_key)),
).await?;


Ok(())}
async fn example6()->Result<(),Box<dyn std::error::Error>>{let api=Affinity::new("test")?;let practice=api.for_practice("prac_a");let practice_id="prac_a";let patient_id="pat_a";let order_id="ord_a";let draft=Draft::default();let job=Job::default();let review=Review::default();
practice.orders.sign(
    order_id,
    OrderSignParams {
        prescriber: PrescriberSelector::id(&review.prescriber_id),
        expected_revision: review.order_revision.clone(),
        signature_attestation: review.signature_attestation,
        ..Default::default()
    },
    Some(RequestOptions::new().idempotency_key(&job.sign_order_key)),
).await?;

let submission = practice.orders.submit(order_id,
    Some(RequestOptions::new().idempotency_key(&job.submit_order_key)),
).await?;


Ok(())}
async fn example7()->Result<(),Box<dyn std::error::Error>>{let api=Affinity::new("test")?;let practice=api.for_practice("prac_a");let practice_id="prac_a";let patient_id="pat_a";let order_id="ord_a";let draft=Draft::default();let job=Job::default();let review=Review::default();
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

Ok(())}
async fn example8()->Result<(),Box<dyn std::error::Error>>{let api=Affinity::new("test")?;let practice=api.for_practice("prac_a");let practice_id="prac_a";let patient_id="pat_a";let order_id="ord_a";let draft=Draft::default();let job=Job::default();let review=Review::default();
if let Err(error) = practice.patients.get(patient_id, None).await {
    eprintln!("status={:?} code={:?} request={:?} retryable={} retryAfter={:?}",
        error.status(), error.code(), error.request_id(),
        error.retryable(), error.retry_after());
    return Err(error.into());
}

Ok(())}
async fn example9()->Result<(),Box<dyn std::error::Error>>{let api=Affinity::new("test")?;let practice=api.for_practice("prac_a");let practice_id="prac_a";let patient_id="pat_a";let order_id="ord_a";let draft=Draft::default();let job=Job::default();let review=Review::default();
let practices = api.practices.list(
    PracticeListParams { limit: Some(20), ..Default::default() }, None,
).await?;

let selected = api.practices.get(practice_id, None).await?;
let endpoints = api.webhooks.endpoints.list(
    WebhookEndpointListParams { limit: Some(20), ..Default::default() }, None,
).await?;


Ok(())}
fn main(){}