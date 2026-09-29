use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ExceptionsClient {
    pub http_client: HttpClient,
}

impl ExceptionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
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
    pub async fn act(
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
}
