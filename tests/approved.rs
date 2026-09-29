use affinity_health_sdk::{Affinity, RequestOptions, SdkOptions, models::*};
use futures_util::TryStreamExt;
#[tokio::test]
async fn approved_interface()->Result<(),Box<dyn std::error::Error>>{
 let base="http://127.0.0.1:5199/rust-practice-retry";let http=reqwest::Client::new();http.get(format!("{base}/reset")).send().await?;
 let api=Affinity::with_options("test",SdkOptions{base_url:base.into(),max_retries:1,..Default::default()})?;
 let patient=api.patients.create(PatientCreateParams{name:PatientName{first:"Alex".into(),last:"Example".into(),..Default::default()},date_of_birth:"1990-01-01".into(),..Default::default()},None).await?;assert_eq!(patient.id,"pat_a");
 api.patients.update(&patient.id,PatientUpdateParams::default().clear("email")?,None).await?;api.patients.delete(&patient.id,None).await?;
 let mut patients=api.patients.iterate(PatientListParams{limit:Some(1),query:Some("Alex".into()),..Default::default()},None);let mut ids=Vec::new();while let Some(p)=patients.try_next().await?{ids.push(p.id);}assert_eq!(ids,vec!["pat_a","pat_b"]);
 assert!(api.patients.get("pat_a",Some(RequestOptions::new().practice_id("prac_b"))).await.is_err());assert!(api.orders.submit("ord_a",None).await.is_err());
 api.orders.sign("ord_a",OrderSignParams{prescriber:PrescriberSelector::id("prov_a"),expected_revision:"rev_reviewed".into(),signature_attestation:true,..Default::default()},Some(RequestOptions::new().idempotency_key("sign_job"))).await?;
 api.orders.submit("ord_a",Some(RequestOptions::new().idempotency_key("submit_job"))).await?;
 let error=api.patients.get("pat_error",None).await.unwrap_err();assert_eq!(error.status(),Some(429));assert_eq!(error.code(),Some("rate_limited"));assert_eq!(error.request_id(),Some("req_a"));assert_eq!(error.retry_after(),Some(0.0));assert!(!error.to_string().contains("private"));
 let trace:serde_json::Value=http.get(format!("{base}/trace")).send().await?.json().await?;let requests=trace.as_array().unwrap();assert_eq!(requests.iter().filter(|r|r["path"]=="/v1/auth/access").count(),1);let writes:Vec<_>=requests.iter().filter(|r|r["method"]=="PATCH").collect();assert_eq!(writes.len(),2);assert_eq!(writes[0]["key"],writes[1]["key"]);assert_eq!(writes[0]["body"],serde_json::json!({"email":null}));
 assert!(api.for_practice("").patients.get("pat_a",None).await.is_err());
 assert!(api.for_practice("prac_a").for_practice("prac_b").patients.get("pat_a",None).await.is_err());
 assert!(api.patients.get("pat_a",Some(RequestOptions::new().api_key("different"))).await.is_err());
 Ok(())
}
