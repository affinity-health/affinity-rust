use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct TestSimulationClient {
    pub http_client: HttpClient,
}

impl TestSimulationClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
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
    pub async fn get(
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
    pub async fn update(
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
}
