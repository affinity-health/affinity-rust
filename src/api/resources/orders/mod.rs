use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod exceptions;
pub use exceptions::ExceptionsClient;
pub mod events;
pub use events::EventsClient;
pub mod test_simulation;
pub use test_simulation::TestSimulationClient;
pub mod prescriptions;
pub use prescriptions::PrescriptionsClient;
pub mod batches;
pub use batches::BatchesClient;
pub struct OrdersClient {
    pub http_client: HttpClient,
    pub exceptions: ExceptionsClient,
    pub events: EventsClient,
    pub test_simulation: TestSimulationClient,
    pub prescriptions: PrescriptionsClient,
    pub batches: BatchesClient,
}

impl OrdersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            exceptions: ExceptionsClient::new(config.clone())?,
            events: EventsClient::new(config.clone())?,
            test_simulation: TestSimulationClient::new(config.clone())?,
            prescriptions: PrescriptionsClient::new(config.clone())?,
            batches: BatchesClient::new(config.clone())?,
        })
    }

    pub async fn list(
        &self,
        request: &OrdersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOrdersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/orders",
                None,
                QueryBuilder::new()
                    .structured_query("query", request.query.clone())
                    .serialize("externalOrderId", request.external_order_id.clone())
                    .serialize("createdAfter", request.created_after.clone())
                    .serialize("createdBefore", request.created_before.clone())
                    .serialize("endingBefore", request.ending_before.clone())
                    .int("limit", request.limit.clone())
                    .serialize("orderId", request.order_id.clone())
                    .serialize("patientId", request.patient_id.clone())
                    .serialize("patientExternalId", request.patient_external_id.clone())
                    .serialize("practiceId", request.practice_id.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .serialize("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates one unsigned order with 1–20 prescriptions for one patient in one practice. Supply patientId or patient; inline patient creation requires patients:write. Prescriber is optional: select by npi, provider id, or integration-scoped externalId, or leave the draft unassigned until signing. First-use prescriber registration requires team:write. Legacy userId is supported but cannot be combined with prescriber. Idempotency-Key is required.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreateOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/orders",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn get(
        &self,
        order_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/orders/{}", order_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Requests cancellation. HTTP 200 means the request was handled; check cancellation.status for confirmed, pending, partial, or failed. Only confirmed means the entire order is cancelled. Shipment possession makes a fulfillment cancellation too late.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cancel(
        &self,
        order_id: &str,
        request: &CancelOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<CancelOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/orders/{}/cancel", order_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires orders:write and catalog:read. Supply exactly one of patientId, patientExternalId, or inline patient details. External-ID lookup additionally requires patients:read; inline details require patients:write. Resolves defaults and explicit edits for 1–20 prescriptions. Reuses stored patient details when identifiers match; otherwise previews inline details without creating a patient. Complete previews contain an orders.create input. Does not create records, reserve prices, sign, charge or transmit. No idempotency key is required. Creation and signing recheck current requirements.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn preview(
        &self,
        request: &PreviewOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<PreviewOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/order-previews",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires orders:sign, Idempotency-Key, signatureAttestation, and expectedRevision from the reviewed order. Existing integrations may send expectedVersions instead; supply exactly one. A stale revision returns 409 and requires renewed clinician review. Select prescriber by npi, provider id, or integration-scoped externalId, or inherit the draft's prescriber. First-use registration requires team:write. Actor headers are optional audit metadata with prescriber; legacy userId requires matching clinician actor headers. Signing does not submit to a pharmacy.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn sign(
        &self,
        order_id: &str,
        request: &SignOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<SignOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/orders/{}/sign", order_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires orders:sign, Idempotency-Key, signatureAttestation, and expectedRevision from the reviewed order. Existing integrations may send expectedVersions instead; supply exactly one. A stale revision returns 409 and requires renewed clinician review. Select prescriber by npi, provider id, or externalId, or inherit the draft's prescriber. First-use registration requires team:write. Actor headers are optional with prescriber; legacy userId requires matching clinician actor headers. Signs the complete order, then attempts each submission. Signing remains committed if submission fails. Replay the same key after an uncertain response; retry reported submission failures through Submit order with a new key. Submitted means queued, not pharmacy acceptance.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn sign_and_submit(
        &self,
        order_id: &str,
        request: &SignAndSubmitOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<SignAndSubmitOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/orders/{}/sign-and-submit", order_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires orders:sign and Idempotency-Key. Queues signed prescriptions after rechecking authorization, signature integrity, billing, and fulfillment eligibility. Track pharmacy acceptance through order reads and webhooks. After a partial failure, retry submission with a new idempotency key; already queued prescriptions are not duplicated.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn submit(
        &self,
        order_id: &str,
        request: &SubmitOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<SubmitOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/orders/{}/submit", order_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires orders:sign and Idempotency-Key. Select a prescriber or inherit the draft's prescriber. Legacy userId requires matching clinician actor headers. Supply expectedRevision from the reviewed order, or expectedVersions for existing integrations. Permanently rejects the complete unsigned order after checking its revision.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn reject(
        &self,
        order_id: &str,
        request: &RejectOrderRequest,
        options: Option<RequestOptions>,
    ) -> Result<RejectOrderResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/orders/{}/rejection", order_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
