use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct OrdersClient {
    pub http_client: HttpClient,
}

impl OrdersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_orders(
        &self,
        request: &ListOrdersQueryRequest,
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
    pub async fn create_order(
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

    pub async fn get_order(
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
    pub async fn cancel_order(
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

    /// Acknowledge, retry, contact, or resolve an order exception in the credential's Test/Live mode. assign_to_me requires a signed-in dashboard user; API keys receive 400 and may use acknowledge instead. Actor headers do not create a dashboard assignee.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn act_on_order_exception(
        &self,
        order_id: &str,
        exception_id: &str,
        request: &ActOnOrderExceptionRequest,
        options: Option<RequestOptions>,
    ) -> Result<ActOnOrderExceptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/orders/{}/exceptions/{}/actions", order_id, exception_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn list_order_events(
        &self,
        order_id: &str,
        request: &ListOrderEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOrderEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/orders/{}/events", order_id),
                None,
                QueryBuilder::new()
                    .serialize("endingBefore", request.ending_before.clone())
                    .int("limit", request.limit.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Requires orders:write. Available only in Test mode.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_order_test_simulation(
        &self,
        order_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetOrderTestSimulationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/orders/{}/test-simulation", order_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Requires orders:write and Idempotency-Key. Configure before submission or queue a valid pharmacy event in manual mode. Events use normal order history and Test webhooks. Live requests are rejected.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_order_test_simulation(
        &self,
        order_id: &str,
        request: &UpdateOrderTestSimulationRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateOrderTestSimulationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/orders/{}/test-simulation", order_id),
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
    pub async fn preview_order(
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
    pub async fn sign_order(
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
    pub async fn sign_and_submit_order(
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
    pub async fn submit_order(
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
    pub async fn reject_order(
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

    /// Requires orders:write, Idempotency-Key and expectedRevision from the order being edited. Existing integrations may send expectedVersions instead; supply exactly one. Adds a complete prescription to an unsigned Order and returns all new versions. Omitted actor context defaults to the authenticated service account as a system actor. Patient and prescriber attribution stay fixed. Signed orders cannot be amended through this endpoint. Signing and submission require orders:sign through their separate endpoints.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_order_prescription(
        &self,
        order_id: &str,
        request: &AddOrderPrescriptionRequest,
        options: Option<RequestOptions>,
    ) -> Result<AddOrderPrescriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/orders/{}/prescriptions", order_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires orders:write, Idempotency-Key and expectedRevision from the order being edited. Existing integrations may send expectedVersions instead; supply exactly one. Replaces one prescription with complete medication instructions and returns all new versions. Omitted actor context defaults to the authenticated service account as a system actor. Patient and prescriber attribution stay fixed. Signed orders cannot be amended through this endpoint. Signing and submission require orders:sign through their separate endpoints.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_order_prescription(
        &self,
        order_id: &str,
        prescription_id: &str,
        request: &UpdateOrderPrescriptionRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateOrderPrescriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/orders/{}/prescriptions/{}", order_id, prescription_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates 1–20 orders for distinct patients in one practice, each with 1–20 prescriptions. Each accepts patientId or inline patient details. Orders and newly created patients commit atomically; any failure saves none. Requires orders:write and Idempotency-Key; inline patients also require patients:write. Omitted actor context defaults to the authenticated service account as a system actor. Sign and submit each resulting order separately using orders:sign.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_order_batch(
        &self,
        request: &CreateOrderBatchRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateOrderBatchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/order-batches",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
