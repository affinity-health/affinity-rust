use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LicensesClient {
    pub http_client: HttpClient,
}

impl LicensesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires team:write and an active accepted prescriber account connection in this practice. Adds a license. Expiration is optional, but must be in the future when supplied. An exact repeat returns the existing license; update an existing license with PATCH and its license ID. Licenses are shared across practices and Test/Live. Other licenses stay unchanged.
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
        practice_id: &str,
        prescriber_id: &str,
        request: &CreatePracticeTeamLicenseRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePracticeTeamLicenseResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/practices/{}/team/prescribers/{}/licenses",
                    practice_id, prescriber_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires team:write and an active accepted prescriber account connection in this practice. Correct the state or license number, or set or clear the optional expiresAt value. A supplied expiration must be in the future. Other licenses stay unchanged. Changes apply across practices and Test/Live.
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
        practice_id: &str,
        prescriber_id: &str,
        license_id: &str,
        request: &UpdatePracticeTeamLicenseRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePracticeTeamLicenseResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/practices/{}/team/prescribers/{}/licenses/{}",
                    practice_id, prescriber_id, license_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
