use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod addresses;
pub use addresses::AddressesClient;
pub mod allergies;
pub use allergies::AllergiesClient;
pub struct PatientsClient {
    pub http_client: HttpClient,
    pub addresses: AddressesClient,
    pub allergies: AllergiesClient,
}

impl PatientsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            addresses: AddressesClient::new(config.clone())?,
            allergies: AllergiesClient::new(config.clone())?,
        })
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
    pub async fn list(
        &self,
        practice_id: &str,
        request: &PatientsListQueryRequest,
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
    pub async fn create(
        &self,
        practice_id: &str,
        request: &CreatePatientRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePatientResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
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
    pub async fn get(
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
    pub async fn delete(
        &self,
        practice_id: &str,
        patient_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeletePatientResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
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
    pub async fn update(
        &self,
        practice_id: &str,
        patient_id: &str,
        request: &UpdatePatientRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePatientResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
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
}
