use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PracticesClient {
    pub http_client: HttpClient,
}

impl PracticesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the practices that belong to the platform. The default Affinity-Version is 2026-09-28.
    ///
    /// # Arguments
    ///
    /// * `search` - Case-insensitive search by practice name or external ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_practices(
        &self,
        request: &ListPracticesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPracticesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/practices",
                None,
                QueryBuilder::new()
                    .serialize("search", request.search.clone())
                    .serialize("endingBefore", request.ending_before.clone())
                    .int("limit", request.limit.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a practice owned by the platform. Set liveEnabled to true to enable Live access at creation with an approved platform and a Live request. Defaults to false. Requires practices:write. Send Idempotency-Key when you retry the same request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_practice(
        &self,
        request: &CreatePracticeRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePracticeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/practices",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns one practice that belongs to the platform.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_practice(
        &self,
        practice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPracticeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/practices/{}", practice_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates one practice owned by the platform. Set liveEnabled to true or false to control Live access with an approved platform and a Live request. Affinity Admin decisions take precedence. Requires practices:write. Send Idempotency-Key when you retry the same request.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_practice(
        &self,
        practice_id: &str,
        request: &UpdatePracticeRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePracticeResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/practices/{}", practice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
