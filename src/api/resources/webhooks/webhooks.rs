use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
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
    pub async fn list_webhook_endpoints(
        &self,
        request: &ListWebhookEndpointsQueryRequest,
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
    pub async fn create_webhook_endpoint(
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

    pub async fn delete_webhook_endpoint(
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
    pub async fn update_webhook_endpoint(
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

    pub async fn rotate_webhook_endpoint_secret(
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

    pub async fn test_webhook_endpoint(
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

    pub async fn list_webhook_events(
        &self,
        request: &ListWebhookEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhookEventsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/webhook-events",
                None,
                QueryBuilder::new()
                    .serialize("endingBefore", request.ending_before.clone())
                    .int("limit", request.limit.clone())
                    .serialize("status", request.status.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_webhook_event(
        &self,
        event_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetWebhookEventResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/webhook-events/{}", event_id),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn replay_webhook_event(
        &self,
        event_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ReplayWebhookEventResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/webhook-events/{}/replay", event_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Requires webhooks:read on the owning practice or pharmacy key. Lists platform webhook grants in the key's mode. Platforms cannot list or grant themselves delegated access.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_webhook_grants(
        &self,
        request: &ListWebhookGrantsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhookGrantsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/webhook-grants",
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .serialize("endingBefore", request.ending_before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Requires webhooks:write on the owning practice or pharmacy key and Idempotency-Key. Grants or replaces a platform's webhook permissions in this mode. A practice must already be connected to that platform. The grant does not give the platform access to other API resources.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn save_webhook_grant(
        &self,
        platform_id: &str,
        request: &SaveWebhookGrantRequest,
        options: Option<RequestOptions>,
    ) -> Result<SaveWebhookGrantResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/webhook-grants/{}", platform_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires webhooks:write on the owning practice or pharmacy key and Idempotency-Key. Removes platform webhook access in this mode. Existing endpoints remain owned by the practice or pharmacy and continue operating.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn revoke_webhook_grant(
        &self,
        platform_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RevokeWebhookGrantResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/webhook-grants/{}", platform_id),
                None,
                None,
                options,
            )
            .await
    }
}
