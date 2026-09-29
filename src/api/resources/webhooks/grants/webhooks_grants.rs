use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct GrantsClient {
    pub http_client: HttpClient,
}

impl GrantsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
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
    pub async fn list(
        &self,
        request: &WebhooksGrantsListQueryRequest,
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
    pub async fn save(
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
    pub async fn revoke(
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
