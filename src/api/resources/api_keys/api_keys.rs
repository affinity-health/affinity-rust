use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ApiKeysClient {
    pub http_client: HttpClient,
}

impl ApiKeysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a practice API key for a connected practice. Requires a platform key with service_keys:write and every requested scope. The practice key uses the platform key's Test or Live mode and cannot outlive it. Requires Idempotency-Key for safe retries; the secret is returned in the encrypted replay response for 24 hours.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_platform_practice_api_key(
        &self,
        practice_id: &str,
        request: &CreatePlatformPracticeApiKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePlatformPracticeApiKeyResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/practices/{}/api-keys", practice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the subject, mode, and scopes for the API key.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_api_access(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GetApiAccessResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/auth/access", None, None, options)
            .await
    }
}
