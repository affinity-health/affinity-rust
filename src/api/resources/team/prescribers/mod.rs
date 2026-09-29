use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod licenses;
pub use licenses::LicensesClient;
pub struct PrescribersClient {
    pub http_client: HttpClient,
    pub licenses: LicensesClient,
}

impl PrescribersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            licenses: LicensesClient::new(config.clone())?,
        })
    }

    /// Requires team:read. Filter practice prescribers by name, NPI, state, and practice status. Records include submitted licenses and their IDs. Signing authority also requires an active account connection, Live practice access, and prescription eligibility.
    ///
    /// # Arguments
    ///
    /// * `state` - Match a submitted license jurisdiction. This does not establish signing eligibility.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        practice_id: &str,
        request: &TeamPrescribersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPracticeTeamPrescribersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/practices/{}/team/prescribers", practice_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .serialize("endingBefore", request.ending_before.clone())
                    .serialize("search", request.search.clone())
                    .serialize("npi", request.npi.clone())
                    .serialize("state", request.state.clone())
                    .serialize("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Requires team:read. Returns the clinical profile and submitted licenses, including license IDs. This is setup information, not a signing authorization.
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
        prescriber_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPracticeTeamPrescriberResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/practices/{}/team/prescribers/{}",
                    practice_id, prescriber_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Requires team:write. Set practiceStatus to inactive to remove prescribing access in this practice, or active to restore an existing association. This does not create membership or signing authority. Practice status applies to Test and Live. Shared identity and license edits require Affinity support.
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
        request: &UpdatePracticeTeamPrescriberRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePracticeTeamPrescriberResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/practices/{}/team/prescribers/{}",
                    practice_id, prescriber_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
