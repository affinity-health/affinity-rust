use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AllergiesClient {
    pub http_client: HttpClient,
}

impl AllergiesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the patient's structured allergy entries and review status. A not_reviewed status is not a no-known-allergies assertion and blocks clinical review and signing.
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
        practice_id: &str,
        patient_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPatientAllergiesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/practices/{}/patients/{}/allergies",
                    practice_id, patient_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Replaces the patient's structured allergy record. Sending no_known is the explicit no-known-allergies acknowledgement; recorded requires at least one entry. Idempotency-Key is required.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn replace(
        &self,
        practice_id: &str,
        patient_id: &str,
        request: &ReplacePatientAllergiesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReplacePatientAllergiesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/practices/{}/patients/{}/allergies",
                    practice_id, patient_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
