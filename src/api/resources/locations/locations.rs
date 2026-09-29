use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LocationsClient {
    pub http_client: HttpClient,
}

impl LocationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires locations:read on a practice key or an authorized platform key. Lists active and archived locations by name, with cursor pagination. Use status to filter. Location records are shared between Test and Live for the same practice.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_practice_locations(
        &self,
        practice_id: &str,
        request: &ListPracticeLocationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPracticeLocationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/practices/{}/locations", practice_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .serialize("endingBefore", request.ending_before.clone())
                    .serialize("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Requires locations:write and Idempotency-Key for API keys. Creates an active location with a unique name in this practice. Locations are shared between Test and Live. Use the returned ID for Team location access.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_practice_location(
        &self,
        practice_id: &str,
        request: &CreatePracticeLocationRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePracticeLocationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/practices/{}/locations", practice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires locations:read. Returns one active or archived location in the authorized practice.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_practice_location(
        &self,
        practice_id: &str,
        location_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPracticeLocationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/practices/{}/locations/{}", practice_id, location_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Requires locations:write and Idempotency-Key for API keys. Updates only supplied fields; null clears optional contact and address fields. Archived locations cannot be updated. Changes apply to both Test and Live.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_practice_location(
        &self,
        practice_id: &str,
        location_id: &str,
        request: &UpdatePracticeLocationRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePracticeLocationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/practices/{}/locations/{}", practice_id, location_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires locations:write and Idempotency-Key for API keys. Retains the location and historical associations. Archived locations cannot receive new Team assignments. Repeating archive returns the archived location. Changes apply to both Test and Live.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn archive_practice_location(
        &self,
        practice_id: &str,
        location_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ArchivePracticeLocationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/practices/{}/locations/{}/archive",
                    practice_id, location_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
