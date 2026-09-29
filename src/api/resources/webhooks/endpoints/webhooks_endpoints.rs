use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EndpointsClient {
    pub http_client: HttpClient,
}

impl EndpointsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires webhooks:read. Returns endpoints owned by the key organization, or the organization selected with X-Affinity-Organization-Id. Platform delegation requires a webhook grant in the key's mode.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &WebhooksEndpointsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhookEndpointsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/webhook-endpoints",
                None,
                QueryBuilder::new()
                    .string("endingBefore", request.ending_before.clone())
                    .int("limit", request.limit.clone())
                    .string("startingAfter", request.starting_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Requires webhooks:write and Idempotency-Key. Defaults to the API key organization. A platform can select a practice or pharmacy owner with X-Affinity-Organization-Id and an explicit webhook grant. For platform-owned endpoints, practiceIds narrows delivery to selected connected practices. An empty filter receives all otherwise-authorized events.
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
        request: &CreateWebhookEndpointRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateWebhookEndpointResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/webhook-endpoints",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn delete(
        &self,
        endpoint_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteWebhookEndpointResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/webhook-endpoints/{}", endpoint_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Requires webhooks:write and Idempotency-Key. Updates an endpoint in the selected organization and mode. Omitted practiceIds preserves the filter; an empty array removes the practice filter. Subscription changes apply to newly generated events.
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
        endpoint_id: &str,
        request: &UpdateWebhookEndpointRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateWebhookEndpointResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/webhook-endpoints/{}", endpoint_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    pub async fn rotate_secret(
        &self,
        endpoint_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RotateWebhookEndpointSecretResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/webhook-endpoints/{}/rotate-secret", endpoint_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn test(
        &self,
        endpoint_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TestWebhookEndpointResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/webhook-endpoints/{}/test", endpoint_id),
                None,
                None,
                options,
            )
            .await
    }
}
