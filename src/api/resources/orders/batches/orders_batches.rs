use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct BatchesClient {
    pub http_client: HttpClient,
}

impl BatchesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
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
    pub async fn create(
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
