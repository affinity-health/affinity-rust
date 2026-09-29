use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MembersClient {
    pub http_client: HttpClient,
}

impl MembersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires team:read. Search the roster by name or email, and filter by role or membership status. Includes members invited in Clinic, location access, and account-specific prescriber connections. Memberships are shared between Test and Live.
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
        request: &TeamMembersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPracticeTeamMembersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/practices/{}/team/members", practice_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .serialize("endingBefore", request.ending_before.clone())
                    .serialize("search", request.search.clone())
                    .serialize("role", request.role.clone())
                    .serialize("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Requires team:read. Returns current account membership, roles, location access, and prescriber connection. The member ID identifies practice access; it is not the integration user ID used by orders.
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
        member_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPracticeTeamMemberResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/practices/{}/team/members/{}", practice_id, member_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Requires team:write. Supply role, status, or locationIds; omitted values stay unchanged. A role replaces existing roles. Disable access with status disabled. An empty locationIds array grants all practice locations. Ownership changes require an active practice owner using a personal API key; service keys manage non-owner memberships. The final active owner cannot be removed. Changes apply to both Test and Live. Sign-in email and account security remain account settings.
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
        member_id: &str,
        request: &UpdatePracticeTeamMemberRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePracticeTeamMemberResponse, ApiError> {
        let mut options = options.unwrap_or_default();
        if !options.additional_headers.keys().any(|key| key.eq_ignore_ascii_case("Idempotency-Key")) {
            options.additional_headers.insert("Idempotency-Key".into(), uuid::Uuid::new_v4().to_string());
        }
        let options = Some(options); // affinity-sdk-auto-key
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/practices/{}/team/members/{}", practice_id, member_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
