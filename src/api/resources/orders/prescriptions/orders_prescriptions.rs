use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PrescriptionsClient {
    pub http_client: HttpClient,
}

impl PrescriptionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
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
    pub async fn add(
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
    pub async fn update(
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
}
