use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AddressesClient {
    pub http_client: HttpClient,
}

impl AddressesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list(
        &self,
        practice_id: &str,
        patient_id: &str,
        request: &PatientsAddressesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPatientAddressesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/practices/{}/patients/{}/addresses",
                    practice_id, patient_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("status", request.status.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .serialize("endingBefore", request.ending_before.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns the existing active address for a normalized duplicate. The first address becomes the default. API keys require Idempotency-Key.
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
        patient_id: &str,
        request: &CreatePatientAddressRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePatientAddressResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/practices/{}/patients/{}/addresses",
                    practice_id, patient_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Preserves the address ID and history. Archiving the default selects the oldest remaining active address. Existing orders remain unchanged.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn archive(
        &self,
        practice_id: &str,
        patient_id: &str,
        address_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ArchivePatientAddressResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/practices/{}/patients/{}/addresses/{}",
                    practice_id, patient_id, address_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn update(
        &self,
        practice_id: &str,
        patient_id: &str,
        address_id: &str,
        request: &UpdatePatientAddressRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePatientAddressResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/practices/{}/patients/{}/addresses/{}",
                    practice_id, patient_id, address_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Changes delivery selection for future drafts, without changing patient clinical location or existing signed orders.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn set_default(
        &self,
        practice_id: &str,
        patient_id: &str,
        address_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SetDefaultPatientAddressResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/practices/{}/patients/{}/addresses/{}/default",
                    practice_id, patient_id, address_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
