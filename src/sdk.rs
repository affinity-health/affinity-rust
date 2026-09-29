use std::{sync::Arc,collections::HashMap,time::Duration};
use serde::Serialize;
use serde_json::{Value,json};
use tokio::sync::Mutex;
use crate::core::RequestOptions;
use crate::api::types::*;
use self::models::*;

#[derive(Debug,thiserror::Error)]
pub enum SdkError {
 #[error("Affinity API request failed ({status})")]
 Api {status:u16,code:Option<String>,request_id:Option<String>,retry_after:Option<f64>},
 #[error("Affinity transport failed")]
 Transport(#[from] reqwest::Error),
 #[error("Affinity response could not be decoded")]
 Json(#[from] serde_json::Error),
 #[error("{0}")]
 Invalid(String),
}
impl SdkError {
 pub fn status(&self)->Option<u16>{match self{Self::Api{status,..}=>Some(*status),_=>None}}
 pub fn code(&self)->Option<&str>{match self{Self::Api{code,..}=>code.as_deref(),_=>None}}
 pub fn request_id(&self)->Option<&str>{match self{Self::Api{request_id,..}=>request_id.as_deref(),_=>None}}
 pub fn retry_after(&self)->Option<f64>{match self{Self::Api{retry_after,..}=>*retry_after,_=>None}}
 pub fn retryable(&self)->bool{matches!(self.status(),Some(408|429|500|502|503|504))}
}
#[derive(Clone)]
pub struct SdkOptions {pub base_url:String,pub timeout:Duration,pub max_retries:u32}
impl Default for SdkOptions{fn default()->Self{Self{base_url:"https://api.joinaffinityai.com".into(),timeout:Duration::from_secs(60),max_retries:0}}}
struct SdkTransport {key:String,config:SdkOptions,http:reqwest::Client,identity:Mutex<Option<Value>>}
impl SdkTransport {
 fn new(key:String,config:SdkOptions)->Result<Self,SdkError>{if key.trim().is_empty(){return Err(SdkError::Invalid("An API key is required".into()))}if config.timeout.is_zero()||config.max_retries>10{return Err(SdkError::Invalid("Invalid timeout or retry limit".into()))}Ok(Self{key,http:reqwest::Client::builder().timeout(config.timeout).redirect(reqwest::redirect::Policy::none()).build()?,config,identity:Mutex::new(None)})}
 async fn access(&self)->Result<Value,SdkError>{let mut identity=self.identity.lock().await;if let Some(value)=identity.as_ref(){return Ok(value.clone())}let result=self.request("/v1/auth/access","GET",None,HashMap::new()).await?;let subject=&result["serviceAccount"];if !subject["subjectType"].is_string()||!subject["subjectId"].is_string(){return Err(SdkError::Invalid("Invalid API key access response".into()))}*identity=Some(subject.clone());Ok(subject.clone())}
 async fn request(&self,path:&str,method:&str,body:Option<Value>,headers:HashMap<String,String>)->Result<Value,SdkError>{
  let retries=if method=="GET"||headers.contains_key("Idempotency-Key"){self.config.max_retries}else{0};
  for attempt in 0..=retries {
   let mut request=self.http.request(reqwest::Method::from_bytes(method.as_bytes()).map_err(|_|SdkError::Invalid("Invalid HTTP method".into()))?,format!("{}{}",self.config.base_url.trim_end_matches('/'),path)).bearer_auth(&self.key).header("Affinity-Version","2026-09-28");
   for(k,v)in &headers{request=request.header(k,v);}if let Some(ref data)=body{request=request.json(data);}
   let mut delay=0.25*2f64.powi(attempt as i32);
   let result=async {let response=request.send().await?;let status=response.status().as_u16();let retry_after=response.headers().get("Retry-After").and_then(|v|v.to_str().ok()).and_then(|v|v.parse::<f64>().ok().or_else(||httpdate::parse_http_date(v).ok().map(|d|d.duration_since(std::time::SystemTime::now()).unwrap_or_default().as_secs_f64()))).map(|s|s.max(0.0));let bytes=response.bytes().await?;if (200..300).contains(&status){return Ok(if bytes.is_empty(){Value::Null}else{serde_json::from_slice(&bytes)?})}let problem:Value=serde_json::from_slice(&bytes).unwrap_or(Value::Null);Err(SdkError::Api{status,code:problem["code"].as_str().map(str::to_owned),request_id:problem["requestId"].as_str().map(str::to_owned),retry_after})}.await;
   match result {Ok(value)=>return Ok(value),Err(error)=>{if attempt==retries||(!error.retryable()&&!matches!(error,SdkError::Transport(_))){return Err(error)}delay=delay.max(error.retry_after().unwrap_or(0.0));}}
   tokio::time::sleep(Duration::from_secs_f64(delay.min(30.0))).await;
  }
  unreachable!()
 }
}
#[derive(Clone)]
struct SdkContext{transport:Arc<SdkTransport>,practice_id:Option<String>,invalid:Option<String>}
impl SdkContext{
 async fn call<T:serde::de::DeserializeOwned>(&self,operation:&str,ids:Vec<String>,params:Value,options:Option<RequestOptions>)->Result<T,SdkError>{
  if let Some(error)=&self.invalid{return Err(SdkError::Invalid(error.clone()))}
  let operations:Value=serde_json::from_str(SDK_OPERATIONS)?;let op=&operations[operation];let options=options.unwrap_or_default();
  if options.api_key.is_some()||options.token.is_some()||options.max_retries.is_some()||options.timeout_seconds.is_some()||!options.additional_headers.is_empty()||!options.additional_query_params.is_empty(){return Err(SdkError::Invalid("Legacy transport overrides are unsupported; configure the Affinity client with SdkOptions".into()))}
  if op["rootOnly"]==true&&self.practice_id.is_some(){return Err(SdkError::Invalid("Use the root client for platform-wide operations".into()))}
  if self.practice_id.is_some()&&options.practice_id.is_some()&&self.practice_id!=options.practice_id{return Err(SdkError::Invalid("Conflicting practice ID".into()))}
  let mut key=options.idempotency_key;let policy=op["idempotency"].as_str().unwrap();if key.as_ref().is_some_and(|k|k.trim().is_empty()){return Err(SdkError::Invalid("idempotency_key must not be empty".into()))}if policy=="required"&&key.is_none(){return Err(SdkError::Invalid("A persisted idempotency_key is required".into()))}if policy=="none"&&key.is_some(){return Err(SdkError::Invalid("This endpoint does not support idempotency keys".into()))}if policy=="auto"&&key.is_none(){key=Some(uuid::Uuid::new_v4().to_string());}
  let mut practice=options.practice_id.or(self.practice_id.clone());let scope=op["practice"].as_str().unwrap();if scope!="none"{let subject=self.transport.access().await?;if subject["subjectType"]=="practice"{let selected=subject["subjectId"].as_str().unwrap().to_string();if practice.as_ref().is_some_and(|p|p!=&selected){return Err(SdkError::Invalid("Practice context conflicts with the API key".into()))}practice=Some(selected);}if practice.as_ref().is_none_or(|p|p.trim().is_empty()){return Err(SdkError::Invalid("A platform key requires practice_id".into()))}}else if practice.is_some()&&self.practice_id.is_none(){return Err(SdkError::Invalid("This endpoint does not accept practice context".into()))}
  let mut data=params.as_object().cloned().unwrap_or_default();if operation=="updatePatient"&&data.get("status")==Some(&json!("archived")){data.insert("status".into(),json!("inactive"));}if data.contains_key("practiceId"){return Err(SdkError::Invalid("Pass practice_id in request options".into()))}
  let names=op["ids"].as_array().unwrap();let mut path=op["path"].as_str().unwrap().to_string();for(i,name)in names.iter().enumerate(){let id=ids.get(i).filter(|id|!id.trim().is_empty()).ok_or_else(||SdkError::Invalid("A resource ID is required".into()))?;path=path.replace(&format!("{{{}}}",name.as_str().unwrap()),&escape(id));}
  if scope=="path"{path=path.replace("{practiceId}",&escape(practice.as_ref().unwrap()));}if scope=="body"||scope=="query"{data.insert("practiceId".into(),json!(practice));}
  if scope=="order"{let index=names.iter().position(|n|n=="orderId").unwrap();let order=self.transport.request(&format!("/v1/orders/{}",escape(&ids[index])),"GET",None,HashMap::new()).await?;if order["practiceId"].as_str()!=practice.as_deref(){return Err(SdkError::Invalid("Order does not belong to the selected practice".into()))}if operation=="getOrder"{return Ok(serde_json::from_value(order)?);}}
  let mut query=Vec::new();for name in op["query"].as_array().unwrap(){let name=name.as_str().unwrap();if let Some(value)=data.remove(name){if !value.is_null(){query.push(format!("{}={}",escape(name),escape(&value.as_str().map(str::to_owned).unwrap_or_else(||value.to_string()))));}}}if !query.is_empty(){path.push('?');path.push_str(&query.join("&"));}
  let mut headers=HashMap::new();for(name,value)in [("organizationId",options.organization_id),("actorId",options.actor_id),("actorType",options.actor_type)]{if let (Some(header),Some(value))=(op["headers"][name].as_str(),value){headers.insert(header.into(),value);}}if let Some(key)=key{headers.insert("Idempotency-Key".into(),key);}
  let response=self.transport.request(&path,op["verb"].as_str().unwrap(),if op["body"]==true{Some(Value::Object(data))}else{None},headers).await?;Ok(serde_json::from_value(response)?)
 }
 fn iterate<T:serde::de::DeserializeOwned+Send+'static>(&self,operation:&'static str,ids:Vec<String>,params:Value,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<T,SdkError>>{
  let context=self.clone();Box::pin(futures::stream::try_unfold((context,params,options,ids,Vec::<Value>::new().into_iter(),false),move |(context,mut params,options,ids,mut items,mut done)|async move{
   if !params["endingBefore"].is_null(){return Err(SdkError::Invalid("iterate supports forward pagination".into()))}
   loop{if let Some(item)=items.next(){let item=serde_json::from_value(item)?;return Ok(Some((item,(context,params,options,ids,items,done))))}if done{return Ok(None)}let page:Value=context.call(operation,ids.clone(),params.clone(),options.clone()).await?;let records=page["data"].as_array().ok_or_else(||SdkError::Invalid("Invalid page".into()))?;done=page["hasMore"]!=true;if !done{let cursor=records.last().and_then(|r|r["id"].as_str()).ok_or_else(||SdkError::Invalid("Pagination did not advance".into()))?;if params["startingAfter"]==cursor{return Err(SdkError::Invalid("Pagination did not advance".into()))}params["startingAfter"]=json!(cursor);}items=records.clone().into_iter();}
  }))
 }
}
fn escape(value:&str)->String{percent_encoding::utf8_percent_encode(value,percent_encoding::NON_ALPHANUMERIC).to_string()}

const SDK_OPERATIONS:&str=r###"{"listPracticeLocations":{"id":"listPracticeLocations","group":"locations","method":"list","verb":"GET","path":"/v1/practices/{practiceId}/locations","ids":[],"paramsName":"LocationListParams","hasParams":true,"paramsRequired":false,"response":"ListPracticeLocationsResponse","paginated":true,"query":["limit","startingAfter","endingBefore","status"],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"createPracticeLocation":{"id":"createPracticeLocation","group":"locations","method":"create","verb":"POST","path":"/v1/practices/{practiceId}/locations","ids":[],"paramsName":"LocationCreateParams","hasParams":true,"paramsRequired":true,"response":"CreatePracticeLocationResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"path","rootOnly":false,"idempotency":"auto"},"getPracticeLocation":{"id":"getPracticeLocation","group":"locations","method":"get","verb":"GET","path":"/v1/practices/{practiceId}/locations/{locationId}","ids":["locationId"],"paramsName":"LocationGetParams","hasParams":false,"paramsRequired":false,"response":"GetPracticeLocationResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"updatePracticeLocation":{"id":"updatePracticeLocation","group":"locations","method":"update","verb":"PATCH","path":"/v1/practices/{practiceId}/locations/{locationId}","ids":["locationId"],"paramsName":"LocationUpdateParams","hasParams":true,"paramsRequired":false,"response":"UpdatePracticeLocationResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"path","rootOnly":false,"idempotency":"auto"},"archivePracticeLocation":{"id":"archivePracticeLocation","group":"locations","method":"archive","verb":"POST","path":"/v1/practices/{practiceId}/locations/{locationId}/archive","ids":["locationId"],"paramsName":"LocationArchiveParams","hasParams":false,"paramsRequired":false,"response":"ArchivePracticeLocationResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"auto"},"createPlatformPracticeApiKey":{"id":"createPlatformPracticeApiKey","group":"apiKeys","method":"create","verb":"POST","path":"/v1/practices/{practiceId}/api-keys","ids":[],"paramsName":"ApiKeyCreateParams","hasParams":true,"paramsRequired":true,"response":"CreatePlatformPracticeApiKeyResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"path","rootOnly":false,"idempotency":"required"},"getAccount":{"id":"getAccount","group":"account","method":"get","verb":"GET","path":"/v1/account","ids":[],"paramsName":"AccountGetParams","hasParams":true,"paramsRequired":false,"response":"GetAccountResponse","paginated":false,"query":["orgId"],"headers":{},"body":false,"practice":"none","rootOnly":true,"idempotency":"none"},"listCatalogItems":{"id":"listCatalogItems","group":"catalog.items","method":"list","verb":"GET","path":"/v1/catalog/items","ids":[],"paramsName":"CatalogItemListParams","hasParams":true,"paramsRequired":false,"response":"ListCatalogItemsResponse","paginated":true,"query":["view","relatedToCatalogItemId","catalogKind","sort","catalogItemId","availability","pharmacyIds","dosageForms","endingBefore","hideControlledSubstances","hideUnpriced","limit","orgId","practiceId","query","requirement","routes","startingAfter"],"headers":{},"body":false,"practice":"query","rootOnly":false,"idempotency":"none"},"listPharmacies":{"id":"listPharmacies","group":"pharmacies","method":"list","verb":"GET","path":"/v1/pharmacies","ids":[],"paramsName":"PharmacyListParams","hasParams":true,"paramsRequired":false,"response":"ListPharmaciesResponse","paginated":true,"query":["endingBefore","limit","orgId","pharmacyId","query","shipsToState","startingAfter"],"headers":{},"body":false,"practice":"none","rootOnly":false,"idempotency":"none"},"listShippingOptions":{"id":"listShippingOptions","group":"catalog.shippingOptions","method":"list","verb":"GET","path":"/v1/catalog/items/{catalogItemId}/shipping-options","ids":["catalogItemId"],"paramsName":"ShippingOptionListParams","hasParams":true,"paramsRequired":true,"response":"ListShippingOptionsResponse","paginated":false,"query":["destinationState","destinationType"],"headers":{},"body":false,"practice":"none","rootOnly":false,"idempotency":"none"},"listOrders":{"id":"listOrders","group":"orders","method":"list","verb":"GET","path":"/v1/orders","ids":[],"paramsName":"OrderListParams","hasParams":true,"paramsRequired":false,"response":"ListOrdersResponse","paginated":true,"query":["query","externalOrderId","createdAfter","createdBefore","endingBefore","limit","orderId","patientId","patientExternalId","practiceId","sort","startingAfter","status"],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":false,"practice":"query","rootOnly":false,"idempotency":"none"},"createOrder":{"id":"createOrder","group":"orders","method":"create","verb":"POST","path":"/v1/orders","ids":[],"paramsName":"OrderCreateParams","hasParams":true,"paramsRequired":true,"response":"CreateOrderResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"body","rootOnly":false,"idempotency":"required"},"getOrder":{"id":"getOrder","group":"orders","method":"get","verb":"GET","path":"/v1/orders/{orderId}","ids":["orderId"],"paramsName":"OrderGetParams","hasParams":false,"paramsRequired":false,"response":"GetOrderResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":false,"practice":"order","rootOnly":false,"idempotency":"none"},"cancelOrder":{"id":"cancelOrder","group":"orders","method":"cancel","verb":"POST","path":"/v1/orders/{orderId}/cancel","ids":["orderId"],"paramsName":"OrderCancelParams","hasParams":true,"paramsRequired":true,"response":"CancelOrderResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"order","rootOnly":false,"idempotency":"required"},"actOnOrderException":{"id":"actOnOrderException","group":"orders.exceptions","method":"act","verb":"POST","path":"/v1/orders/{orderId}/exceptions/{exceptionId}/actions","ids":["orderId","exceptionId"],"paramsName":"OrderExceptionActParams","hasParams":true,"paramsRequired":true,"response":"ActOnOrderExceptionResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"order","rootOnly":false,"idempotency":"required"},"listOrderEvents":{"id":"listOrderEvents","group":"orders.events","method":"list","verb":"GET","path":"/v1/orders/{orderId}/events","ids":["orderId"],"paramsName":"OrderEventListParams","hasParams":true,"paramsRequired":false,"response":"ListOrderEventsResponse","paginated":true,"query":["endingBefore","limit","startingAfter"],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":false,"practice":"order","rootOnly":false,"idempotency":"none"},"listWebhookEndpoints":{"id":"listWebhookEndpoints","group":"webhooks.endpoints","method":"list","verb":"GET","path":"/v1/webhook-endpoints","ids":[],"paramsName":"WebhookEndpointListParams","hasParams":true,"paramsRequired":false,"response":"ListWebhookEndpointsResponse","paginated":true,"query":["endingBefore","limit","startingAfter"],"headers":{"organizationId":"X-Affinity-Organization-Id"},"body":false,"practice":"none","rootOnly":true,"idempotency":"none"},"createWebhookEndpoint":{"id":"createWebhookEndpoint","group":"webhooks.endpoints","method":"create","verb":"POST","path":"/v1/webhook-endpoints","ids":[],"paramsName":"WebhookEndpointCreateParams","hasParams":true,"paramsRequired":true,"response":"CreateWebhookEndpointResponse","paginated":false,"query":[],"headers":{"organizationId":"X-Affinity-Organization-Id"},"body":true,"practice":"none","rootOnly":true,"idempotency":"required"},"updateWebhookEndpoint":{"id":"updateWebhookEndpoint","group":"webhooks.endpoints","method":"update","verb":"PATCH","path":"/v1/webhook-endpoints/{endpointId}","ids":["endpointId"],"paramsName":"WebhookEndpointUpdateParams","hasParams":true,"paramsRequired":false,"response":"UpdateWebhookEndpointResponse","paginated":false,"query":[],"headers":{"organizationId":"X-Affinity-Organization-Id"},"body":true,"practice":"none","rootOnly":true,"idempotency":"required"},"deleteWebhookEndpoint":{"id":"deleteWebhookEndpoint","group":"webhooks.endpoints","method":"delete","verb":"DELETE","path":"/v1/webhook-endpoints/{endpointId}","ids":["endpointId"],"paramsName":"WebhookEndpointDeleteParams","hasParams":false,"paramsRequired":false,"response":"DeleteWebhookEndpointResponse","paginated":false,"query":[],"headers":{"organizationId":"X-Affinity-Organization-Id"},"body":false,"practice":"none","rootOnly":true,"idempotency":"required"},"rotateWebhookEndpointSecret":{"id":"rotateWebhookEndpointSecret","group":"webhooks.endpoints","method":"rotateSecret","verb":"POST","path":"/v1/webhook-endpoints/{endpointId}/rotate-secret","ids":["endpointId"],"paramsName":"WebhookEndpointRotateSecretParams","hasParams":false,"paramsRequired":false,"response":"RotateWebhookEndpointSecretResponse","paginated":false,"query":[],"headers":{"organizationId":"X-Affinity-Organization-Id"},"body":false,"practice":"none","rootOnly":true,"idempotency":"required"},"testWebhookEndpoint":{"id":"testWebhookEndpoint","group":"webhooks.endpoints","method":"test","verb":"POST","path":"/v1/webhook-endpoints/{endpointId}/test","ids":["endpointId"],"paramsName":"WebhookEndpointTestParams","hasParams":false,"paramsRequired":false,"response":"TestWebhookEndpointResponse","paginated":false,"query":[],"headers":{"organizationId":"X-Affinity-Organization-Id"},"body":false,"practice":"none","rootOnly":true,"idempotency":"required"},"listWebhookEvents":{"id":"listWebhookEvents","group":"webhooks.events","method":"list","verb":"GET","path":"/v1/webhook-events","ids":[],"paramsName":"WebhookEventListParams","hasParams":true,"paramsRequired":false,"response":"ListWebhookEventsResponse","paginated":true,"query":["endingBefore","limit","status","startingAfter"],"headers":{"organizationId":"X-Affinity-Organization-Id"},"body":false,"practice":"none","rootOnly":true,"idempotency":"none"},"getWebhookEvent":{"id":"getWebhookEvent","group":"webhooks.events","method":"get","verb":"GET","path":"/v1/webhook-events/{eventId}","ids":["eventId"],"paramsName":"WebhookEventGetParams","hasParams":false,"paramsRequired":false,"response":"GetWebhookEventResponse","paginated":false,"query":[],"headers":{"organizationId":"X-Affinity-Organization-Id"},"body":false,"practice":"none","rootOnly":true,"idempotency":"none"},"replayWebhookEvent":{"id":"replayWebhookEvent","group":"webhooks.events","method":"replay","verb":"POST","path":"/v1/webhook-events/{eventId}/replay","ids":["eventId"],"paramsName":"WebhookEventReplayParams","hasParams":false,"paramsRequired":false,"response":"ReplayWebhookEventResponse","paginated":false,"query":[],"headers":{"organizationId":"X-Affinity-Organization-Id"},"body":false,"practice":"none","rootOnly":true,"idempotency":"required"},"getOrderTestSimulation":{"id":"getOrderTestSimulation","group":"orders.testSimulation","method":"get","verb":"GET","path":"/v1/orders/{orderId}/test-simulation","ids":["orderId"],"paramsName":"OrderTestSimulationGetParams","hasParams":false,"paramsRequired":false,"response":"GetOrderTestSimulationResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"order","rootOnly":false,"idempotency":"none"},"updateOrderTestSimulation":{"id":"updateOrderTestSimulation","group":"orders.testSimulation","method":"update","verb":"PUT","path":"/v1/orders/{orderId}/test-simulation","ids":["orderId"],"paramsName":"OrderTestSimulationUpdateParams","hasParams":true,"paramsRequired":true,"response":"UpdateOrderTestSimulationResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"order","rootOnly":false,"idempotency":"required"},"retrievePrescribingOptions":{"id":"retrievePrescribingOptions","group":"catalog.prescribingOptions","method":"get","verb":"GET","path":"/v1/catalog/items/{catalogItemId}/prescribing-options","ids":["catalogItemId"],"paramsName":"PrescribingOptionGetParams","hasParams":false,"paramsRequired":false,"response":"RetrievePrescribingOptionsResponse","paginated":false,"query":["practiceId"],"headers":{},"body":false,"practice":"query","rootOnly":false,"idempotency":"none"},"previewOrder":{"id":"previewOrder","group":"orders","method":"preview","verb":"POST","path":"/v1/order-previews","ids":[],"paramsName":"OrderPreviewParams","hasParams":true,"paramsRequired":true,"response":"PreviewOrderResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"body","rootOnly":false,"idempotency":"none"},"signOrder":{"id":"signOrder","group":"orders","method":"sign","verb":"POST","path":"/v1/orders/{orderId}/sign","ids":["orderId"],"paramsName":"OrderSignParams","hasParams":true,"paramsRequired":true,"response":"SignOrderResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"body","rootOnly":false,"idempotency":"required"},"signAndSubmitOrder":{"id":"signAndSubmitOrder","group":"orders","method":"signAndSubmit","verb":"POST","path":"/v1/orders/{orderId}/sign-and-submit","ids":["orderId"],"paramsName":"OrderSignAndSubmitParams","hasParams":true,"paramsRequired":true,"response":"SignAndSubmitOrderResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"body","rootOnly":false,"idempotency":"required"},"submitOrder":{"id":"submitOrder","group":"orders","method":"submit","verb":"POST","path":"/v1/orders/{orderId}/submit","ids":["orderId"],"paramsName":"OrderSubmitParams","hasParams":false,"paramsRequired":false,"response":"SubmitOrderResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"body","rootOnly":false,"idempotency":"required"},"rejectOrder":{"id":"rejectOrder","group":"orders","method":"reject","verb":"POST","path":"/v1/orders/{orderId}/rejection","ids":["orderId"],"paramsName":"OrderRejectParams","hasParams":true,"paramsRequired":true,"response":"RejectOrderResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"body","rootOnly":false,"idempotency":"required"},"registerUser":{"id":"registerUser","group":"team","method":"register","verb":"POST","path":"/v1/practices/{practiceId}/users","ids":[],"paramsName":"TeamRegisterParams","hasParams":true,"paramsRequired":true,"response":"RegisterUserResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"path","rootOnly":false,"idempotency":"required"},"listPatientAddresses":{"id":"listPatientAddresses","group":"patients.addresses","method":"list","verb":"GET","path":"/v1/practices/{practiceId}/patients/{patientId}/addresses","ids":["patientId"],"paramsName":"PatientAddressListParams","hasParams":true,"paramsRequired":false,"response":"ListPatientAddressesResponse","paginated":true,"query":["status","startingAfter","endingBefore","limit"],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"createPatientAddress":{"id":"createPatientAddress","group":"patients.addresses","method":"create","verb":"POST","path":"/v1/practices/{practiceId}/patients/{patientId}/addresses","ids":["patientId"],"paramsName":"PatientAddressCreateParams","hasParams":true,"paramsRequired":true,"response":"CreatePatientAddressResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"path","rootOnly":false,"idempotency":"auto"},"updatePatientAddress":{"id":"updatePatientAddress","group":"patients.addresses","method":"update","verb":"PATCH","path":"/v1/practices/{practiceId}/patients/{patientId}/addresses/{addressId}","ids":["patientId","addressId"],"paramsName":"PatientAddressUpdateParams","hasParams":true,"paramsRequired":false,"response":"UpdatePatientAddressResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"path","rootOnly":false,"idempotency":"auto"},"archivePatientAddress":{"id":"archivePatientAddress","group":"patients.addresses","method":"archive","verb":"DELETE","path":"/v1/practices/{practiceId}/patients/{patientId}/addresses/{addressId}","ids":["patientId","addressId"],"paramsName":"PatientAddressArchiveParams","hasParams":false,"paramsRequired":false,"response":"ArchivePatientAddressResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":false,"practice":"path","rootOnly":false,"idempotency":"auto"},"setDefaultPatientAddress":{"id":"setDefaultPatientAddress","group":"patients.addresses","method":"setDefault","verb":"PUT","path":"/v1/practices/{practiceId}/patients/{patientId}/addresses/{addressId}/default","ids":["patientId","addressId"],"paramsName":"PatientAddressSetDefaultParams","hasParams":false,"paramsRequired":false,"response":"SetDefaultPatientAddressResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":false,"practice":"path","rootOnly":false,"idempotency":"auto"},"invitePracticeTeamPerson":{"id":"invitePracticeTeamPerson","group":"team.invitations","method":"create","verb":"POST","path":"/v1/practices/{practiceId}/team/invitations","ids":[],"paramsName":"TeamInvitationCreateParams","hasParams":true,"paramsRequired":true,"response":"InvitePracticeTeamPersonResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"path","rootOnly":false,"idempotency":"required"},"listPracticeTeamInvitations":{"id":"listPracticeTeamInvitations","group":"team.invitations","method":"list","verb":"GET","path":"/v1/practices/{practiceId}/team/invitations","ids":[],"paramsName":"TeamInvitationListParams","hasParams":true,"paramsRequired":false,"response":"ListPracticeTeamInvitationsResponse","paginated":true,"query":["limit","startingAfter","endingBefore","status","email","externalId"],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"getPracticeTeam":{"id":"getPracticeTeam","group":"team","method":"get","verb":"GET","path":"/v1/practices/{practiceId}/team","ids":[],"paramsName":"TeamGetParams","hasParams":false,"paramsRequired":false,"response":"GetPracticeTeamResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"listPracticeTeamMembers":{"id":"listPracticeTeamMembers","group":"team.members","method":"list","verb":"GET","path":"/v1/practices/{practiceId}/team/members","ids":[],"paramsName":"TeamMemberListParams","hasParams":true,"paramsRequired":false,"response":"ListPracticeTeamMembersResponse","paginated":true,"query":["limit","startingAfter","endingBefore","search","role","status"],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"listPracticeTeamPrescribers":{"id":"listPracticeTeamPrescribers","group":"team.prescribers","method":"list","verb":"GET","path":"/v1/practices/{practiceId}/team/prescribers","ids":[],"paramsName":"TeamPrescriberListParams","hasParams":true,"paramsRequired":false,"response":"ListPracticeTeamPrescribersResponse","paginated":true,"query":["limit","startingAfter","endingBefore","search","npi","state","status"],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"getPracticeTeamMember":{"id":"getPracticeTeamMember","group":"team.members","method":"get","verb":"GET","path":"/v1/practices/{practiceId}/team/members/{memberId}","ids":["memberId"],"paramsName":"TeamMemberGetParams","hasParams":false,"paramsRequired":false,"response":"GetPracticeTeamMemberResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"updatePracticeTeamMember":{"id":"updatePracticeTeamMember","group":"team.members","method":"update","verb":"PATCH","path":"/v1/practices/{practiceId}/team/members/{memberId}","ids":["memberId"],"paramsName":"TeamMemberUpdateParams","hasParams":true,"paramsRequired":false,"response":"UpdatePracticeTeamMemberResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"path","rootOnly":false,"idempotency":"auto"},"getPracticeTeamPrescriber":{"id":"getPracticeTeamPrescriber","group":"team.prescribers","method":"get","verb":"GET","path":"/v1/practices/{practiceId}/team/prescribers/{prescriberId}","ids":["prescriberId"],"paramsName":"TeamPrescriberGetParams","hasParams":false,"paramsRequired":false,"response":"GetPracticeTeamPrescriberResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"updatePracticeTeamPrescriber":{"id":"updatePracticeTeamPrescriber","group":"team.prescribers","method":"update","verb":"PATCH","path":"/v1/practices/{practiceId}/team/prescribers/{prescriberId}","ids":["prescriberId"],"paramsName":"TeamPrescriberUpdateParams","hasParams":true,"paramsRequired":false,"response":"UpdatePracticeTeamPrescriberResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"path","rootOnly":false,"idempotency":"auto"},"createPracticeTeamLicense":{"id":"createPracticeTeamLicense","group":"team.prescribers.licenses","method":"create","verb":"POST","path":"/v1/practices/{practiceId}/team/prescribers/{prescriberId}/licenses","ids":["prescriberId"],"paramsName":"TeamPrescriberLicenseCreateParams","hasParams":true,"paramsRequired":true,"response":"CreatePracticeTeamLicenseResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"path","rootOnly":false,"idempotency":"auto"},"updatePracticeTeamLicense":{"id":"updatePracticeTeamLicense","group":"team.prescribers.licenses","method":"update","verb":"PATCH","path":"/v1/practices/{practiceId}/team/prescribers/{prescriberId}/licenses/{licenseId}","ids":["prescriberId","licenseId"],"paramsName":"TeamPrescriberLicenseUpdateParams","hasParams":true,"paramsRequired":false,"response":"UpdatePracticeTeamLicenseResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"path","rootOnly":false,"idempotency":"auto"},"getPracticeTeamInvitation":{"id":"getPracticeTeamInvitation","group":"team.invitations","method":"get","verb":"GET","path":"/v1/practices/{practiceId}/team/invitations/{invitationId}","ids":["invitationId"],"paramsName":"TeamInvitationGetParams","hasParams":false,"paramsRequired":false,"response":"GetPracticeTeamInvitationResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"revokePracticeTeamInvitation":{"id":"revokePracticeTeamInvitation","group":"team.invitations","method":"revoke","verb":"DELETE","path":"/v1/practices/{practiceId}/team/invitations/{invitationId}","ids":["invitationId"],"paramsName":"TeamInvitationRevokeParams","hasParams":false,"paramsRequired":false,"response":"RevokePracticeTeamInvitationResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"required"},"resendPracticeTeamInvitation":{"id":"resendPracticeTeamInvitation","group":"team.invitations","method":"resend","verb":"POST","path":"/v1/practices/{practiceId}/team/invitations/{invitationId}/resend","ids":["invitationId"],"paramsName":"TeamInvitationResendParams","hasParams":false,"paramsRequired":false,"response":"ResendPracticeTeamInvitationResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"path","rootOnly":false,"idempotency":"required"},"getApiAccess":{"id":"getApiAccess","group":"apiKeys","method":"getAccess","verb":"GET","path":"/v1/auth/access","ids":[],"paramsName":"ApiKeyGetAccessParams","hasParams":false,"paramsRequired":false,"response":"GetApiAccessResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"none","rootOnly":false,"idempotency":"none"},"listPractices":{"id":"listPractices","group":"practices","method":"list","verb":"GET","path":"/v1/practices","ids":[],"paramsName":"PracticeListParams","hasParams":true,"paramsRequired":false,"response":"ListPracticesResponse","paginated":true,"query":["search","endingBefore","limit","startingAfter"],"headers":{},"body":false,"practice":"none","rootOnly":true,"idempotency":"none"},"createPractice":{"id":"createPractice","group":"practices","method":"create","verb":"POST","path":"/v1/practices","ids":[],"paramsName":"PracticeCreateParams","hasParams":true,"paramsRequired":true,"response":"CreatePracticeResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"none","rootOnly":true,"idempotency":"auto"},"getPractice":{"id":"getPractice","group":"practices","method":"get","verb":"GET","path":"/v1/practices/{practiceId}","ids":["practiceId"],"paramsName":"PracticeGetParams","hasParams":false,"paramsRequired":false,"response":"GetPracticeResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"none","rootOnly":true,"idempotency":"none"},"updatePractice":{"id":"updatePractice","group":"practices","method":"update","verb":"PATCH","path":"/v1/practices/{practiceId}","ids":["practiceId"],"paramsName":"PracticeUpdateParams","hasParams":true,"paramsRequired":false,"response":"UpdatePracticeResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"none","rootOnly":true,"idempotency":"auto"},"listPatients":{"id":"listPatients","group":"patients","method":"list","verb":"GET","path":"/v1/practices/{practiceId}/patients","ids":[],"paramsName":"PatientListParams","hasParams":true,"paramsRequired":false,"response":"ListPatientsResponse","paginated":true,"query":["endingBefore","externalId","externalIdentitySource","externalIdentityValue","gender","lastOrderAfter","lastOrderBefore","limit","program","query","sort","startingAfter","states","status"],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"createPatient":{"id":"createPatient","group":"patients","method":"create","verb":"POST","path":"/v1/practices/{practiceId}/patients","ids":[],"paramsName":"PatientCreateParams","hasParams":true,"paramsRequired":true,"response":"CreatePatientResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"path","rootOnly":false,"idempotency":"auto"},"getPatient":{"id":"getPatient","group":"patients","method":"get","verb":"GET","path":"/v1/practices/{practiceId}/patients/{patientId}","ids":["patientId"],"paramsName":"PatientGetParams","hasParams":false,"paramsRequired":false,"response":"GetPatientResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"deletePatient":{"id":"deletePatient","group":"patients","method":"delete","verb":"DELETE","path":"/v1/practices/{practiceId}/patients/{patientId}","ids":["patientId"],"paramsName":"PatientDeleteParams","hasParams":false,"paramsRequired":false,"response":"DeletePatientResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":false,"practice":"path","rootOnly":false,"idempotency":"auto"},"updatePatient":{"id":"updatePatient","group":"patients","method":"update","verb":"PATCH","path":"/v1/practices/{practiceId}/patients/{patientId}","ids":["patientId"],"paramsName":"PatientUpdateParams","hasParams":true,"paramsRequired":false,"response":"UpdatePatientResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"path","rootOnly":false,"idempotency":"auto"},"getPatientAllergies":{"id":"getPatientAllergies","group":"patients.allergies","method":"get","verb":"GET","path":"/v1/practices/{practiceId}/patients/{patientId}/allergies","ids":["patientId"],"paramsName":"PatientAllergyGetParams","hasParams":false,"paramsRequired":false,"response":"GetPatientAllergiesResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":false,"practice":"path","rootOnly":false,"idempotency":"none"},"replacePatientAllergies":{"id":"replacePatientAllergies","group":"patients.allergies","method":"replace","verb":"PUT","path":"/v1/practices/{practiceId}/patients/{patientId}/allergies","ids":["patientId"],"paramsName":"PatientAllergyReplaceParams","hasParams":true,"paramsRequired":true,"response":"ReplacePatientAllergiesResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"path","rootOnly":false,"idempotency":"required"},"addOrderPrescription":{"id":"addOrderPrescription","group":"orders.prescriptions","method":"add","verb":"POST","path":"/v1/orders/{orderId}/prescriptions","ids":["orderId"],"paramsName":"OrderPrescriptionAddParams","hasParams":true,"paramsRequired":true,"response":"AddOrderPrescriptionResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"body","rootOnly":false,"idempotency":"required"},"updateOrderPrescription":{"id":"updateOrderPrescription","group":"orders.prescriptions","method":"update","verb":"PATCH","path":"/v1/orders/{orderId}/prescriptions/{prescriptionId}","ids":["orderId","prescriptionId"],"paramsName":"OrderPrescriptionUpdateParams","hasParams":true,"paramsRequired":true,"response":"UpdateOrderPrescriptionResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"body","rootOnly":false,"idempotency":"required"},"createOrderBatch":{"id":"createOrderBatch","group":"orders.batches","method":"create","verb":"POST","path":"/v1/order-batches","ids":[],"paramsName":"OrderBatchCreateParams","hasParams":true,"paramsRequired":true,"response":"CreateOrderBatchResponse","paginated":false,"query":[],"headers":{"actorId":"Affinity-Actor-Id","actorType":"Affinity-Actor-Type"},"body":true,"practice":"body","rootOnly":false,"idempotency":"required"},"platform.public-api.selling-prices.readSellingPrice":{"id":"platform.public-api.selling-prices.readSellingPrice","group":"catalog.sellingPrices","method":"get","verb":"GET","path":"/v1/catalog/items/{catalogItemId}/selling-price","ids":["catalogItemId"],"paramsName":"SellingPriceGetParams","hasParams":false,"paramsRequired":false,"response":"PlatformPublicApiSellingPricesReadSellingPriceResponse","paginated":false,"query":["practiceId"],"headers":{},"body":false,"practice":"query","rootOnly":false,"idempotency":"none"},"platform.public-api.selling-prices.updateSellingPrice":{"id":"platform.public-api.selling-prices.updateSellingPrice","group":"catalog.sellingPrices","method":"update","verb":"PUT","path":"/v1/catalog/items/{catalogItemId}/selling-price","ids":["catalogItemId"],"paramsName":"SellingPriceUpdateParams","hasParams":true,"paramsRequired":true,"response":"PlatformPublicApiSellingPricesUpdateSellingPriceResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"body","rootOnly":false,"idempotency":"required"},"listWebhookGrants":{"id":"listWebhookGrants","group":"webhooks.grants","method":"list","verb":"GET","path":"/v1/webhook-grants","ids":[],"paramsName":"WebhookGrantListParams","hasParams":true,"paramsRequired":false,"response":"ListWebhookGrantsResponse","paginated":true,"query":["limit","startingAfter","endingBefore"],"headers":{},"body":false,"practice":"none","rootOnly":true,"idempotency":"none"},"saveWebhookGrant":{"id":"saveWebhookGrant","group":"webhooks.grants","method":"save","verb":"PUT","path":"/v1/webhook-grants/{platformId}","ids":["platformId"],"paramsName":"WebhookGrantSaveParams","hasParams":true,"paramsRequired":true,"response":"SaveWebhookGrantResponse","paginated":false,"query":[],"headers":{},"body":true,"practice":"none","rootOnly":true,"idempotency":"required"},"revokeWebhookGrant":{"id":"revokeWebhookGrant","group":"webhooks.grants","method":"revoke","verb":"DELETE","path":"/v1/webhook-grants/{platformId}","ids":["platformId"],"paramsName":"WebhookGrantRevokeParams","hasParams":false,"paramsRequired":false,"response":"RevokeWebhookGrantResponse","paginated":false,"query":[],"headers":{},"body":false,"practice":"none","rootOnly":true,"idempotency":"required"}}"###;
pub mod models {use super::*;
#[derive(Debug,Clone,Default,Serialize)]
pub struct LocationListParams{
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl LocationListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["startingAfter","endingBefore","status"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct LocationCreateParams{
#[serde(rename="city",skip_serializing_if="Option::is_none")]
pub city:Option<String>,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1",skip_serializing_if="Option::is_none")]
pub line1:Option<String>,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="name")]
pub name:String,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(rename="postalCode",skip_serializing_if="Option::is_none")]
pub postal_code:Option<String>,
#[serde(rename="state",skip_serializing_if="Option::is_none")]
pub state:Option<String>,
#[serde(rename="timezone",skip_serializing_if="Option::is_none")]
pub timezone:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl LocationCreateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["city","country","line1","line2","phone","postalCode","state","timezone"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct LocationUpdateParams{
#[serde(rename="city",skip_serializing_if="Option::is_none")]
pub city:Option<String>,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1",skip_serializing_if="Option::is_none")]
pub line1:Option<String>,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="name",skip_serializing_if="Option::is_none")]
pub name:Option<String>,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(rename="postalCode",skip_serializing_if="Option::is_none")]
pub postal_code:Option<String>,
#[serde(rename="state",skip_serializing_if="Option::is_none")]
pub state:Option<String>,
#[serde(rename="timezone",skip_serializing_if="Option::is_none")]
pub timezone:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl LocationUpdateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["city","country","line1","line2","name","phone","postalCode","state","timezone"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct ApiKeyCreateParams{
#[serde(rename="allowedIps",skip_serializing_if="Option::is_none")]
pub allowed_ips:Option<Vec<serde_json::Value>>,
#[serde(rename="expiresAt",skip_serializing_if="Option::is_none")]
pub expires_at:Option<String>,
#[serde(rename="name")]
pub name:String,
#[serde(rename="scopes",skip_serializing_if="Option::is_none")]
pub scopes:Option<Vec<String>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl ApiKeyCreateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["allowedIps","expiresAt","scopes"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct AccountGetParams{
#[serde(rename="orgId",skip_serializing_if="Option::is_none")]
pub org_id:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl AccountGetParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["orgId"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct CatalogItemListParams{
#[serde(rename="view",skip_serializing_if="Option::is_none")]
pub view:Option<String>,
#[serde(rename="relatedToCatalogItemId",skip_serializing_if="Option::is_none")]
pub related_to_catalog_item_id:Option<String>,
#[serde(rename="catalogKind",skip_serializing_if="Option::is_none")]
pub catalog_kind:Option<String>,
#[serde(rename="sort",skip_serializing_if="Option::is_none")]
pub sort:Option<String>,
#[serde(rename="catalogItemId",skip_serializing_if="Option::is_none")]
pub catalog_item_id:Option<String>,
#[serde(rename="availability",skip_serializing_if="Option::is_none")]
pub availability:Option<String>,
#[serde(rename="pharmacyIds",skip_serializing_if="Option::is_none")]
pub pharmacy_ids:Option<serde_json::Value>,
#[serde(rename="dosageForms",skip_serializing_if="Option::is_none")]
pub dosage_forms:Option<serde_json::Value>,
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="hideControlledSubstances",skip_serializing_if="Option::is_none")]
pub hide_controlled_substances:Option<bool>,
#[serde(rename="hideUnpriced",skip_serializing_if="Option::is_none")]
pub hide_unpriced:Option<bool>,
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="orgId",skip_serializing_if="Option::is_none")]
pub org_id:Option<String>,
#[serde(rename="query",skip_serializing_if="Option::is_none")]
pub query:Option<String>,
#[serde(rename="requirement",skip_serializing_if="Option::is_none")]
pub requirement:Option<String>,
#[serde(rename="routes",skip_serializing_if="Option::is_none")]
pub routes:Option<serde_json::Value>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl CatalogItemListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["view","relatedToCatalogItemId","catalogKind","sort","catalogItemId","availability","pharmacyIds","dosageForms","endingBefore","orgId","query","requirement","routes","startingAfter"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PharmacyListParams{
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="orgId",skip_serializing_if="Option::is_none")]
pub org_id:Option<String>,
#[serde(rename="pharmacyId",skip_serializing_if="Option::is_none")]
pub pharmacy_id:Option<String>,
#[serde(rename="query",skip_serializing_if="Option::is_none")]
pub query:Option<String>,
#[serde(rename="shipsToState",skip_serializing_if="Option::is_none")]
pub ships_to_state:Option<String>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PharmacyListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["endingBefore","orgId","pharmacyId","query","shipsToState","startingAfter"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct ShippingOptionListParams{
#[serde(rename="destinationState")]
pub destination_state:String,
#[serde(rename="destinationType",skip_serializing_if="Option::is_none")]
pub destination_type:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl ShippingOptionListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["destinationType"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderListParams{
#[serde(rename="query",skip_serializing_if="Option::is_none")]
pub query:Option<String>,
#[serde(rename="externalOrderId",skip_serializing_if="Option::is_none")]
pub external_order_id:Option<String>,
#[serde(rename="createdAfter",skip_serializing_if="Option::is_none")]
pub created_after:Option<String>,
#[serde(rename="createdBefore",skip_serializing_if="Option::is_none")]
pub created_before:Option<String>,
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="orderId",skip_serializing_if="Option::is_none")]
pub order_id:Option<String>,
#[serde(rename="patientId",skip_serializing_if="Option::is_none")]
pub patient_id:Option<String>,
#[serde(rename="patientExternalId",skip_serializing_if="Option::is_none")]
pub patient_external_id:Option<String>,
#[serde(rename="sort",skip_serializing_if="Option::is_none")]
pub sort:Option<String>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["query","externalOrderId","createdAfter","createdBefore","endingBefore","orderId","patientId","patientExternalId","sort","startingAfter","status"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParams{
#[serde(rename="userId",skip_serializing_if="Option::is_none")]
pub user_id:Option<String>,
#[serde(rename="prescriber")]
pub prescriber:PrescriberSelector,
#[serde(rename="otcItems",skip_serializing_if="Option::is_none")]
pub otc_items:Option<Vec<OrderCreateParamsOtcItemsItem>>,
#[serde(rename="externalOrderId",skip_serializing_if="Option::is_none")]
pub external_order_id:Option<String>,
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<std::collections::HashMap<String,serde_json::Value>>,
#[serde(rename="patientId")]
pub patient_id:String,
#[serde(rename="patient",skip_serializing_if="Option::is_none")]
pub patient:Option<OrderCreateParamsPatient>,
#[serde(rename="shippingAddressId",skip_serializing_if="Option::is_none")]
pub shipping_address_id:Option<String>,
#[serde(rename="prescriptions")]
pub prescriptions:Vec<OrderCreateParamsPrescriptionsItem>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["userId","prescriber","otcItems","externalOrderId","metadata","patientId","patient","shippingAddressId"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PrescriberSelector{
#[serde(rename="id",skip_serializing_if="Option::is_none")]
pub id:Option<String>,
#[serde(rename="npi",skip_serializing_if="Option::is_none")]
pub npi:Option<String>,
#[serde(rename="externalId",skip_serializing_if="Option::is_none")]
pub external_id:Option<String>,
#[serde(rename="profile",skip_serializing_if="Option::is_none")]
pub profile:Option<PrescriberSelectorProfile>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PrescriberSelector{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["id","npi","externalId","profile"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
impl PrescriberSelector {pub fn id(id:impl Into<String>)->Self{Self{id:Some(id.into()),..Default::default()}}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PrescriberSelectorProfile{
#[serde(rename="email",skip_serializing_if="Option::is_none")]
pub email:Option<String>,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PrescriberSelectorProfile{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["email","phone"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsOtcItemsItem{
#[serde(rename="catalogItemId")]
pub catalog_item_id:String,
#[serde(rename="quantity")]
pub quantity:i64,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPatient{
#[serde(rename="address",skip_serializing_if="Option::is_none")]
pub address:Option<OrderCreateParamsPatientAddress>,
#[serde(rename="clinicalProfile",skip_serializing_if="Option::is_none")]
pub clinical_profile:Option<OrderCreateParamsPatientClinicalProfile>,
#[serde(rename="dateOfBirth")]
pub date_of_birth:String,
#[serde(rename="email",skip_serializing_if="Option::is_none")]
pub email:Option<String>,
#[serde(rename="externalId",skip_serializing_if="Option::is_none")]
pub external_id:Option<String>,
#[serde(rename="externalIdentities",skip_serializing_if="Option::is_none")]
pub external_identities:Option<Vec<OrderCreateParamsPatientExternalIdentitiesItem>>,
#[serde(rename="addresses",skip_serializing_if="Option::is_none")]
pub addresses:Option<Vec<OrderCreateParamsPatientAddressesItem>>,
#[serde(rename="encounters",skip_serializing_if="Option::is_none")]
pub encounters:Option<Vec<OrderCreateParamsPatientEncountersItem>>,
#[serde(rename="gender",skip_serializing_if="Option::is_none")]
pub gender:Option<String>,
#[serde(rename="locationId",skip_serializing_if="Option::is_none")]
pub location_id:Option<String>,
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<serde_json::Value>,
#[serde(rename="medicalRecordNumber",skip_serializing_if="Option::is_none")]
pub medical_record_number:Option<String>,
#[serde(rename="measurements",skip_serializing_if="Option::is_none")]
pub measurements:Option<Vec<OrderCreateParamsPatientMeasurementsItem>>,
#[serde(rename="name")]
pub name:OrderCreateParamsPatientName,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(rename="programs",skip_serializing_if="Option::is_none")]
pub programs:Option<Vec<OrderCreateParamsPatientProgramsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPatient{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["address","clinicalProfile","email","externalId","externalIdentities","addresses","encounters","gender","locationId","metadata","medicalRecordNumber","measurements","phone","programs"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPatientAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPatientAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["line2","country"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPatientClinicalProfile{
#[serde(rename="currentMedications")]
pub current_medications:Vec<String>,
#[serde(rename="heightInches",skip_serializing_if="Option::is_none")]
pub height_inches:Option<serde_json::Value>,
#[serde(rename="reviewedAt",skip_serializing_if="Option::is_none")]
pub reviewed_at:Option<String>,
#[serde(rename="weightPounds",skip_serializing_if="Option::is_none")]
pub weight_pounds:Option<serde_json::Value>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPatientClinicalProfile{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["heightInches","reviewedAt","weightPounds"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPatientExternalIdentitiesItem{
#[serde(rename="source")]
pub source:String,
#[serde(rename="value")]
pub value:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPatientAddressesItem{
#[serde(rename="id",skip_serializing_if="Option::is_none")]
pub id:Option<String>,
#[serde(rename="address")]
pub address:OrderCreateParamsPatientAddressesItemAddress,
#[serde(rename="label")]
pub label:String,
#[serde(rename="preferredShipping")]
pub preferred_shipping:bool,
#[serde(rename="recipientName")]
pub recipient_name:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPatientAddressesItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["id","recipientName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPatientAddressesItemAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPatientAddressesItemAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["country","line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPatientEncountersItem{
#[serde(rename="notes")]
pub notes:String,
#[serde(rename="occurredAt")]
pub occurred_at:String,
#[serde(rename="providerName")]
pub provider_name:String,
#[serde(rename="type")]
pub r#type:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPatientEncountersItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["notes","providerName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPatientMeasurementsItem{
#[serde(rename="heightCentimeters")]
pub height_centimeters:serde_json::Value,
#[serde(rename="recordedAt")]
pub recorded_at:String,
#[serde(rename="source")]
pub source:String,
#[serde(rename="weightKilograms")]
pub weight_kilograms:serde_json::Value,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPatientMeasurementsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["heightCentimeters","weightKilograms"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPatientName{
#[serde(rename="first")]
pub first:String,
#[serde(rename="last")]
pub last:String,
#[serde(rename="middle",skip_serializing_if="Option::is_none")]
pub middle:Option<String>,
#[serde(rename="preferred",skip_serializing_if="Option::is_none")]
pub preferred:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPatientName{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["middle","preferred"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPatientProgramsItem{
#[serde(rename="endedAt")]
pub ended_at:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="startedAt")]
pub started_at:String,
#[serde(rename="status")]
pub status:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPatientProgramsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["endedAt"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPrescriptionsItem{
#[serde(rename="externalPrescriptionId",skip_serializing_if="Option::is_none")]
pub external_prescription_id:Option<String>,
#[serde(rename="clinical",skip_serializing_if="Option::is_none")]
pub clinical:Option<OrderCreateParamsPrescriptionsItemClinical>,
#[serde(rename="pharmacyId",skip_serializing_if="Option::is_none")]
pub pharmacy_id:Option<String>,
#[serde(rename="daysSupply")]
pub days_supply:i64,
#[serde(rename="dispensing")]
pub dispensing:OrderCreateParamsPrescriptionsItemDispensing,
#[serde(rename="directions")]
pub directions:String,
#[serde(rename="medicationId")]
pub medication_id:String,
#[serde(rename="quantity")]
pub quantity:serde_json::Value,
#[serde(rename="quantityUnit")]
pub quantity_unit:String,
#[serde(rename="refills")]
pub refills:i64,
#[serde(rename="structuredSig",skip_serializing_if="Option::is_none")]
pub structured_sig:Option<OrderCreateParamsPrescriptionsItemStructuredSig>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPrescriptionsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["externalPrescriptionId","clinical","pharmacyId","structuredSig"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPrescriptionsItemClinical{
#[serde(rename="compoundingReason",skip_serializing_if="Option::is_none")]
pub compounding_reason:Option<OrderCreateParamsPrescriptionsItemClinicalCompoundingReason>,
#[serde(rename="medicationReviewStatus",skip_serializing_if="Option::is_none")]
pub medication_review_status:Option<String>,
#[serde(rename="diagnosisReviewStatus",skip_serializing_if="Option::is_none")]
pub diagnosis_review_status:Option<String>,
#[serde(rename="currentMedications",skip_serializing_if="Option::is_none")]
pub current_medications:Option<Vec<String>>,
#[serde(rename="diagnoses",skip_serializing_if="Option::is_none")]
pub diagnoses:Option<Vec<OrderCreateParamsPrescriptionsItemClinicalDiagnosesItem>>,
#[serde(rename="observations",skip_serializing_if="Option::is_none")]
pub observations:Option<Vec<OrderCreateParamsPrescriptionsItemClinicalObservationsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPrescriptionsItemClinical{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["compoundingReason","medicationReviewStatus","diagnosisReviewStatus","currentMedications","diagnoses","observations"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPrescriptionsItemClinicalCompoundingReason{
#[serde(rename="category",skip_serializing_if="Option::is_none")]
pub category:Option<String>,
#[serde(rename="context",skip_serializing_if="Option::is_none")]
pub context:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPrescriptionsItemClinicalCompoundingReason{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["category","context"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPrescriptionsItemClinicalDiagnosesItem{
#[serde(rename="code")]
pub code:String,
#[serde(rename="display")]
pub display:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPrescriptionsItemClinicalObservationsItem{
#[serde(rename="display")]
pub display:String,
#[serde(rename="unit")]
pub unit:String,
#[serde(rename="value")]
pub value:serde_json::Value,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPrescriptionsItemDispensing{
#[serde(rename="dispenseUponAcceptance",skip_serializing_if="Option::is_none")]
pub dispense_upon_acceptance:Option<bool>,
#[serde(rename="shippingOptionId",skip_serializing_if="Option::is_none")]
pub shipping_option_id:Option<String>,
#[serde(rename="shippingAmountCents",skip_serializing_if="Option::is_none")]
pub shipping_amount_cents:Option<i64>,
#[serde(rename="shippingDestinationType",skip_serializing_if="Option::is_none")]
pub shipping_destination_type:Option<String>,
#[serde(rename="pharmacyNotes",skip_serializing_if="Option::is_none")]
pub pharmacy_notes:Option<String>,
#[serde(rename="requestedFillDate",skip_serializing_if="Option::is_none")]
pub requested_fill_date:Option<String>,
#[serde(rename="substitutionPermitted",skip_serializing_if="Option::is_none")]
pub substitution_permitted:Option<bool>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPrescriptionsItemDispensing{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["dispenseUponAcceptance","shippingOptionId","shippingAmountCents","shippingDestinationType","pharmacyNotes","requestedFillDate","substitutionPermitted"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCreateParamsPrescriptionsItemStructuredSig{
#[serde(rename="dose")]
pub dose:String,
#[serde(rename="doseUnit")]
pub dose_unit:String,
#[serde(rename="duration",skip_serializing_if="Option::is_none")]
pub duration:Option<String>,
#[serde(rename="frequency")]
pub frequency:String,
#[serde(rename="indication",skip_serializing_if="Option::is_none")]
pub indication:Option<String>,
#[serde(rename="maxDailyUse",skip_serializing_if="Option::is_none")]
pub max_daily_use:Option<String>,
#[serde(rename="prn",skip_serializing_if="Option::is_none")]
pub prn:Option<bool>,
#[serde(rename="route")]
pub route:String,
#[serde(rename="titrationSchedule",skip_serializing_if="Option::is_none")]
pub titration_schedule:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderCreateParamsPrescriptionsItemStructuredSig{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["duration","indication","maxDailyUse","prn","titrationSchedule"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderCancelParams{
#[serde(rename="reason")]
pub reason:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderExceptionActParams{
#[serde(rename="action")]
pub action:String,
#[serde(rename="note",skip_serializing_if="Option::is_none")]
pub note:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderExceptionActParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["note"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderEventListParams{
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderEventListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["endingBefore","startingAfter"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct WebhookEndpointListParams{
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct WebhookEndpointCreateParams{
#[serde(rename="practiceIds",skip_serializing_if="Option::is_none")]
pub practice_ids:Option<Vec<String>>,
#[serde(rename="description",skip_serializing_if="Option::is_none")]
pub description:Option<String>,
#[serde(rename="payloadStyle",skip_serializing_if="Option::is_none")]
pub payload_style:Option<String>,
#[serde(rename="subscribedEvents",skip_serializing_if="Option::is_none")]
pub subscribed_events:Option<Vec<String>>,
#[serde(rename="url")]
pub url:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct WebhookEndpointUpdateParams{
#[serde(rename="practiceIds",skip_serializing_if="Option::is_none")]
pub practice_ids:Option<Vec<String>>,
#[serde(rename="description",skip_serializing_if="Option::is_none")]
pub description:Option<String>,
#[serde(rename="payloadStyle",skip_serializing_if="Option::is_none")]
pub payload_style:Option<String>,
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(rename="subscribedEvents",skip_serializing_if="Option::is_none")]
pub subscribed_events:Option<Vec<String>>,
#[serde(rename="url",skip_serializing_if="Option::is_none")]
pub url:Option<String>,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct WebhookEventListParams{
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl WebhookEventListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["endingBefore","status","startingAfter"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderTestSimulationUpdateParams{
#[serde(rename="mode")]
pub mode:String,
#[serde(rename="scenario")]
pub scenario:String,
#[serde(rename="action",skip_serializing_if="Option::is_none")]
pub action:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderTestSimulationUpdateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["action"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParams{
#[serde(rename="otcItems",skip_serializing_if="Option::is_none")]
pub otc_items:Option<Vec<OrderPreviewParamsOtcItemsItem>>,
#[serde(rename="patientId",skip_serializing_if="Option::is_none")]
pub patient_id:Option<String>,
#[serde(rename="patientExternalId",skip_serializing_if="Option::is_none")]
pub patient_external_id:Option<String>,
#[serde(rename="patient",skip_serializing_if="Option::is_none")]
pub patient:Option<OrderPreviewParamsPatient>,
#[serde(rename="userId",skip_serializing_if="Option::is_none")]
pub user_id:Option<String>,
#[serde(rename="prescriber",skip_serializing_if="Option::is_none")]
pub prescriber:Option<PrescriberSelector>,
#[serde(rename="shippingAddressId",skip_serializing_if="Option::is_none")]
pub shipping_address_id:Option<String>,
#[serde(rename="externalOrderId",skip_serializing_if="Option::is_none")]
pub external_order_id:Option<String>,
#[serde(rename="prescriptions")]
pub prescriptions:Vec<OrderPreviewParamsPrescriptionsItem>,
#[serde(rename="shipping",skip_serializing_if="Option::is_none")]
pub shipping:Option<OrderPreviewParamsShipping>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["otcItems","patientId","patientExternalId","patient","userId","prescriber","shippingAddressId","externalOrderId","shipping"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsOtcItemsItem{
#[serde(rename="catalogItemId")]
pub catalog_item_id:String,
#[serde(rename="quantity")]
pub quantity:i64,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPatient{
#[serde(rename="address",skip_serializing_if="Option::is_none")]
pub address:Option<OrderPreviewParamsPatientAddress>,
#[serde(rename="clinicalProfile",skip_serializing_if="Option::is_none")]
pub clinical_profile:Option<OrderPreviewParamsPatientClinicalProfile>,
#[serde(rename="dateOfBirth")]
pub date_of_birth:String,
#[serde(rename="email",skip_serializing_if="Option::is_none")]
pub email:Option<String>,
#[serde(rename="externalId",skip_serializing_if="Option::is_none")]
pub external_id:Option<String>,
#[serde(rename="externalIdentities",skip_serializing_if="Option::is_none")]
pub external_identities:Option<Vec<OrderPreviewParamsPatientExternalIdentitiesItem>>,
#[serde(rename="addresses",skip_serializing_if="Option::is_none")]
pub addresses:Option<Vec<OrderPreviewParamsPatientAddressesItem>>,
#[serde(rename="encounters",skip_serializing_if="Option::is_none")]
pub encounters:Option<Vec<OrderPreviewParamsPatientEncountersItem>>,
#[serde(rename="gender",skip_serializing_if="Option::is_none")]
pub gender:Option<String>,
#[serde(rename="locationId",skip_serializing_if="Option::is_none")]
pub location_id:Option<String>,
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<serde_json::Value>,
#[serde(rename="medicalRecordNumber",skip_serializing_if="Option::is_none")]
pub medical_record_number:Option<String>,
#[serde(rename="measurements",skip_serializing_if="Option::is_none")]
pub measurements:Option<Vec<OrderPreviewParamsPatientMeasurementsItem>>,
#[serde(rename="name")]
pub name:OrderPreviewParamsPatientName,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(rename="programs",skip_serializing_if="Option::is_none")]
pub programs:Option<Vec<OrderPreviewParamsPatientProgramsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPatient{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["address","clinicalProfile","email","externalId","externalIdentities","addresses","encounters","gender","locationId","metadata","medicalRecordNumber","measurements","phone","programs"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPatientAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPatientAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["line2","country"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPatientClinicalProfile{
#[serde(rename="currentMedications")]
pub current_medications:Vec<String>,
#[serde(rename="heightInches",skip_serializing_if="Option::is_none")]
pub height_inches:Option<serde_json::Value>,
#[serde(rename="reviewedAt",skip_serializing_if="Option::is_none")]
pub reviewed_at:Option<String>,
#[serde(rename="weightPounds",skip_serializing_if="Option::is_none")]
pub weight_pounds:Option<serde_json::Value>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPatientClinicalProfile{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["heightInches","reviewedAt","weightPounds"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPatientExternalIdentitiesItem{
#[serde(rename="source")]
pub source:String,
#[serde(rename="value")]
pub value:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPatientAddressesItem{
#[serde(rename="id",skip_serializing_if="Option::is_none")]
pub id:Option<String>,
#[serde(rename="address")]
pub address:OrderPreviewParamsPatientAddressesItemAddress,
#[serde(rename="label")]
pub label:String,
#[serde(rename="preferredShipping")]
pub preferred_shipping:bool,
#[serde(rename="recipientName")]
pub recipient_name:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPatientAddressesItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["id","recipientName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPatientAddressesItemAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPatientAddressesItemAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["country","line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPatientEncountersItem{
#[serde(rename="notes")]
pub notes:String,
#[serde(rename="occurredAt")]
pub occurred_at:String,
#[serde(rename="providerName")]
pub provider_name:String,
#[serde(rename="type")]
pub r#type:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPatientEncountersItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["notes","providerName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPatientMeasurementsItem{
#[serde(rename="heightCentimeters")]
pub height_centimeters:serde_json::Value,
#[serde(rename="recordedAt")]
pub recorded_at:String,
#[serde(rename="source")]
pub source:String,
#[serde(rename="weightKilograms")]
pub weight_kilograms:serde_json::Value,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPatientMeasurementsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["heightCentimeters","weightKilograms"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPatientName{
#[serde(rename="first")]
pub first:String,
#[serde(rename="last")]
pub last:String,
#[serde(rename="middle",skip_serializing_if="Option::is_none")]
pub middle:Option<String>,
#[serde(rename="preferred",skip_serializing_if="Option::is_none")]
pub preferred:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPatientName{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["middle","preferred"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPatientProgramsItem{
#[serde(rename="endedAt")]
pub ended_at:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="startedAt")]
pub started_at:String,
#[serde(rename="status")]
pub status:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPatientProgramsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["endedAt"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPrescriptionsItem{
#[serde(rename="medicationId")]
pub medication_id:String,
#[serde(rename="externalPrescriptionId",skip_serializing_if="Option::is_none")]
pub external_prescription_id:Option<String>,
#[serde(rename="preset",skip_serializing_if="Option::is_none")]
pub preset:Option<String>,
#[serde(rename="expectedRevision",skip_serializing_if="Option::is_none")]
pub expected_revision:Option<String>,
#[serde(rename="overrides",skip_serializing_if="Option::is_none")]
pub overrides:Option<OrderPreviewParamsPrescriptionsItemOverrides>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPrescriptionsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["externalPrescriptionId","preset","expectedRevision","overrides"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPrescriptionsItemOverrides{
#[serde(rename="sig",skip_serializing_if="Option::is_none")]
pub sig:Option<serde_json::Value>,
#[serde(rename="quantity",skip_serializing_if="Option::is_none")]
pub quantity:Option<OrderPreviewParamsPrescriptionsItemOverridesQuantity>,
#[serde(rename="daysSupply",skip_serializing_if="Option::is_none")]
pub days_supply:Option<i64>,
#[serde(rename="refills",skip_serializing_if="Option::is_none")]
pub refills:Option<i64>,
#[serde(rename="clinical",skip_serializing_if="Option::is_none")]
pub clinical:Option<OrderPreviewParamsPrescriptionsItemOverridesClinical>,
#[serde(rename="dispensing",skip_serializing_if="Option::is_none")]
pub dispensing:Option<OrderPreviewParamsPrescriptionsItemOverridesDispensing>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPrescriptionsItemOverrides{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["sig","quantity","daysSupply","refills","clinical","dispensing"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPrescriptionsItemOverridesQuantity{
#[serde(rename="value")]
pub value:f64,
#[serde(rename="unit")]
pub unit:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPrescriptionsItemOverridesClinical{
#[serde(rename="compoundingReason",skip_serializing_if="Option::is_none")]
pub compounding_reason:Option<OrderPreviewParamsPrescriptionsItemOverridesClinicalCompoundingReason>,
#[serde(rename="medicationReviewStatus",skip_serializing_if="Option::is_none")]
pub medication_review_status:Option<String>,
#[serde(rename="diagnosisReviewStatus",skip_serializing_if="Option::is_none")]
pub diagnosis_review_status:Option<String>,
#[serde(rename="currentMedications",skip_serializing_if="Option::is_none")]
pub current_medications:Option<Vec<String>>,
#[serde(rename="diagnoses",skip_serializing_if="Option::is_none")]
pub diagnoses:Option<Vec<OrderPreviewParamsPrescriptionsItemOverridesClinicalDiagnosesItem>>,
#[serde(rename="observations",skip_serializing_if="Option::is_none")]
pub observations:Option<Vec<OrderPreviewParamsPrescriptionsItemOverridesClinicalObservationsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPrescriptionsItemOverridesClinical{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["compoundingReason","medicationReviewStatus","diagnosisReviewStatus","currentMedications","diagnoses","observations"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPrescriptionsItemOverridesClinicalCompoundingReason{
#[serde(rename="category",skip_serializing_if="Option::is_none")]
pub category:Option<String>,
#[serde(rename="context",skip_serializing_if="Option::is_none")]
pub context:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPrescriptionsItemOverridesClinicalCompoundingReason{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["category","context"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPrescriptionsItemOverridesClinicalDiagnosesItem{
#[serde(rename="code")]
pub code:String,
#[serde(rename="display")]
pub display:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPrescriptionsItemOverridesClinicalObservationsItem{
#[serde(rename="display")]
pub display:String,
#[serde(rename="unit")]
pub unit:String,
#[serde(rename="value")]
pub value:serde_json::Value,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsPrescriptionsItemOverridesDispensing{
#[serde(rename="dispenseUponAcceptance",skip_serializing_if="Option::is_none")]
pub dispense_upon_acceptance:Option<bool>,
#[serde(rename="shippingOptionId",skip_serializing_if="Option::is_none")]
pub shipping_option_id:Option<String>,
#[serde(rename="shippingAmountCents",skip_serializing_if="Option::is_none")]
pub shipping_amount_cents:Option<i64>,
#[serde(rename="shippingDestinationType",skip_serializing_if="Option::is_none")]
pub shipping_destination_type:Option<String>,
#[serde(rename="pharmacyNotes",skip_serializing_if="Option::is_none")]
pub pharmacy_notes:Option<String>,
#[serde(rename="requestedFillDate",skip_serializing_if="Option::is_none")]
pub requested_fill_date:Option<String>,
#[serde(rename="substitutionPermitted",skip_serializing_if="Option::is_none")]
pub substitution_permitted:Option<bool>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsPrescriptionsItemOverridesDispensing{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["dispenseUponAcceptance","shippingOptionId","shippingAmountCents","shippingDestinationType","pharmacyNotes","requestedFillDate","substitutionPermitted"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPreviewParamsShipping{
#[serde(rename="selection",skip_serializing_if="Option::is_none")]
pub selection:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPreviewParamsShipping{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["selection"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderSignParams{
#[serde(rename="userId",skip_serializing_if="Option::is_none")]
pub user_id:Option<String>,
#[serde(rename="prescriber")]
pub prescriber:PrescriberSelector,
#[serde(rename="signatureAttestation")]
pub signature_attestation:bool,
#[serde(rename="expectedRevision")]
pub expected_revision:String,
#[serde(rename="expectedVersions",skip_serializing_if="Option::is_none")]
pub expected_versions:Option<Vec<OrderSignParamsExpectedVersionsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderSignParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["userId","prescriber","expectedRevision","expectedVersions"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderSignParamsExpectedVersionsItem{
#[serde(rename="prescriptionId")]
pub prescription_id:String,
#[serde(rename="version")]
pub version:i64,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderSignAndSubmitParams{
#[serde(rename="userId",skip_serializing_if="Option::is_none")]
pub user_id:Option<String>,
#[serde(rename="prescriber")]
pub prescriber:PrescriberSelector,
#[serde(rename="signatureAttestation")]
pub signature_attestation:bool,
#[serde(rename="expectedRevision")]
pub expected_revision:String,
#[serde(rename="expectedVersions",skip_serializing_if="Option::is_none")]
pub expected_versions:Option<Vec<OrderSignAndSubmitParamsExpectedVersionsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderSignAndSubmitParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["userId","prescriber","expectedRevision","expectedVersions"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderSignAndSubmitParamsExpectedVersionsItem{
#[serde(rename="prescriptionId")]
pub prescription_id:String,
#[serde(rename="version")]
pub version:i64,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderRejectParams{
#[serde(rename="userId",skip_serializing_if="Option::is_none")]
pub user_id:Option<String>,
#[serde(rename="prescriber",skip_serializing_if="Option::is_none")]
pub prescriber:Option<PrescriberSelector>,
#[serde(rename="reason")]
pub reason:String,
#[serde(rename="expectedRevision",skip_serializing_if="Option::is_none")]
pub expected_revision:Option<String>,
#[serde(rename="expectedVersions",skip_serializing_if="Option::is_none")]
pub expected_versions:Option<Vec<OrderRejectParamsExpectedVersionsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderRejectParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["userId","prescriber","expectedRevision","expectedVersions"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderRejectParamsExpectedVersionsItem{
#[serde(rename="prescriptionId")]
pub prescription_id:String,
#[serde(rename="version")]
pub version:i64,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamRegisterParams{
#[serde(rename="externalId")]
pub external_id:String,
#[serde(rename="email")]
pub email:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="role")]
pub role:String,
#[serde(rename="roles",skip_serializing_if="Option::is_none")]
pub roles:Option<Vec<String>>,
#[serde(rename="profileDetails",skip_serializing_if="Option::is_none")]
pub profile_details:Option<TeamRegisterParamsProfileDetails>,
#[serde(rename="npi",skip_serializing_if="Option::is_none")]
pub npi:Option<String>,
#[serde(rename="licenses",skip_serializing_if="Option::is_none")]
pub licenses:Option<Vec<TeamRegisterParamsLicensesItem>>,
#[serde(rename="legalName",skip_serializing_if="Option::is_none")]
pub legal_name:Option<String>,
#[serde(rename="displayName",skip_serializing_if="Option::is_none")]
pub display_name:Option<String>,
#[serde(rename="credentials",skip_serializing_if="Option::is_none")]
pub credentials:Option<String>,
#[serde(rename="address",skip_serializing_if="Option::is_none")]
pub address:Option<TeamRegisterParamsAddress>,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(rename="locationIds",skip_serializing_if="Option::is_none")]
pub location_ids:Option<Vec<String>>,
#[serde(rename="identityAttestation")]
pub identity_attestation:bool,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamRegisterParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["roles","profileDetails","npi","licenses","legalName","displayName","credentials","address","phone","locationIds"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamRegisterParamsProfileDetails{
#[serde(rename="firstName",skip_serializing_if="Option::is_none")]
pub first_name:Option<String>,
#[serde(rename="middleName",skip_serializing_if="Option::is_none")]
pub middle_name:Option<String>,
#[serde(rename="lastName",skip_serializing_if="Option::is_none")]
pub last_name:Option<String>,
#[serde(rename="namePrefix",skip_serializing_if="Option::is_none")]
pub name_prefix:Option<String>,
#[serde(rename="nameSuffix",skip_serializing_if="Option::is_none")]
pub name_suffix:Option<String>,
#[serde(rename="fax",skip_serializing_if="Option::is_none")]
pub fax:Option<String>,
#[serde(rename="specialties",skip_serializing_if="Option::is_none")]
pub specialties:Option<Vec<TeamRegisterParamsProfileDetailsSpecialtiesItem>>,
#[serde(rename="addresses",skip_serializing_if="Option::is_none")]
pub addresses:Option<Vec<TeamRegisterParamsProfileDetailsAddressesItem>>,
#[serde(rename="otherNames",skip_serializing_if="Option::is_none")]
pub other_names:Option<Vec<TeamRegisterParamsProfileDetailsOtherNamesItem>>,
#[serde(rename="identifiers",skip_serializing_if="Option::is_none")]
pub identifiers:Option<Vec<TeamRegisterParamsProfileDetailsIdentifiersItem>>,
#[serde(rename="endpoints",skip_serializing_if="Option::is_none")]
pub endpoints:Option<Vec<TeamRegisterParamsProfileDetailsEndpointsItem>>,
#[serde(rename="certifications",skip_serializing_if="Option::is_none")]
pub certifications:Option<Vec<TeamRegisterParamsProfileDetailsCertificationsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamRegisterParamsProfileDetails{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["firstName","middleName","lastName","namePrefix","nameSuffix","fax","specialties","addresses","otherNames","identifiers","endpoints","certifications"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamRegisterParamsProfileDetailsSpecialtiesItem{
#[serde(rename="code")]
pub code:String,
#[serde(rename="description")]
pub description:String,
#[serde(rename="primary")]
pub primary:bool,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamRegisterParamsProfileDetailsAddressesItem{
#[serde(rename="purpose")]
pub purpose:String,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2")]
pub line2:String,
#[serde(rename="city")]
pub city:String,
#[serde(rename="state")]
pub state:String,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="country")]
pub country:String,
#[serde(rename="phone")]
pub phone:String,
#[serde(rename="fax")]
pub fax:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamRegisterParamsProfileDetailsOtherNamesItem{
#[serde(rename="name")]
pub name:String,
#[serde(rename="credentials")]
pub credentials:String,
#[serde(rename="type")]
pub r#type:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamRegisterParamsProfileDetailsIdentifiersItem{
#[serde(rename="identifier")]
pub identifier:String,
#[serde(rename="issuer")]
pub issuer:String,
#[serde(rename="state")]
pub state:String,
#[serde(rename="description")]
pub description:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamRegisterParamsProfileDetailsEndpointsItem{
#[serde(rename="endpoint")]
pub endpoint:String,
#[serde(rename="type")]
pub r#type:String,
#[serde(rename="description")]
pub description:String,
#[serde(rename="use")]
pub r#use:String,
#[serde(rename="affiliation")]
pub affiliation:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamRegisterParamsProfileDetailsCertificationsItem{
#[serde(rename="name")]
pub name:String,
#[serde(rename="issuer")]
pub issuer:String,
#[serde(rename="expiresAt")]
pub expires_at:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamRegisterParamsLicensesItem{
#[serde(rename="state")]
pub state:String,
#[serde(rename="licenseNumber")]
pub license_number:String,
#[serde(rename="expiresAt",skip_serializing_if="Option::is_none")]
pub expires_at:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamRegisterParamsLicensesItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["expiresAt"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamRegisterParamsAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country")]
pub country:String,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamRegisterParamsAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientAddressListParams{
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientAddressListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["status","startingAfter","endingBefore"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientAddressCreateParams{
#[serde(rename="address")]
pub address:PatientAddressCreateParamsAddress,
#[serde(rename="label",skip_serializing_if="Option::is_none")]
pub label:Option<String>,
#[serde(rename="preferredShipping",skip_serializing_if="Option::is_none")]
pub preferred_shipping:Option<bool>,
#[serde(rename="recipientName",skip_serializing_if="Option::is_none")]
pub recipient_name:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientAddressCreateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["label","preferredShipping","recipientName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientAddressCreateParamsAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientAddressCreateParamsAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["country","line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientAddressUpdateParams{
#[serde(rename="address",skip_serializing_if="Option::is_none")]
pub address:Option<PatientAddressUpdateParamsAddress>,
#[serde(rename="label",skip_serializing_if="Option::is_none")]
pub label:Option<String>,
#[serde(rename="recipientName",skip_serializing_if="Option::is_none")]
pub recipient_name:Option<String>,
#[serde(rename="preferredShipping",skip_serializing_if="Option::is_none")]
pub preferred_shipping:Option<bool>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientAddressUpdateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["address","label","recipientName","preferredShipping"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientAddressUpdateParamsAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientAddressUpdateParamsAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["country","line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationCreateParams{
#[serde(rename="externalId")]
pub external_id:String,
#[serde(rename="email")]
pub email:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="role",skip_serializing_if="Option::is_none")]
pub role:Option<String>,
#[serde(rename="roles",skip_serializing_if="Option::is_none")]
pub roles:Option<Vec<String>>,
#[serde(rename="profileDetails",skip_serializing_if="Option::is_none")]
pub profile_details:Option<TeamInvitationCreateParamsProfileDetails>,
#[serde(rename="npi",skip_serializing_if="Option::is_none")]
pub npi:Option<String>,
#[serde(rename="licenses",skip_serializing_if="Option::is_none")]
pub licenses:Option<Vec<TeamInvitationCreateParamsLicensesItem>>,
#[serde(rename="legalName",skip_serializing_if="Option::is_none")]
pub legal_name:Option<String>,
#[serde(rename="displayName",skip_serializing_if="Option::is_none")]
pub display_name:Option<String>,
#[serde(rename="credentials",skip_serializing_if="Option::is_none")]
pub credentials:Option<String>,
#[serde(rename="address",skip_serializing_if="Option::is_none")]
pub address:Option<TeamInvitationCreateParamsAddress>,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(rename="locationIds",skip_serializing_if="Option::is_none")]
pub location_ids:Option<Vec<String>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamInvitationCreateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["role","roles","profileDetails","npi","licenses","legalName","displayName","credentials","address","phone","locationIds"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationCreateParamsProfileDetails{
#[serde(rename="firstName",skip_serializing_if="Option::is_none")]
pub first_name:Option<String>,
#[serde(rename="middleName",skip_serializing_if="Option::is_none")]
pub middle_name:Option<String>,
#[serde(rename="lastName",skip_serializing_if="Option::is_none")]
pub last_name:Option<String>,
#[serde(rename="namePrefix",skip_serializing_if="Option::is_none")]
pub name_prefix:Option<String>,
#[serde(rename="nameSuffix",skip_serializing_if="Option::is_none")]
pub name_suffix:Option<String>,
#[serde(rename="fax",skip_serializing_if="Option::is_none")]
pub fax:Option<String>,
#[serde(rename="specialties",skip_serializing_if="Option::is_none")]
pub specialties:Option<Vec<TeamInvitationCreateParamsProfileDetailsSpecialtiesItem>>,
#[serde(rename="addresses",skip_serializing_if="Option::is_none")]
pub addresses:Option<Vec<TeamInvitationCreateParamsProfileDetailsAddressesItem>>,
#[serde(rename="otherNames",skip_serializing_if="Option::is_none")]
pub other_names:Option<Vec<TeamInvitationCreateParamsProfileDetailsOtherNamesItem>>,
#[serde(rename="identifiers",skip_serializing_if="Option::is_none")]
pub identifiers:Option<Vec<TeamInvitationCreateParamsProfileDetailsIdentifiersItem>>,
#[serde(rename="endpoints",skip_serializing_if="Option::is_none")]
pub endpoints:Option<Vec<TeamInvitationCreateParamsProfileDetailsEndpointsItem>>,
#[serde(rename="certifications",skip_serializing_if="Option::is_none")]
pub certifications:Option<Vec<TeamInvitationCreateParamsProfileDetailsCertificationsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamInvitationCreateParamsProfileDetails{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["firstName","middleName","lastName","namePrefix","nameSuffix","fax","specialties","addresses","otherNames","identifiers","endpoints","certifications"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationCreateParamsProfileDetailsSpecialtiesItem{
#[serde(rename="code")]
pub code:String,
#[serde(rename="description")]
pub description:String,
#[serde(rename="primary")]
pub primary:bool,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationCreateParamsProfileDetailsAddressesItem{
#[serde(rename="purpose")]
pub purpose:String,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2")]
pub line2:String,
#[serde(rename="city")]
pub city:String,
#[serde(rename="state")]
pub state:String,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="country")]
pub country:String,
#[serde(rename="phone")]
pub phone:String,
#[serde(rename="fax")]
pub fax:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationCreateParamsProfileDetailsOtherNamesItem{
#[serde(rename="name")]
pub name:String,
#[serde(rename="credentials")]
pub credentials:String,
#[serde(rename="type")]
pub r#type:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationCreateParamsProfileDetailsIdentifiersItem{
#[serde(rename="identifier")]
pub identifier:String,
#[serde(rename="issuer")]
pub issuer:String,
#[serde(rename="state")]
pub state:String,
#[serde(rename="description")]
pub description:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationCreateParamsProfileDetailsEndpointsItem{
#[serde(rename="endpoint")]
pub endpoint:String,
#[serde(rename="type")]
pub r#type:String,
#[serde(rename="description")]
pub description:String,
#[serde(rename="use")]
pub r#use:String,
#[serde(rename="affiliation")]
pub affiliation:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationCreateParamsProfileDetailsCertificationsItem{
#[serde(rename="name")]
pub name:String,
#[serde(rename="issuer")]
pub issuer:String,
#[serde(rename="expiresAt")]
pub expires_at:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationCreateParamsLicensesItem{
#[serde(rename="state")]
pub state:String,
#[serde(rename="licenseNumber")]
pub license_number:String,
#[serde(rename="expiresAt",skip_serializing_if="Option::is_none")]
pub expires_at:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamInvitationCreateParamsLicensesItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["expiresAt"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationCreateParamsAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country")]
pub country:String,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamInvitationCreateParamsAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamInvitationListParams{
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(rename="email",skip_serializing_if="Option::is_none")]
pub email:Option<String>,
#[serde(rename="externalId",skip_serializing_if="Option::is_none")]
pub external_id:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamInvitationListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["startingAfter","endingBefore","status","email","externalId"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamMemberListParams{
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="search",skip_serializing_if="Option::is_none")]
pub search:Option<String>,
#[serde(rename="role",skip_serializing_if="Option::is_none")]
pub role:Option<String>,
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamMemberListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["startingAfter","endingBefore","search","role","status"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamPrescriberListParams{
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="search",skip_serializing_if="Option::is_none")]
pub search:Option<String>,
#[serde(rename="npi",skip_serializing_if="Option::is_none")]
pub npi:Option<String>,
#[serde(rename="state",skip_serializing_if="Option::is_none")]
pub state:Option<String>,
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamPrescriberListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["startingAfter","endingBefore","search","npi","state","status"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamMemberUpdateParams{
#[serde(rename="role",skip_serializing_if="Option::is_none")]
pub role:Option<String>,
#[serde(rename="roles",skip_serializing_if="Option::is_none")]
pub roles:Option<Vec<String>>,
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(rename="locationIds",skip_serializing_if="Option::is_none")]
pub location_ids:Option<Vec<String>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamMemberUpdateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["role","roles","status","locationIds"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamPrescriberUpdateParams{
#[serde(rename="displayName",skip_serializing_if="Option::is_none")]
pub display_name:Option<String>,
#[serde(rename="legalName",skip_serializing_if="Option::is_none")]
pub legal_name:Option<String>,
#[serde(rename="credentials",skip_serializing_if="Option::is_none")]
pub credentials:Option<String>,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(rename="address",skip_serializing_if="Option::is_none")]
pub address:Option<TeamPrescriberUpdateParamsAddress>,
#[serde(rename="practiceStatus",skip_serializing_if="Option::is_none")]
pub practice_status:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamPrescriberUpdateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["displayName","legalName","credentials","phone","address"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamPrescriberUpdateParamsAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country")]
pub country:String,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamPrescriberUpdateParamsAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamPrescriberLicenseCreateParams{
#[serde(rename="state")]
pub state:String,
#[serde(rename="licenseNumber")]
pub license_number:String,
#[serde(rename="expiresAt",skip_serializing_if="Option::is_none")]
pub expires_at:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamPrescriberLicenseCreateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["expiresAt"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct TeamPrescriberLicenseUpdateParams{
#[serde(rename="state",skip_serializing_if="Option::is_none")]
pub state:Option<String>,
#[serde(rename="licenseNumber",skip_serializing_if="Option::is_none")]
pub license_number:Option<String>,
#[serde(rename="expiresAt",skip_serializing_if="Option::is_none")]
pub expires_at:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl TeamPrescriberLicenseUpdateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["state","licenseNumber","expiresAt"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeListParams{
#[serde(rename="search",skip_serializing_if="Option::is_none")]
pub search:Option<String>,
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["search","endingBefore","startingAfter"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeCreateParams{
#[serde(rename="liveEnabled",skip_serializing_if="Option::is_none")]
pub live_enabled:Option<bool>,
#[serde(rename="address")]
pub address:PracticeCreateParamsAddress,
#[serde(rename="attestations")]
pub attestations:PracticeCreateParamsAttestations,
#[serde(rename="complianceContact",skip_serializing_if="Option::is_none")]
pub compliance_contact:Option<PracticeCreateParamsComplianceContact>,
#[serde(rename="externalId",skip_serializing_if="Option::is_none")]
pub external_id:Option<String>,
#[serde(rename="legalName",skip_serializing_if="Option::is_none")]
pub legal_name:Option<String>,
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<serde_json::Value>,
#[serde(rename="name")]
pub name:String,
#[serde(rename="prescribers",skip_serializing_if="Option::is_none")]
pub prescribers:Option<Vec<PracticeCreateParamsPrescribersItem>>,
#[serde(rename="primaryContact",skip_serializing_if="Option::is_none")]
pub primary_contact:Option<PracticeCreateParamsPrimaryContact>,
#[serde(rename="supportEmail",skip_serializing_if="Option::is_none")]
pub support_email:Option<String>,
#[serde(rename="supportPhone",skip_serializing_if="Option::is_none")]
pub support_phone:Option<String>,
#[serde(rename="timezone",skip_serializing_if="Option::is_none")]
pub timezone:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeCreateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["complianceContact","externalId","legalName","metadata","prescribers","primaryContact","supportEmail","supportPhone","timezone"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeCreateParamsAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeCreateParamsAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["country","line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeCreateParamsAttestations{
#[serde(rename="authorizedPracticeRelationship")]
pub authorized_practice_relationship:bool,
#[serde(rename="authorizedPhiTransfer")]
pub authorized_phi_transfer:bool,
#[serde(rename="minimumNecessaryPhi")]
pub minimum_necessary_phi:bool,
#[serde(rename="providerDataAccuracy")]
pub provider_data_accuracy:bool,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeCreateParamsComplianceContact{
#[serde(rename="email")]
pub email:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeCreateParamsComplianceContact{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["phone"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeCreateParamsPrescribersItem{
#[serde(rename="credentials",skip_serializing_if="Option::is_none")]
pub credentials:Option<String>,
#[serde(rename="licenseStates")]
pub license_states:Vec<String>,
#[serde(rename="name")]
pub name:String,
#[serde(rename="npi")]
pub npi:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeCreateParamsPrescribersItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["credentials"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeCreateParamsPrimaryContact{
#[serde(rename="email")]
pub email:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeCreateParamsPrimaryContact{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["phone"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeUpdateParams{
#[serde(rename="liveEnabled",skip_serializing_if="Option::is_none")]
pub live_enabled:Option<bool>,
#[serde(rename="address",skip_serializing_if="Option::is_none")]
pub address:Option<PracticeUpdateParamsAddress>,
#[serde(rename="attestations",skip_serializing_if="Option::is_none")]
pub attestations:Option<PracticeUpdateParamsAttestations>,
#[serde(rename="complianceContact",skip_serializing_if="Option::is_none")]
pub compliance_contact:Option<PracticeUpdateParamsComplianceContact>,
#[serde(rename="externalId",skip_serializing_if="Option::is_none")]
pub external_id:Option<String>,
#[serde(rename="legalName",skip_serializing_if="Option::is_none")]
pub legal_name:Option<String>,
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<serde_json::Value>,
#[serde(rename="name",skip_serializing_if="Option::is_none")]
pub name:Option<String>,
#[serde(rename="prescribers",skip_serializing_if="Option::is_none")]
pub prescribers:Option<Vec<PracticeUpdateParamsPrescribersItem>>,
#[serde(rename="primaryContact",skip_serializing_if="Option::is_none")]
pub primary_contact:Option<PracticeUpdateParamsPrimaryContact>,
#[serde(rename="supportEmail",skip_serializing_if="Option::is_none")]
pub support_email:Option<String>,
#[serde(rename="supportPhone",skip_serializing_if="Option::is_none")]
pub support_phone:Option<String>,
#[serde(rename="timezone",skip_serializing_if="Option::is_none")]
pub timezone:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeUpdateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["address","attestations","complianceContact","externalId","legalName","metadata","name","prescribers","primaryContact","supportEmail","supportPhone","timezone"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeUpdateParamsAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeUpdateParamsAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["country","line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeUpdateParamsAttestations{
#[serde(rename="authorizedPracticeRelationship")]
pub authorized_practice_relationship:bool,
#[serde(rename="authorizedPhiTransfer")]
pub authorized_phi_transfer:bool,
#[serde(rename="minimumNecessaryPhi")]
pub minimum_necessary_phi:bool,
#[serde(rename="providerDataAccuracy")]
pub provider_data_accuracy:bool,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeUpdateParamsComplianceContact{
#[serde(rename="email")]
pub email:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeUpdateParamsComplianceContact{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["phone"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeUpdateParamsPrescribersItem{
#[serde(rename="credentials",skip_serializing_if="Option::is_none")]
pub credentials:Option<String>,
#[serde(rename="licenseStates")]
pub license_states:Vec<String>,
#[serde(rename="name")]
pub name:String,
#[serde(rename="npi")]
pub npi:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeUpdateParamsPrescribersItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["credentials"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PracticeUpdateParamsPrimaryContact{
#[serde(rename="email")]
pub email:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PracticeUpdateParamsPrimaryContact{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["phone"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientListParams{
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(rename="externalId",skip_serializing_if="Option::is_none")]
pub external_id:Option<String>,
#[serde(rename="externalIdentitySource",skip_serializing_if="Option::is_none")]
pub external_identity_source:Option<String>,
#[serde(rename="externalIdentityValue",skip_serializing_if="Option::is_none")]
pub external_identity_value:Option<String>,
#[serde(rename="gender",skip_serializing_if="Option::is_none")]
pub gender:Option<String>,
#[serde(rename="lastOrderAfter",skip_serializing_if="Option::is_none")]
pub last_order_after:Option<String>,
#[serde(rename="lastOrderBefore",skip_serializing_if="Option::is_none")]
pub last_order_before:Option<String>,
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="program",skip_serializing_if="Option::is_none")]
pub program:Option<String>,
#[serde(rename="query",skip_serializing_if="Option::is_none")]
pub query:Option<String>,
#[serde(rename="sort",skip_serializing_if="Option::is_none")]
pub sort:Option<String>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(rename="states",skip_serializing_if="Option::is_none")]
pub states:Option<String>,
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["endingBefore","externalId","externalIdentitySource","externalIdentityValue","gender","lastOrderAfter","lastOrderBefore","program","query","sort","startingAfter","states","status"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientCreateParams{
#[serde(rename="address",skip_serializing_if="Option::is_none")]
pub address:Option<PatientCreateParamsAddress>,
#[serde(rename="clinicalProfile",skip_serializing_if="Option::is_none")]
pub clinical_profile:Option<PatientCreateParamsClinicalProfile>,
#[serde(rename="dateOfBirth")]
pub date_of_birth:String,
#[serde(rename="email",skip_serializing_if="Option::is_none")]
pub email:Option<String>,
#[serde(rename="externalId",skip_serializing_if="Option::is_none")]
pub external_id:Option<String>,
#[serde(rename="externalIdentities",skip_serializing_if="Option::is_none")]
pub external_identities:Option<Vec<PatientCreateParamsExternalIdentitiesItem>>,
#[serde(rename="addresses",skip_serializing_if="Option::is_none")]
pub addresses:Option<Vec<PatientCreateParamsAddressesItem>>,
#[serde(rename="encounters",skip_serializing_if="Option::is_none")]
pub encounters:Option<Vec<PatientCreateParamsEncountersItem>>,
#[serde(rename="gender",skip_serializing_if="Option::is_none")]
pub gender:Option<String>,
#[serde(rename="locationId",skip_serializing_if="Option::is_none")]
pub location_id:Option<String>,
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<serde_json::Value>,
#[serde(rename="medicalRecordNumber",skip_serializing_if="Option::is_none")]
pub medical_record_number:Option<String>,
#[serde(rename="measurements",skip_serializing_if="Option::is_none")]
pub measurements:Option<Vec<PatientCreateParamsMeasurementsItem>>,
#[serde(rename="name")]
pub name:PatientName,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(rename="programs",skip_serializing_if="Option::is_none")]
pub programs:Option<Vec<PatientCreateParamsProgramsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientCreateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["address","clinicalProfile","email","externalId","externalIdentities","addresses","encounters","gender","locationId","metadata","medicalRecordNumber","measurements","phone","programs"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientCreateParamsAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientCreateParamsAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["line2","country"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientCreateParamsClinicalProfile{
#[serde(rename="currentMedications")]
pub current_medications:Vec<String>,
#[serde(rename="heightInches",skip_serializing_if="Option::is_none")]
pub height_inches:Option<serde_json::Value>,
#[serde(rename="reviewedAt",skip_serializing_if="Option::is_none")]
pub reviewed_at:Option<String>,
#[serde(rename="weightPounds",skip_serializing_if="Option::is_none")]
pub weight_pounds:Option<serde_json::Value>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientCreateParamsClinicalProfile{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["heightInches","reviewedAt","weightPounds"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientCreateParamsExternalIdentitiesItem{
#[serde(rename="source")]
pub source:String,
#[serde(rename="value")]
pub value:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientCreateParamsAddressesItem{
#[serde(rename="id",skip_serializing_if="Option::is_none")]
pub id:Option<String>,
#[serde(rename="address")]
pub address:PatientCreateParamsAddressesItemAddress,
#[serde(rename="label")]
pub label:String,
#[serde(rename="preferredShipping")]
pub preferred_shipping:bool,
#[serde(rename="recipientName")]
pub recipient_name:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientCreateParamsAddressesItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["id","recipientName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientCreateParamsAddressesItemAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientCreateParamsAddressesItemAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["country","line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientCreateParamsEncountersItem{
#[serde(rename="notes")]
pub notes:String,
#[serde(rename="occurredAt")]
pub occurred_at:String,
#[serde(rename="providerName")]
pub provider_name:String,
#[serde(rename="type")]
pub r#type:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientCreateParamsEncountersItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["notes","providerName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientCreateParamsMeasurementsItem{
#[serde(rename="heightCentimeters")]
pub height_centimeters:serde_json::Value,
#[serde(rename="recordedAt")]
pub recorded_at:String,
#[serde(rename="source")]
pub source:String,
#[serde(rename="weightKilograms")]
pub weight_kilograms:serde_json::Value,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientCreateParamsMeasurementsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["heightCentimeters","weightKilograms"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientName{
#[serde(rename="first")]
pub first:String,
#[serde(rename="last")]
pub last:String,
#[serde(rename="middle",skip_serializing_if="Option::is_none")]
pub middle:Option<String>,
#[serde(rename="preferred",skip_serializing_if="Option::is_none")]
pub preferred:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientName{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["middle","preferred"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientCreateParamsProgramsItem{
#[serde(rename="endedAt")]
pub ended_at:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="startedAt")]
pub started_at:String,
#[serde(rename="status")]
pub status:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientCreateParamsProgramsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["endedAt"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientUpdateParams{
#[serde(rename="address",skip_serializing_if="Option::is_none")]
pub address:Option<PatientUpdateParamsAddress>,
#[serde(rename="clinicalProfile",skip_serializing_if="Option::is_none")]
pub clinical_profile:Option<PatientUpdateParamsClinicalProfile>,
#[serde(rename="dateOfBirth",skip_serializing_if="Option::is_none")]
pub date_of_birth:Option<String>,
#[serde(rename="email",skip_serializing_if="Option::is_none")]
pub email:Option<String>,
#[serde(rename="externalId",skip_serializing_if="Option::is_none")]
pub external_id:Option<String>,
#[serde(rename="externalIdentities",skip_serializing_if="Option::is_none")]
pub external_identities:Option<Vec<PatientUpdateParamsExternalIdentitiesItem>>,
#[serde(rename="addresses",skip_serializing_if="Option::is_none")]
pub addresses:Option<Vec<PatientUpdateParamsAddressesItem>>,
#[serde(rename="encounters",skip_serializing_if="Option::is_none")]
pub encounters:Option<Vec<PatientUpdateParamsEncountersItem>>,
#[serde(rename="gender",skip_serializing_if="Option::is_none")]
pub gender:Option<String>,
#[serde(rename="locationId",skip_serializing_if="Option::is_none")]
pub location_id:Option<String>,
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<serde_json::Value>,
#[serde(rename="medicalRecordNumber",skip_serializing_if="Option::is_none")]
pub medical_record_number:Option<String>,
#[serde(rename="measurements",skip_serializing_if="Option::is_none")]
pub measurements:Option<Vec<PatientUpdateParamsMeasurementsItem>>,
#[serde(rename="name",skip_serializing_if="Option::is_none")]
pub name:Option<PatientUpdateParamsName>,
#[serde(rename="programs",skip_serializing_if="Option::is_none")]
pub programs:Option<Vec<PatientUpdateParamsProgramsItem>>,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(rename="status",skip_serializing_if="Option::is_none")]
pub status:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientUpdateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["address","clinicalProfile","dateOfBirth","email","externalId","externalIdentities","addresses","encounters","gender","locationId","metadata","medicalRecordNumber","measurements","name","programs","phone","status"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientUpdateParamsAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientUpdateParamsAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["line2","country"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientUpdateParamsClinicalProfile{
#[serde(rename="currentMedications",skip_serializing_if="Option::is_none")]
pub current_medications:Option<Vec<String>>,
#[serde(rename="heightInches",skip_serializing_if="Option::is_none")]
pub height_inches:Option<serde_json::Value>,
#[serde(rename="reviewedAt",skip_serializing_if="Option::is_none")]
pub reviewed_at:Option<String>,
#[serde(rename="weightPounds",skip_serializing_if="Option::is_none")]
pub weight_pounds:Option<serde_json::Value>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientUpdateParamsClinicalProfile{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["currentMedications","heightInches","reviewedAt","weightPounds"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientUpdateParamsExternalIdentitiesItem{
#[serde(rename="source")]
pub source:String,
#[serde(rename="value")]
pub value:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientUpdateParamsAddressesItem{
#[serde(rename="id",skip_serializing_if="Option::is_none")]
pub id:Option<String>,
#[serde(rename="address")]
pub address:PatientUpdateParamsAddressesItemAddress,
#[serde(rename="label")]
pub label:String,
#[serde(rename="preferredShipping")]
pub preferred_shipping:bool,
#[serde(rename="recipientName")]
pub recipient_name:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientUpdateParamsAddressesItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["id","recipientName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientUpdateParamsAddressesItemAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientUpdateParamsAddressesItemAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["country","line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientUpdateParamsEncountersItem{
#[serde(rename="notes")]
pub notes:String,
#[serde(rename="occurredAt")]
pub occurred_at:String,
#[serde(rename="providerName")]
pub provider_name:String,
#[serde(rename="type")]
pub r#type:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientUpdateParamsEncountersItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["notes","providerName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientUpdateParamsMeasurementsItem{
#[serde(rename="heightCentimeters")]
pub height_centimeters:serde_json::Value,
#[serde(rename="recordedAt")]
pub recorded_at:String,
#[serde(rename="source")]
pub source:String,
#[serde(rename="weightKilograms")]
pub weight_kilograms:serde_json::Value,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientUpdateParamsMeasurementsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["heightCentimeters","weightKilograms"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientUpdateParamsName{
#[serde(rename="first",skip_serializing_if="Option::is_none")]
pub first:Option<String>,
#[serde(rename="last",skip_serializing_if="Option::is_none")]
pub last:Option<String>,
#[serde(rename="middle",skip_serializing_if="Option::is_none")]
pub middle:Option<String>,
#[serde(rename="preferred",skip_serializing_if="Option::is_none")]
pub preferred:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientUpdateParamsName{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["first","last","middle","preferred"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientUpdateParamsProgramsItem{
#[serde(rename="endedAt")]
pub ended_at:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="startedAt")]
pub started_at:String,
#[serde(rename="status")]
pub status:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientUpdateParamsProgramsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["endedAt"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientAllergyReplaceParams{
#[serde(rename="allergies")]
pub allergies:Vec<PatientAllergyReplaceParamsAllergiesItem>,
#[serde(rename="reviewStatus")]
pub review_status:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientAllergyReplaceParamsAllergiesItem{
#[serde(rename="category")]
pub category:String,
#[serde(rename="code",skip_serializing_if="Option::is_none")]
pub code:Option<String>,
#[serde(rename="codeSystem",skip_serializing_if="Option::is_none")]
pub code_system:Option<String>,
#[serde(rename="reactions")]
pub reactions:Vec<PatientAllergyReplaceParamsAllergiesItemReactionsItem>,
#[serde(rename="severity",skip_serializing_if="Option::is_none")]
pub severity:Option<String>,
#[serde(rename="source")]
pub source:String,
#[serde(rename="substance")]
pub substance:String,
#[serde(rename="type")]
pub r#type:String,
#[serde(rename="verificationStatus")]
pub verification_status:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientAllergyReplaceParamsAllergiesItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["code","codeSystem","severity","type"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct PatientAllergyReplaceParamsAllergiesItemReactionsItem{
#[serde(rename="code",skip_serializing_if="Option::is_none")]
pub code:Option<String>,
#[serde(rename="codeSystem",skip_serializing_if="Option::is_none")]
pub code_system:Option<String>,
#[serde(rename="display")]
pub display:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl PatientAllergyReplaceParamsAllergiesItemReactionsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["code","codeSystem"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionAddParams{
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<std::collections::HashMap<String,serde_json::Value>>,
#[serde(rename="expectedRevision",skip_serializing_if="Option::is_none")]
pub expected_revision:Option<String>,
#[serde(rename="expectedVersions",skip_serializing_if="Option::is_none")]
pub expected_versions:Option<Vec<OrderPrescriptionAddParamsExpectedVersionsItem>>,
#[serde(rename="prescription")]
pub prescription:OrderPrescriptionAddParamsPrescription,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionAddParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["metadata","expectedRevision","expectedVersions"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionAddParamsExpectedVersionsItem{
#[serde(rename="prescriptionId")]
pub prescription_id:String,
#[serde(rename="version")]
pub version:i64,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionAddParamsPrescription{
#[serde(rename="externalPrescriptionId",skip_serializing_if="Option::is_none")]
pub external_prescription_id:Option<String>,
#[serde(rename="clinical",skip_serializing_if="Option::is_none")]
pub clinical:Option<OrderPrescriptionAddParamsPrescriptionClinical>,
#[serde(rename="pharmacyId",skip_serializing_if="Option::is_none")]
pub pharmacy_id:Option<String>,
#[serde(rename="daysSupply")]
pub days_supply:i64,
#[serde(rename="dispensing")]
pub dispensing:OrderPrescriptionAddParamsPrescriptionDispensing,
#[serde(rename="directions")]
pub directions:String,
#[serde(rename="medicationId")]
pub medication_id:String,
#[serde(rename="quantity")]
pub quantity:serde_json::Value,
#[serde(rename="quantityUnit")]
pub quantity_unit:String,
#[serde(rename="refills")]
pub refills:i64,
#[serde(rename="structuredSig",skip_serializing_if="Option::is_none")]
pub structured_sig:Option<OrderPrescriptionAddParamsPrescriptionStructuredSig>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionAddParamsPrescription{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["externalPrescriptionId","clinical","pharmacyId","structuredSig"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionAddParamsPrescriptionClinical{
#[serde(rename="compoundingReason",skip_serializing_if="Option::is_none")]
pub compounding_reason:Option<OrderPrescriptionAddParamsPrescriptionClinicalCompoundingReason>,
#[serde(rename="medicationReviewStatus",skip_serializing_if="Option::is_none")]
pub medication_review_status:Option<String>,
#[serde(rename="diagnosisReviewStatus",skip_serializing_if="Option::is_none")]
pub diagnosis_review_status:Option<String>,
#[serde(rename="currentMedications",skip_serializing_if="Option::is_none")]
pub current_medications:Option<Vec<String>>,
#[serde(rename="diagnoses",skip_serializing_if="Option::is_none")]
pub diagnoses:Option<Vec<OrderPrescriptionAddParamsPrescriptionClinicalDiagnosesItem>>,
#[serde(rename="observations",skip_serializing_if="Option::is_none")]
pub observations:Option<Vec<OrderPrescriptionAddParamsPrescriptionClinicalObservationsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionAddParamsPrescriptionClinical{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["compoundingReason","medicationReviewStatus","diagnosisReviewStatus","currentMedications","diagnoses","observations"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionAddParamsPrescriptionClinicalCompoundingReason{
#[serde(rename="category",skip_serializing_if="Option::is_none")]
pub category:Option<String>,
#[serde(rename="context",skip_serializing_if="Option::is_none")]
pub context:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionAddParamsPrescriptionClinicalCompoundingReason{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["category","context"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionAddParamsPrescriptionClinicalDiagnosesItem{
#[serde(rename="code")]
pub code:String,
#[serde(rename="display")]
pub display:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionAddParamsPrescriptionClinicalObservationsItem{
#[serde(rename="display")]
pub display:String,
#[serde(rename="unit")]
pub unit:String,
#[serde(rename="value")]
pub value:serde_json::Value,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionAddParamsPrescriptionDispensing{
#[serde(rename="dispenseUponAcceptance",skip_serializing_if="Option::is_none")]
pub dispense_upon_acceptance:Option<bool>,
#[serde(rename="shippingOptionId",skip_serializing_if="Option::is_none")]
pub shipping_option_id:Option<String>,
#[serde(rename="shippingAmountCents",skip_serializing_if="Option::is_none")]
pub shipping_amount_cents:Option<i64>,
#[serde(rename="shippingDestinationType",skip_serializing_if="Option::is_none")]
pub shipping_destination_type:Option<String>,
#[serde(rename="pharmacyNotes",skip_serializing_if="Option::is_none")]
pub pharmacy_notes:Option<String>,
#[serde(rename="requestedFillDate",skip_serializing_if="Option::is_none")]
pub requested_fill_date:Option<String>,
#[serde(rename="substitutionPermitted",skip_serializing_if="Option::is_none")]
pub substitution_permitted:Option<bool>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionAddParamsPrescriptionDispensing{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["dispenseUponAcceptance","shippingOptionId","shippingAmountCents","shippingDestinationType","pharmacyNotes","requestedFillDate","substitutionPermitted"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionAddParamsPrescriptionStructuredSig{
#[serde(rename="dose")]
pub dose:String,
#[serde(rename="doseUnit")]
pub dose_unit:String,
#[serde(rename="duration",skip_serializing_if="Option::is_none")]
pub duration:Option<String>,
#[serde(rename="frequency")]
pub frequency:String,
#[serde(rename="indication",skip_serializing_if="Option::is_none")]
pub indication:Option<String>,
#[serde(rename="maxDailyUse",skip_serializing_if="Option::is_none")]
pub max_daily_use:Option<String>,
#[serde(rename="prn",skip_serializing_if="Option::is_none")]
pub prn:Option<bool>,
#[serde(rename="route")]
pub route:String,
#[serde(rename="titrationSchedule",skip_serializing_if="Option::is_none")]
pub titration_schedule:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionAddParamsPrescriptionStructuredSig{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["duration","indication","maxDailyUse","prn","titrationSchedule"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionUpdateParams{
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<std::collections::HashMap<String,serde_json::Value>>,
#[serde(rename="expectedRevision",skip_serializing_if="Option::is_none")]
pub expected_revision:Option<String>,
#[serde(rename="expectedVersions",skip_serializing_if="Option::is_none")]
pub expected_versions:Option<Vec<OrderPrescriptionUpdateParamsExpectedVersionsItem>>,
#[serde(rename="prescription")]
pub prescription:OrderPrescriptionUpdateParamsPrescription,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionUpdateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["metadata","expectedRevision","expectedVersions"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionUpdateParamsExpectedVersionsItem{
#[serde(rename="prescriptionId")]
pub prescription_id:String,
#[serde(rename="version")]
pub version:i64,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionUpdateParamsPrescription{
#[serde(rename="clinical",skip_serializing_if="Option::is_none")]
pub clinical:Option<OrderPrescriptionUpdateParamsPrescriptionClinical>,
#[serde(rename="pharmacyId",skip_serializing_if="Option::is_none")]
pub pharmacy_id:Option<String>,
#[serde(rename="daysSupply")]
pub days_supply:i64,
#[serde(rename="dispensing")]
pub dispensing:OrderPrescriptionUpdateParamsPrescriptionDispensing,
#[serde(rename="directions")]
pub directions:String,
#[serde(rename="medicationId")]
pub medication_id:String,
#[serde(rename="quantity")]
pub quantity:serde_json::Value,
#[serde(rename="quantityUnit")]
pub quantity_unit:String,
#[serde(rename="refills")]
pub refills:i64,
#[serde(rename="structuredSig",skip_serializing_if="Option::is_none")]
pub structured_sig:Option<OrderPrescriptionUpdateParamsPrescriptionStructuredSig>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionUpdateParamsPrescription{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["clinical","pharmacyId","structuredSig"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionUpdateParamsPrescriptionClinical{
#[serde(rename="compoundingReason",skip_serializing_if="Option::is_none")]
pub compounding_reason:Option<OrderPrescriptionUpdateParamsPrescriptionClinicalCompoundingReason>,
#[serde(rename="medicationReviewStatus",skip_serializing_if="Option::is_none")]
pub medication_review_status:Option<String>,
#[serde(rename="diagnosisReviewStatus",skip_serializing_if="Option::is_none")]
pub diagnosis_review_status:Option<String>,
#[serde(rename="currentMedications",skip_serializing_if="Option::is_none")]
pub current_medications:Option<Vec<String>>,
#[serde(rename="diagnoses",skip_serializing_if="Option::is_none")]
pub diagnoses:Option<Vec<OrderPrescriptionUpdateParamsPrescriptionClinicalDiagnosesItem>>,
#[serde(rename="observations",skip_serializing_if="Option::is_none")]
pub observations:Option<Vec<OrderPrescriptionUpdateParamsPrescriptionClinicalObservationsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionUpdateParamsPrescriptionClinical{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["compoundingReason","medicationReviewStatus","diagnosisReviewStatus","currentMedications","diagnoses","observations"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionUpdateParamsPrescriptionClinicalCompoundingReason{
#[serde(rename="category",skip_serializing_if="Option::is_none")]
pub category:Option<String>,
#[serde(rename="context",skip_serializing_if="Option::is_none")]
pub context:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionUpdateParamsPrescriptionClinicalCompoundingReason{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["category","context"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionUpdateParamsPrescriptionClinicalDiagnosesItem{
#[serde(rename="code")]
pub code:String,
#[serde(rename="display")]
pub display:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionUpdateParamsPrescriptionClinicalObservationsItem{
#[serde(rename="display")]
pub display:String,
#[serde(rename="unit")]
pub unit:String,
#[serde(rename="value")]
pub value:serde_json::Value,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionUpdateParamsPrescriptionDispensing{
#[serde(rename="dispenseUponAcceptance",skip_serializing_if="Option::is_none")]
pub dispense_upon_acceptance:Option<bool>,
#[serde(rename="shippingOptionId",skip_serializing_if="Option::is_none")]
pub shipping_option_id:Option<String>,
#[serde(rename="shippingAmountCents",skip_serializing_if="Option::is_none")]
pub shipping_amount_cents:Option<i64>,
#[serde(rename="shippingDestinationType",skip_serializing_if="Option::is_none")]
pub shipping_destination_type:Option<String>,
#[serde(rename="pharmacyNotes",skip_serializing_if="Option::is_none")]
pub pharmacy_notes:Option<String>,
#[serde(rename="requestedFillDate",skip_serializing_if="Option::is_none")]
pub requested_fill_date:Option<String>,
#[serde(rename="substitutionPermitted",skip_serializing_if="Option::is_none")]
pub substitution_permitted:Option<bool>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionUpdateParamsPrescriptionDispensing{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["dispenseUponAcceptance","shippingOptionId","shippingAmountCents","shippingDestinationType","pharmacyNotes","requestedFillDate","substitutionPermitted"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderPrescriptionUpdateParamsPrescriptionStructuredSig{
#[serde(rename="dose")]
pub dose:String,
#[serde(rename="doseUnit")]
pub dose_unit:String,
#[serde(rename="duration",skip_serializing_if="Option::is_none")]
pub duration:Option<String>,
#[serde(rename="frequency")]
pub frequency:String,
#[serde(rename="indication",skip_serializing_if="Option::is_none")]
pub indication:Option<String>,
#[serde(rename="maxDailyUse",skip_serializing_if="Option::is_none")]
pub max_daily_use:Option<String>,
#[serde(rename="prn",skip_serializing_if="Option::is_none")]
pub prn:Option<bool>,
#[serde(rename="route")]
pub route:String,
#[serde(rename="titrationSchedule",skip_serializing_if="Option::is_none")]
pub titration_schedule:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderPrescriptionUpdateParamsPrescriptionStructuredSig{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["duration","indication","maxDailyUse","prn","titrationSchedule"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParams{
#[serde(rename="userId",skip_serializing_if="Option::is_none")]
pub user_id:Option<String>,
#[serde(rename="prescriber",skip_serializing_if="Option::is_none")]
pub prescriber:Option<PrescriberSelector>,
#[serde(rename="orders")]
pub orders:Vec<OrderBatchCreateParamsOrdersItem>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["userId","prescriber"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItem{
#[serde(rename="otcItems",skip_serializing_if="Option::is_none")]
pub otc_items:Option<Vec<OrderBatchCreateParamsOrdersItemOtcItemsItem>>,
#[serde(rename="externalOrderId",skip_serializing_if="Option::is_none")]
pub external_order_id:Option<String>,
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<std::collections::HashMap<String,serde_json::Value>>,
#[serde(rename="patientId",skip_serializing_if="Option::is_none")]
pub patient_id:Option<String>,
#[serde(rename="patient",skip_serializing_if="Option::is_none")]
pub patient:Option<OrderBatchCreateParamsOrdersItemPatient>,
#[serde(rename="shippingAddressId",skip_serializing_if="Option::is_none")]
pub shipping_address_id:Option<String>,
#[serde(rename="prescriptions")]
pub prescriptions:Vec<OrderBatchCreateParamsOrdersItemPrescriptionsItem>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["otcItems","externalOrderId","metadata","patientId","patient","shippingAddressId"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemOtcItemsItem{
#[serde(rename="catalogItemId")]
pub catalog_item_id:String,
#[serde(rename="quantity")]
pub quantity:i64,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPatient{
#[serde(rename="address",skip_serializing_if="Option::is_none")]
pub address:Option<OrderBatchCreateParamsOrdersItemPatientAddress>,
#[serde(rename="clinicalProfile",skip_serializing_if="Option::is_none")]
pub clinical_profile:Option<OrderBatchCreateParamsOrdersItemPatientClinicalProfile>,
#[serde(rename="dateOfBirth")]
pub date_of_birth:String,
#[serde(rename="email",skip_serializing_if="Option::is_none")]
pub email:Option<String>,
#[serde(rename="externalId",skip_serializing_if="Option::is_none")]
pub external_id:Option<String>,
#[serde(rename="externalIdentities",skip_serializing_if="Option::is_none")]
pub external_identities:Option<Vec<OrderBatchCreateParamsOrdersItemPatientExternalIdentitiesItem>>,
#[serde(rename="addresses",skip_serializing_if="Option::is_none")]
pub addresses:Option<Vec<OrderBatchCreateParamsOrdersItemPatientAddressesItem>>,
#[serde(rename="encounters",skip_serializing_if="Option::is_none")]
pub encounters:Option<Vec<OrderBatchCreateParamsOrdersItemPatientEncountersItem>>,
#[serde(rename="gender",skip_serializing_if="Option::is_none")]
pub gender:Option<String>,
#[serde(rename="locationId",skip_serializing_if="Option::is_none")]
pub location_id:Option<String>,
#[serde(rename="metadata",skip_serializing_if="Option::is_none")]
pub metadata:Option<serde_json::Value>,
#[serde(rename="medicalRecordNumber",skip_serializing_if="Option::is_none")]
pub medical_record_number:Option<String>,
#[serde(rename="measurements",skip_serializing_if="Option::is_none")]
pub measurements:Option<Vec<OrderBatchCreateParamsOrdersItemPatientMeasurementsItem>>,
#[serde(rename="name")]
pub name:OrderBatchCreateParamsOrdersItemPatientName,
#[serde(rename="phone",skip_serializing_if="Option::is_none")]
pub phone:Option<String>,
#[serde(rename="programs",skip_serializing_if="Option::is_none")]
pub programs:Option<Vec<OrderBatchCreateParamsOrdersItemPatientProgramsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPatient{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["address","clinicalProfile","email","externalId","externalIdentities","addresses","encounters","gender","locationId","metadata","medicalRecordNumber","measurements","phone","programs"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPatientAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPatientAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["line2","country"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPatientClinicalProfile{
#[serde(rename="currentMedications")]
pub current_medications:Vec<String>,
#[serde(rename="heightInches",skip_serializing_if="Option::is_none")]
pub height_inches:Option<serde_json::Value>,
#[serde(rename="reviewedAt",skip_serializing_if="Option::is_none")]
pub reviewed_at:Option<String>,
#[serde(rename="weightPounds",skip_serializing_if="Option::is_none")]
pub weight_pounds:Option<serde_json::Value>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPatientClinicalProfile{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["heightInches","reviewedAt","weightPounds"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPatientExternalIdentitiesItem{
#[serde(rename="source")]
pub source:String,
#[serde(rename="value")]
pub value:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPatientAddressesItem{
#[serde(rename="id",skip_serializing_if="Option::is_none")]
pub id:Option<String>,
#[serde(rename="address")]
pub address:OrderBatchCreateParamsOrdersItemPatientAddressesItemAddress,
#[serde(rename="label")]
pub label:String,
#[serde(rename="preferredShipping")]
pub preferred_shipping:bool,
#[serde(rename="recipientName")]
pub recipient_name:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPatientAddressesItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["id","recipientName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPatientAddressesItemAddress{
#[serde(rename="city")]
pub city:String,
#[serde(rename="country",skip_serializing_if="Option::is_none")]
pub country:Option<String>,
#[serde(rename="line1")]
pub line1:String,
#[serde(rename="line2",skip_serializing_if="Option::is_none")]
pub line2:Option<String>,
#[serde(rename="postalCode")]
pub postal_code:String,
#[serde(rename="state")]
pub state:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPatientAddressesItemAddress{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["country","line2"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPatientEncountersItem{
#[serde(rename="notes")]
pub notes:String,
#[serde(rename="occurredAt")]
pub occurred_at:String,
#[serde(rename="providerName")]
pub provider_name:String,
#[serde(rename="type")]
pub r#type:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPatientEncountersItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["notes","providerName"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPatientMeasurementsItem{
#[serde(rename="heightCentimeters")]
pub height_centimeters:serde_json::Value,
#[serde(rename="recordedAt")]
pub recorded_at:String,
#[serde(rename="source")]
pub source:String,
#[serde(rename="weightKilograms")]
pub weight_kilograms:serde_json::Value,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPatientMeasurementsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["heightCentimeters","weightKilograms"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPatientName{
#[serde(rename="first")]
pub first:String,
#[serde(rename="last")]
pub last:String,
#[serde(rename="middle",skip_serializing_if="Option::is_none")]
pub middle:Option<String>,
#[serde(rename="preferred",skip_serializing_if="Option::is_none")]
pub preferred:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPatientName{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["middle","preferred"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPatientProgramsItem{
#[serde(rename="endedAt")]
pub ended_at:String,
#[serde(rename="name")]
pub name:String,
#[serde(rename="startedAt")]
pub started_at:String,
#[serde(rename="status")]
pub status:String,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPatientProgramsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["endedAt"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPrescriptionsItem{
#[serde(rename="externalPrescriptionId",skip_serializing_if="Option::is_none")]
pub external_prescription_id:Option<String>,
#[serde(rename="clinical",skip_serializing_if="Option::is_none")]
pub clinical:Option<OrderBatchCreateParamsOrdersItemPrescriptionsItemClinical>,
#[serde(rename="pharmacyId",skip_serializing_if="Option::is_none")]
pub pharmacy_id:Option<String>,
#[serde(rename="daysSupply")]
pub days_supply:i64,
#[serde(rename="dispensing")]
pub dispensing:OrderBatchCreateParamsOrdersItemPrescriptionsItemDispensing,
#[serde(rename="directions")]
pub directions:String,
#[serde(rename="medicationId")]
pub medication_id:String,
#[serde(rename="quantity")]
pub quantity:serde_json::Value,
#[serde(rename="quantityUnit")]
pub quantity_unit:String,
#[serde(rename="refills")]
pub refills:i64,
#[serde(rename="structuredSig",skip_serializing_if="Option::is_none")]
pub structured_sig:Option<OrderBatchCreateParamsOrdersItemPrescriptionsItemStructuredSig>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPrescriptionsItem{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["externalPrescriptionId","clinical","pharmacyId","structuredSig"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPrescriptionsItemClinical{
#[serde(rename="compoundingReason",skip_serializing_if="Option::is_none")]
pub compounding_reason:Option<OrderBatchCreateParamsOrdersItemPrescriptionsItemClinicalCompoundingReason>,
#[serde(rename="medicationReviewStatus",skip_serializing_if="Option::is_none")]
pub medication_review_status:Option<String>,
#[serde(rename="diagnosisReviewStatus",skip_serializing_if="Option::is_none")]
pub diagnosis_review_status:Option<String>,
#[serde(rename="currentMedications",skip_serializing_if="Option::is_none")]
pub current_medications:Option<Vec<String>>,
#[serde(rename="diagnoses",skip_serializing_if="Option::is_none")]
pub diagnoses:Option<Vec<OrderBatchCreateParamsOrdersItemPrescriptionsItemClinicalDiagnosesItem>>,
#[serde(rename="observations",skip_serializing_if="Option::is_none")]
pub observations:Option<Vec<OrderBatchCreateParamsOrdersItemPrescriptionsItemClinicalObservationsItem>>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPrescriptionsItemClinical{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["compoundingReason","medicationReviewStatus","diagnosisReviewStatus","currentMedications","diagnoses","observations"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPrescriptionsItemClinicalCompoundingReason{
#[serde(rename="category",skip_serializing_if="Option::is_none")]
pub category:Option<String>,
#[serde(rename="context",skip_serializing_if="Option::is_none")]
pub context:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPrescriptionsItemClinicalCompoundingReason{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["category","context"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPrescriptionsItemClinicalDiagnosesItem{
#[serde(rename="code")]
pub code:String,
#[serde(rename="display")]
pub display:String,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPrescriptionsItemClinicalObservationsItem{
#[serde(rename="display")]
pub display:String,
#[serde(rename="unit")]
pub unit:String,
#[serde(rename="value")]
pub value:serde_json::Value,
}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPrescriptionsItemDispensing{
#[serde(rename="dispenseUponAcceptance",skip_serializing_if="Option::is_none")]
pub dispense_upon_acceptance:Option<bool>,
#[serde(rename="shippingOptionId",skip_serializing_if="Option::is_none")]
pub shipping_option_id:Option<String>,
#[serde(rename="shippingAmountCents",skip_serializing_if="Option::is_none")]
pub shipping_amount_cents:Option<i64>,
#[serde(rename="shippingDestinationType",skip_serializing_if="Option::is_none")]
pub shipping_destination_type:Option<String>,
#[serde(rename="pharmacyNotes",skip_serializing_if="Option::is_none")]
pub pharmacy_notes:Option<String>,
#[serde(rename="requestedFillDate",skip_serializing_if="Option::is_none")]
pub requested_fill_date:Option<String>,
#[serde(rename="substitutionPermitted",skip_serializing_if="Option::is_none")]
pub substitution_permitted:Option<bool>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPrescriptionsItemDispensing{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["dispenseUponAcceptance","shippingOptionId","shippingAmountCents","shippingDestinationType","pharmacyNotes","requestedFillDate","substitutionPermitted"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct OrderBatchCreateParamsOrdersItemPrescriptionsItemStructuredSig{
#[serde(rename="dose")]
pub dose:String,
#[serde(rename="doseUnit")]
pub dose_unit:String,
#[serde(rename="duration",skip_serializing_if="Option::is_none")]
pub duration:Option<String>,
#[serde(rename="frequency")]
pub frequency:String,
#[serde(rename="indication",skip_serializing_if="Option::is_none")]
pub indication:Option<String>,
#[serde(rename="maxDailyUse",skip_serializing_if="Option::is_none")]
pub max_daily_use:Option<String>,
#[serde(rename="prn",skip_serializing_if="Option::is_none")]
pub prn:Option<bool>,
#[serde(rename="route")]
pub route:String,
#[serde(rename="titrationSchedule",skip_serializing_if="Option::is_none")]
pub titration_schedule:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl OrderBatchCreateParamsOrdersItemPrescriptionsItemStructuredSig{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["duration","indication","maxDailyUse","prn","titrationSchedule"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct SellingPriceUpdateParams{
#[serde(rename="amountCents")]
pub amount_cents:i64,
#[serde(rename="baseVersion")]
pub base_version:i64,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl SellingPriceUpdateParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["amountCents"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct WebhookGrantListParams{
#[serde(rename="limit",skip_serializing_if="Option::is_none")]
pub limit:Option<i64>,
#[serde(rename="startingAfter",skip_serializing_if="Option::is_none")]
pub starting_after:Option<String>,
#[serde(rename="endingBefore",skip_serializing_if="Option::is_none")]
pub ending_before:Option<String>,
#[serde(flatten)]
pub null_fields:std::collections::BTreeMap<String,()>,
}
impl WebhookGrantListParams{pub fn clear(mut self,field:&str)->Result<Self,SdkError>{if !["startingAfter","endingBefore"].contains(&field){return Err(SdkError::Invalid("Field cannot be cleared".into()))}self.null_fields.insert(field.into(),());Ok(self)}}
#[derive(Debug,Clone,Default,Serialize)]
pub struct WebhookGrantSaveParams{
#[serde(rename="scopes")]
pub scopes:Vec<String>,
}
}
#[derive(Clone)]
pub struct AccountSDKResource{context:SdkContext,}
impl AccountSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn get(&self,params:AccountGetParams,options:Option<RequestOptions>)->Result<GetAccountResponse,SdkError>{self.context.call("getAccount",vec![],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct ApiKeysSDKResource{context:SdkContext,}
impl ApiKeysSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn create(&self,params:ApiKeyCreateParams,options:Option<RequestOptions>)->Result<CreatePlatformPracticeApiKeyResponse,SdkError>{self.context.call("createPlatformPracticeApiKey",vec![],serde_json::to_value(params)?,options).await}
pub async fn get_access(&self,options:Option<RequestOptions>)->Result<GetApiAccessResponse,SdkError>{self.context.call("getApiAccess",vec![],json!({}),options).await}
}
#[derive(Clone)]
pub struct CatalogSDKResource{context:SdkContext,pub items:CatalogItemsSDKResource,pub prescribing_options:CatalogPrescribingOptionsSDKResource,pub selling_prices:CatalogSellingPricesSDKResource,pub shipping_options:CatalogShippingOptionsSDKResource,}
impl CatalogSDKResource{fn new(context:SdkContext)->Self{Self{items:CatalogItemsSDKResource::new(context.clone()),prescribing_options:CatalogPrescribingOptionsSDKResource::new(context.clone()),selling_prices:CatalogSellingPricesSDKResource::new(context.clone()),shipping_options:CatalogShippingOptionsSDKResource::new(context.clone()),context}}
}
#[derive(Clone)]
pub struct CatalogItemsSDKResource{context:SdkContext,}
impl CatalogItemsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,params:CatalogItemListParams,options:Option<RequestOptions>)->Result<ListCatalogItemsResponse,SdkError>{self.context.call("listCatalogItems",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:CatalogItemListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListCatalogItemsResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listCatalogItems",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
}
#[derive(Clone)]
pub struct CatalogPrescribingOptionsSDKResource{context:SdkContext,}
impl CatalogPrescribingOptionsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn get(&self,catalog_item_id:&str,options:Option<RequestOptions>)->Result<RetrievePrescribingOptionsResponse,SdkError>{self.context.call("retrievePrescribingOptions",vec![catalog_item_id.to_string()],json!({}),options).await}
}
#[derive(Clone)]
pub struct CatalogSellingPricesSDKResource{context:SdkContext,}
impl CatalogSellingPricesSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn get(&self,catalog_item_id:&str,options:Option<RequestOptions>)->Result<PlatformPublicApiSellingPricesReadSellingPriceResponse,SdkError>{self.context.call("platform.public-api.selling-prices.readSellingPrice",vec![catalog_item_id.to_string()],json!({}),options).await}
pub async fn update(&self,catalog_item_id:&str,params:SellingPriceUpdateParams,options:Option<RequestOptions>)->Result<PlatformPublicApiSellingPricesUpdateSellingPriceResponse,SdkError>{self.context.call("platform.public-api.selling-prices.updateSellingPrice",vec![catalog_item_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct CatalogShippingOptionsSDKResource{context:SdkContext,}
impl CatalogShippingOptionsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,catalog_item_id:&str,params:ShippingOptionListParams,options:Option<RequestOptions>)->Result<ListShippingOptionsResponse,SdkError>{self.context.call("listShippingOptions",vec![catalog_item_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct LocationsSDKResource{context:SdkContext,}
impl LocationsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,params:LocationListParams,options:Option<RequestOptions>)->Result<ListPracticeLocationsResponse,SdkError>{self.context.call("listPracticeLocations",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:LocationListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListPracticeLocationsResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listPracticeLocations",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn create(&self,params:LocationCreateParams,options:Option<RequestOptions>)->Result<CreatePracticeLocationResponse,SdkError>{self.context.call("createPracticeLocation",vec![],serde_json::to_value(params)?,options).await}
pub async fn get(&self,location_id:&str,options:Option<RequestOptions>)->Result<GetPracticeLocationResponse,SdkError>{self.context.call("getPracticeLocation",vec![location_id.to_string()],json!({}),options).await}
pub async fn update(&self,location_id:&str,params:LocationUpdateParams,options:Option<RequestOptions>)->Result<UpdatePracticeLocationResponse,SdkError>{self.context.call("updatePracticeLocation",vec![location_id.to_string()],serde_json::to_value(params)?,options).await}
pub async fn archive(&self,location_id:&str,options:Option<RequestOptions>)->Result<ArchivePracticeLocationResponse,SdkError>{self.context.call("archivePracticeLocation",vec![location_id.to_string()],json!({}),options).await}
}
#[derive(Clone)]
pub struct OrdersSDKResource{context:SdkContext,pub batches:OrdersBatchesSDKResource,pub events:OrdersEventsSDKResource,pub exceptions:OrdersExceptionsSDKResource,pub prescriptions:OrdersPrescriptionsSDKResource,pub test_simulation:OrdersTestSimulationSDKResource,}
impl OrdersSDKResource{fn new(context:SdkContext)->Self{Self{batches:OrdersBatchesSDKResource::new(context.clone()),events:OrdersEventsSDKResource::new(context.clone()),exceptions:OrdersExceptionsSDKResource::new(context.clone()),prescriptions:OrdersPrescriptionsSDKResource::new(context.clone()),test_simulation:OrdersTestSimulationSDKResource::new(context.clone()),context}}
pub async fn list(&self,params:OrderListParams,options:Option<RequestOptions>)->Result<ListOrdersResponse,SdkError>{self.context.call("listOrders",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:OrderListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListOrdersResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listOrders",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn create(&self,params:OrderCreateParams,options:Option<RequestOptions>)->Result<CreateOrderResponse,SdkError>{self.context.call("createOrder",vec![],serde_json::to_value(params)?,options).await}
pub async fn get(&self,order_id:&str,options:Option<RequestOptions>)->Result<GetOrderResponse,SdkError>{self.context.call("getOrder",vec![order_id.to_string()],json!({}),options).await}
pub async fn cancel(&self,order_id:&str,params:OrderCancelParams,options:Option<RequestOptions>)->Result<CancelOrderResponse,SdkError>{self.context.call("cancelOrder",vec![order_id.to_string()],serde_json::to_value(params)?,options).await}
pub async fn preview(&self,params:OrderPreviewParams,options:Option<RequestOptions>)->Result<PreviewOrderResponse,SdkError>{self.context.call("previewOrder",vec![],serde_json::to_value(params)?,options).await}
pub async fn sign(&self,order_id:&str,params:OrderSignParams,options:Option<RequestOptions>)->Result<SignOrderResponse,SdkError>{self.context.call("signOrder",vec![order_id.to_string()],serde_json::to_value(params)?,options).await}
pub async fn sign_and_submit(&self,order_id:&str,params:OrderSignAndSubmitParams,options:Option<RequestOptions>)->Result<SignAndSubmitOrderResponse,SdkError>{self.context.call("signAndSubmitOrder",vec![order_id.to_string()],serde_json::to_value(params)?,options).await}
pub async fn submit(&self,order_id:&str,options:Option<RequestOptions>)->Result<SubmitOrderResponse,SdkError>{self.context.call("submitOrder",vec![order_id.to_string()],json!({}),options).await}
pub async fn reject(&self,order_id:&str,params:OrderRejectParams,options:Option<RequestOptions>)->Result<RejectOrderResponse,SdkError>{self.context.call("rejectOrder",vec![order_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct OrdersBatchesSDKResource{context:SdkContext,}
impl OrdersBatchesSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn create(&self,params:OrderBatchCreateParams,options:Option<RequestOptions>)->Result<CreateOrderBatchResponse,SdkError>{self.context.call("createOrderBatch",vec![],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct OrdersEventsSDKResource{context:SdkContext,}
impl OrdersEventsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,order_id:&str,params:OrderEventListParams,options:Option<RequestOptions>)->Result<ListOrderEventsResponse,SdkError>{self.context.call("listOrderEvents",vec![order_id.to_string()],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,order_id:&str,params:OrderEventListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListOrderEventsResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listOrderEvents",vec![order_id.to_string()],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
}
#[derive(Clone)]
pub struct OrdersExceptionsSDKResource{context:SdkContext,}
impl OrdersExceptionsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn act(&self,order_id:&str,exception_id:&str,params:OrderExceptionActParams,options:Option<RequestOptions>)->Result<ActOnOrderExceptionResponse,SdkError>{self.context.call("actOnOrderException",vec![order_id.to_string(),exception_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct OrdersPrescriptionsSDKResource{context:SdkContext,}
impl OrdersPrescriptionsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn add(&self,order_id:&str,params:OrderPrescriptionAddParams,options:Option<RequestOptions>)->Result<AddOrderPrescriptionResponse,SdkError>{self.context.call("addOrderPrescription",vec![order_id.to_string()],serde_json::to_value(params)?,options).await}
pub async fn update(&self,order_id:&str,prescription_id:&str,params:OrderPrescriptionUpdateParams,options:Option<RequestOptions>)->Result<UpdateOrderPrescriptionResponse,SdkError>{self.context.call("updateOrderPrescription",vec![order_id.to_string(),prescription_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct OrdersTestSimulationSDKResource{context:SdkContext,}
impl OrdersTestSimulationSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn get(&self,order_id:&str,options:Option<RequestOptions>)->Result<GetOrderTestSimulationResponse,SdkError>{self.context.call("getOrderTestSimulation",vec![order_id.to_string()],json!({}),options).await}
pub async fn update(&self,order_id:&str,params:OrderTestSimulationUpdateParams,options:Option<RequestOptions>)->Result<UpdateOrderTestSimulationResponse,SdkError>{self.context.call("updateOrderTestSimulation",vec![order_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct PatientsSDKResource{context:SdkContext,pub addresses:PatientsAddressesSDKResource,pub allergies:PatientsAllergiesSDKResource,}
impl PatientsSDKResource{fn new(context:SdkContext)->Self{Self{addresses:PatientsAddressesSDKResource::new(context.clone()),allergies:PatientsAllergiesSDKResource::new(context.clone()),context}}
pub async fn list(&self,params:PatientListParams,options:Option<RequestOptions>)->Result<ListPatientsResponse,SdkError>{self.context.call("listPatients",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:PatientListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListPatientsResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listPatients",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn create(&self,params:PatientCreateParams,options:Option<RequestOptions>)->Result<CreatePatientResponse,SdkError>{self.context.call("createPatient",vec![],serde_json::to_value(params)?,options).await}
pub async fn get(&self,patient_id:&str,options:Option<RequestOptions>)->Result<GetPatientResponse,SdkError>{self.context.call("getPatient",vec![patient_id.to_string()],json!({}),options).await}
pub async fn delete(&self,patient_id:&str,options:Option<RequestOptions>)->Result<DeletePatientResponse,SdkError>{self.context.call("deletePatient",vec![patient_id.to_string()],json!({}),options).await}
pub async fn update(&self,patient_id:&str,params:PatientUpdateParams,options:Option<RequestOptions>)->Result<UpdatePatientResponse,SdkError>{self.context.call("updatePatient",vec![patient_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct PatientsAddressesSDKResource{context:SdkContext,}
impl PatientsAddressesSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,patient_id:&str,params:PatientAddressListParams,options:Option<RequestOptions>)->Result<ListPatientAddressesResponse,SdkError>{self.context.call("listPatientAddresses",vec![patient_id.to_string()],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,patient_id:&str,params:PatientAddressListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListPatientAddressesResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listPatientAddresses",vec![patient_id.to_string()],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn create(&self,patient_id:&str,params:PatientAddressCreateParams,options:Option<RequestOptions>)->Result<CreatePatientAddressResponse,SdkError>{self.context.call("createPatientAddress",vec![patient_id.to_string()],serde_json::to_value(params)?,options).await}
pub async fn update(&self,patient_id:&str,address_id:&str,params:PatientAddressUpdateParams,options:Option<RequestOptions>)->Result<UpdatePatientAddressResponse,SdkError>{self.context.call("updatePatientAddress",vec![patient_id.to_string(),address_id.to_string()],serde_json::to_value(params)?,options).await}
pub async fn archive(&self,patient_id:&str,address_id:&str,options:Option<RequestOptions>)->Result<ArchivePatientAddressResponse,SdkError>{self.context.call("archivePatientAddress",vec![patient_id.to_string(),address_id.to_string()],json!({}),options).await}
pub async fn set_default(&self,patient_id:&str,address_id:&str,options:Option<RequestOptions>)->Result<SetDefaultPatientAddressResponse,SdkError>{self.context.call("setDefaultPatientAddress",vec![patient_id.to_string(),address_id.to_string()],json!({}),options).await}
}
#[derive(Clone)]
pub struct PatientsAllergiesSDKResource{context:SdkContext,}
impl PatientsAllergiesSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn get(&self,patient_id:&str,options:Option<RequestOptions>)->Result<GetPatientAllergiesResponse,SdkError>{self.context.call("getPatientAllergies",vec![patient_id.to_string()],json!({}),options).await}
pub async fn replace(&self,patient_id:&str,params:PatientAllergyReplaceParams,options:Option<RequestOptions>)->Result<ReplacePatientAllergiesResponse,SdkError>{self.context.call("replacePatientAllergies",vec![patient_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct PharmaciesSDKResource{context:SdkContext,}
impl PharmaciesSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,params:PharmacyListParams,options:Option<RequestOptions>)->Result<ListPharmaciesResponse,SdkError>{self.context.call("listPharmacies",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:PharmacyListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListPharmaciesResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listPharmacies",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
}
#[derive(Clone)]
pub struct PracticesSDKResource{context:SdkContext,}
impl PracticesSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,params:PracticeListParams,options:Option<RequestOptions>)->Result<ListPracticesResponse,SdkError>{self.context.call("listPractices",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:PracticeListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListPracticesResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listPractices",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn create(&self,params:PracticeCreateParams,options:Option<RequestOptions>)->Result<CreatePracticeResponse,SdkError>{self.context.call("createPractice",vec![],serde_json::to_value(params)?,options).await}
pub async fn get(&self,practice_id:&str,options:Option<RequestOptions>)->Result<GetPracticeResponse,SdkError>{self.context.call("getPractice",vec![practice_id.to_string()],json!({}),options).await}
pub async fn update(&self,practice_id:&str,params:PracticeUpdateParams,options:Option<RequestOptions>)->Result<UpdatePracticeResponse,SdkError>{self.context.call("updatePractice",vec![practice_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct TeamSDKResource{context:SdkContext,pub invitations:TeamInvitationsSDKResource,pub members:TeamMembersSDKResource,pub prescribers:TeamPrescribersSDKResource,}
impl TeamSDKResource{fn new(context:SdkContext)->Self{Self{invitations:TeamInvitationsSDKResource::new(context.clone()),members:TeamMembersSDKResource::new(context.clone()),prescribers:TeamPrescribersSDKResource::new(context.clone()),context}}
pub async fn register(&self,params:TeamRegisterParams,options:Option<RequestOptions>)->Result<RegisterUserResponse,SdkError>{self.context.call("registerUser",vec![],serde_json::to_value(params)?,options).await}
pub async fn get(&self,options:Option<RequestOptions>)->Result<GetPracticeTeamResponse,SdkError>{self.context.call("getPracticeTeam",vec![],json!({}),options).await}
}
#[derive(Clone)]
pub struct TeamInvitationsSDKResource{context:SdkContext,}
impl TeamInvitationsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn create(&self,params:TeamInvitationCreateParams,options:Option<RequestOptions>)->Result<InvitePracticeTeamPersonResponse,SdkError>{self.context.call("invitePracticeTeamPerson",vec![],serde_json::to_value(params)?,options).await}
pub async fn list(&self,params:TeamInvitationListParams,options:Option<RequestOptions>)->Result<ListPracticeTeamInvitationsResponse,SdkError>{self.context.call("listPracticeTeamInvitations",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:TeamInvitationListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListPracticeTeamInvitationsResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listPracticeTeamInvitations",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn get(&self,invitation_id:&str,options:Option<RequestOptions>)->Result<GetPracticeTeamInvitationResponse,SdkError>{self.context.call("getPracticeTeamInvitation",vec![invitation_id.to_string()],json!({}),options).await}
pub async fn revoke(&self,invitation_id:&str,options:Option<RequestOptions>)->Result<RevokePracticeTeamInvitationResponse,SdkError>{self.context.call("revokePracticeTeamInvitation",vec![invitation_id.to_string()],json!({}),options).await}
pub async fn resend(&self,invitation_id:&str,options:Option<RequestOptions>)->Result<ResendPracticeTeamInvitationResponse,SdkError>{self.context.call("resendPracticeTeamInvitation",vec![invitation_id.to_string()],json!({}),options).await}
}
#[derive(Clone)]
pub struct TeamMembersSDKResource{context:SdkContext,}
impl TeamMembersSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,params:TeamMemberListParams,options:Option<RequestOptions>)->Result<ListPracticeTeamMembersResponse,SdkError>{self.context.call("listPracticeTeamMembers",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:TeamMemberListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListPracticeTeamMembersResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listPracticeTeamMembers",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn get(&self,member_id:&str,options:Option<RequestOptions>)->Result<GetPracticeTeamMemberResponse,SdkError>{self.context.call("getPracticeTeamMember",vec![member_id.to_string()],json!({}),options).await}
pub async fn update(&self,member_id:&str,params:TeamMemberUpdateParams,options:Option<RequestOptions>)->Result<UpdatePracticeTeamMemberResponse,SdkError>{self.context.call("updatePracticeTeamMember",vec![member_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct TeamPrescribersSDKResource{context:SdkContext,pub licenses:TeamPrescribersLicensesSDKResource,}
impl TeamPrescribersSDKResource{fn new(context:SdkContext)->Self{Self{licenses:TeamPrescribersLicensesSDKResource::new(context.clone()),context}}
pub async fn list(&self,params:TeamPrescriberListParams,options:Option<RequestOptions>)->Result<ListPracticeTeamPrescribersResponse,SdkError>{self.context.call("listPracticeTeamPrescribers",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:TeamPrescriberListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListPracticeTeamPrescribersResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listPracticeTeamPrescribers",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn get(&self,prescriber_id:&str,options:Option<RequestOptions>)->Result<GetPracticeTeamPrescriberResponse,SdkError>{self.context.call("getPracticeTeamPrescriber",vec![prescriber_id.to_string()],json!({}),options).await}
pub async fn update(&self,prescriber_id:&str,params:TeamPrescriberUpdateParams,options:Option<RequestOptions>)->Result<UpdatePracticeTeamPrescriberResponse,SdkError>{self.context.call("updatePracticeTeamPrescriber",vec![prescriber_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct TeamPrescribersLicensesSDKResource{context:SdkContext,}
impl TeamPrescribersLicensesSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn create(&self,prescriber_id:&str,params:TeamPrescriberLicenseCreateParams,options:Option<RequestOptions>)->Result<CreatePracticeTeamLicenseResponse,SdkError>{self.context.call("createPracticeTeamLicense",vec![prescriber_id.to_string()],serde_json::to_value(params)?,options).await}
pub async fn update(&self,prescriber_id:&str,license_id:&str,params:TeamPrescriberLicenseUpdateParams,options:Option<RequestOptions>)->Result<UpdatePracticeTeamLicenseResponse,SdkError>{self.context.call("updatePracticeTeamLicense",vec![prescriber_id.to_string(),license_id.to_string()],serde_json::to_value(params)?,options).await}
}
#[derive(Clone)]
pub struct WebhooksSDKResource{context:SdkContext,pub endpoints:WebhooksEndpointsSDKResource,pub events:WebhooksEventsSDKResource,pub grants:WebhooksGrantsSDKResource,}
impl WebhooksSDKResource{fn new(context:SdkContext)->Self{Self{endpoints:WebhooksEndpointsSDKResource::new(context.clone()),events:WebhooksEventsSDKResource::new(context.clone()),grants:WebhooksGrantsSDKResource::new(context.clone()),context}}
}
#[derive(Clone)]
pub struct WebhooksEndpointsSDKResource{context:SdkContext,}
impl WebhooksEndpointsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,params:WebhookEndpointListParams,options:Option<RequestOptions>)->Result<ListWebhookEndpointsResponse,SdkError>{self.context.call("listWebhookEndpoints",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:WebhookEndpointListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListWebhookEndpointsResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listWebhookEndpoints",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn create(&self,params:WebhookEndpointCreateParams,options:Option<RequestOptions>)->Result<CreateWebhookEndpointResponse,SdkError>{self.context.call("createWebhookEndpoint",vec![],serde_json::to_value(params)?,options).await}
pub async fn update(&self,endpoint_id:&str,params:WebhookEndpointUpdateParams,options:Option<RequestOptions>)->Result<UpdateWebhookEndpointResponse,SdkError>{self.context.call("updateWebhookEndpoint",vec![endpoint_id.to_string()],serde_json::to_value(params)?,options).await}
pub async fn delete(&self,endpoint_id:&str,options:Option<RequestOptions>)->Result<DeleteWebhookEndpointResponse,SdkError>{self.context.call("deleteWebhookEndpoint",vec![endpoint_id.to_string()],json!({}),options).await}
pub async fn rotate_secret(&self,endpoint_id:&str,options:Option<RequestOptions>)->Result<RotateWebhookEndpointSecretResponse,SdkError>{self.context.call("rotateWebhookEndpointSecret",vec![endpoint_id.to_string()],json!({}),options).await}
pub async fn test(&self,endpoint_id:&str,options:Option<RequestOptions>)->Result<TestWebhookEndpointResponse,SdkError>{self.context.call("testWebhookEndpoint",vec![endpoint_id.to_string()],json!({}),options).await}
}
#[derive(Clone)]
pub struct WebhooksEventsSDKResource{context:SdkContext,}
impl WebhooksEventsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,params:WebhookEventListParams,options:Option<RequestOptions>)->Result<ListWebhookEventsResponse,SdkError>{self.context.call("listWebhookEvents",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:WebhookEventListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListWebhookEventsResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listWebhookEvents",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn get(&self,event_id:&str,options:Option<RequestOptions>)->Result<GetWebhookEventResponse,SdkError>{self.context.call("getWebhookEvent",vec![event_id.to_string()],json!({}),options).await}
pub async fn replay(&self,event_id:&str,options:Option<RequestOptions>)->Result<ReplayWebhookEventResponse,SdkError>{self.context.call("replayWebhookEvent",vec![event_id.to_string()],json!({}),options).await}
}
#[derive(Clone)]
pub struct WebhooksGrantsSDKResource{context:SdkContext,}
impl WebhooksGrantsSDKResource{fn new(context:SdkContext)->Self{Self{context}}
pub async fn list(&self,params:WebhookGrantListParams,options:Option<RequestOptions>)->Result<ListWebhookGrantsResponse,SdkError>{self.context.call("listWebhookGrants",vec![],serde_json::to_value(params)?,options).await}
pub fn iterate(&self,params:WebhookGrantListParams,options:Option<RequestOptions>)->futures::stream::BoxStream<'static,Result<ListWebhookGrantsResponseDataItem,SdkError>>{match serde_json::to_value(params){Ok(params)=>self.context.iterate("listWebhookGrants",vec![],params,options),Err(e)=>Box::pin(futures::stream::once(async move{Err(e.into())}))}}
pub async fn save(&self,platform_id:&str,params:WebhookGrantSaveParams,options:Option<RequestOptions>)->Result<SaveWebhookGrantResponse,SdkError>{self.context.call("saveWebhookGrant",vec![platform_id.to_string()],serde_json::to_value(params)?,options).await}
pub async fn revoke(&self,platform_id:&str,options:Option<RequestOptions>)->Result<RevokeWebhookGrantResponse,SdkError>{self.context.call("revokeWebhookGrant",vec![platform_id.to_string()],json!({}),options).await}
}
#[derive(Clone)]
pub struct Affinity{context:SdkContext,pub account:AccountSDKResource,pub api_keys:ApiKeysSDKResource,pub catalog:CatalogSDKResource,pub locations:LocationsSDKResource,pub orders:OrdersSDKResource,pub patients:PatientsSDKResource,pub pharmacies:PharmaciesSDKResource,pub practices:PracticesSDKResource,pub team:TeamSDKResource,pub webhooks:WebhooksSDKResource,}
impl Affinity{pub fn new(api_key:impl Into<String>)->Result<Self,SdkError>{Self::with_options(api_key,SdkOptions::default())}pub fn with_options(api_key:impl Into<String>,options:SdkOptions)->Result<Self,SdkError>{Ok(Self::from_context(SdkContext{transport:Arc::new(SdkTransport::new(api_key.into(),options)?),practice_id:None,invalid:None}))}fn from_context(context:SdkContext)->Self{Self{account:AccountSDKResource::new(context.clone()),api_keys:ApiKeysSDKResource::new(context.clone()),catalog:CatalogSDKResource::new(context.clone()),locations:LocationsSDKResource::new(context.clone()),orders:OrdersSDKResource::new(context.clone()),patients:PatientsSDKResource::new(context.clone()),pharmacies:PharmaciesSDKResource::new(context.clone()),practices:PracticesSDKResource::new(context.clone()),team:TeamSDKResource::new(context.clone()),webhooks:WebhooksSDKResource::new(context.clone()),context}}pub fn for_practice(&self,practice_id:impl Into<String>)->Self{let practice_id=practice_id.into();let mut invalid=self.context.invalid.clone();if practice_id.trim().is_empty(){invalid=Some("practice_id is required".into());}if self.context.practice_id.as_ref().is_some_and(|p|p!=&practice_id){invalid=Some("Conflicting practice ID".into());}Self::from_context(SdkContext{transport:self.context.transport.clone(),practice_id:Some(practice_id),invalid})}}
