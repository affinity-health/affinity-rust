use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PatientsClient {
    pub http_client: HttpClient,
}

impl PatientsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_patient_addresses(
        &self,
        practice_id: &str,
        patient_id: &str,
        request: &ListPatientAddressesQueryRequest,
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
    pub async fn create_patient_address(
        &self,
        practice_id: &str,
        patient_id: &str,
        request: &CreatePatientAddressRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePatientAddressResponse, ApiError> {
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
    pub async fn archive_patient_address(
        &self,
        practice_id: &str,
        patient_id: &str,
        address_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ArchivePatientAddressResponse, ApiError> {
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

    pub async fn update_patient_address(
        &self,
        practice_id: &str,
        patient_id: &str,
        address_id: &str,
        request: &UpdatePatientAddressRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePatientAddressResponse, ApiError> {
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
    pub async fn set_default_patient_address(
        &self,
        practice_id: &str,
        patient_id: &str,
        address_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SetDefaultPatientAddressResponse, ApiError> {
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

    /// Lists patients in one practice and mode. Use externalId for an exact match in the calling integration's namespace. Use externalIdentitySource with externalIdentityValue to search an explicit alias. Identity matching is case-sensitive after trimming whitespace. Other filters also apply.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_patients(
        &self,
        practice_id: &str,
        request: &ListPatientsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPatientsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/practices/{}/patients", practice_id),
                None,
                QueryBuilder::new()
                    .serialize("endingBefore", request.ending_before.clone())
                    .serialize("externalId", request.external_id.clone())
                    .serialize(
                        "externalIdentitySource",
                        request.external_identity_source.clone(),
                    )
                    .serialize(
                        "externalIdentityValue",
                        request.external_identity_value.clone(),
                    )
                    .serialize("gender", request.gender.clone())
                    .serialize("lastOrderAfter", request.last_order_after.clone())
                    .serialize("lastOrderBefore", request.last_order_before.clone())
                    .int("limit", request.limit.clone())
                    .serialize("program", request.program.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .serialize("states", request.states.clone())
                    .serialize("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a patient or resolves a matching externalId or external identity within this practice and mode. externalId belongs to the calling integration; externalIdentities holds aliases from other systems. Resolution preserves existing demographics; use PATCH to update them. Conflicting identifiers return 409. Email never merges patients. API keys require Idempotency-Key.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_patient(
        &self,
        practice_id: &str,
        request: &CreatePatientRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePatientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/practices/{}/patients", practice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns one patient in the authorized practice and mode.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_patient(
        &self,
        practice_id: &str,
        patient_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPatientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/practices/{}/patients/{}", practice_id, patient_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Requires patients:write and Idempotency-Key for API keys. Permanently deletes a patient with no order history. Any order history returns 409; use Update patient with status archived instead. Available to practice keys and authorized platform keys. Reusing the same idempotency key returns the original deletion result.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_patient(
        &self,
        practice_id: &str,
        patient_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeletePatientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/practices/{}/patients/{}", practice_id, patient_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a patient in the current practice and mode. Omitted fields remain unchanged; null clears an optional field. externalId updates the calling integration's identifier. externalIdentities replaces its explicit aliases. Identifiers cannot be reassigned from another patient. API keys require Idempotency-Key.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_patient(
        &self,
        practice_id: &str,
        patient_id: &str,
        request: &UpdatePatientRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePatientResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/practices/{}/patients/{}", practice_id, patient_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
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
    pub async fn get_patient_allergies(
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
    pub async fn replace_patient_allergies(
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
