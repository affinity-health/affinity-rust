use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct InvitationsClient {
    pub http_client: HttpClient,
}

impl InvitationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires team:read. Lists practice invitations, including invitations sent in Clinic. Filter by pending, expired, accepted, declined, or revoked status, exact email, or your integration externalId. Only your integration and API key mode can see its external identity and onboarding state. Follow person.nextActions after invitation acceptance.
    ///
    /// # Arguments
    ///
    /// * `external_id` - Match this integration's external identity in the API key's mode.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        practice_id: &str,
        request: &TeamInvitationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPracticeTeamInvitationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/practices/{}/team/invitations", practice_id),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .serialize("endingBefore", request.ending_before.clone())
                    .serialize("status", request.status.clone())
                    .serialize("email", request.email.clone())
                    .serialize("externalId", request.external_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Requires team:write on the practice key or its platform key. Use roles to combine administrator, prescriber, clinical_staff, billing, or developer presets. Ownership uses the protected owner designation. The singular role field remains available for single-role assignments. Creates a real organization invitation and optional prescriber setup. The recipient must accept with their Affinity account. Repeating the same external identity retries pending invitation delivery. Accepted invitations do not change existing access. Team membership is shared between Test and Live; the external identity is mode-scoped. Keys cannot accept invitations. Headless registration and signing use separate endpoints.
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
        request: &InvitePracticeTeamPersonRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvitePracticeTeamPersonResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/practices/{}/team/invitations", practice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Requires team:read. Returns invitation status and current onboarding state for your integration. An accepted invitation can still have disabled membership or pending clinical review. Invitation tokens are never returned.
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
        invitation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPracticeTeamInvitationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/practices/{}/team/invitations/{}",
                    practice_id, invitation_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Requires team:write. Revokes a pending or expired invitation and its pending prescriber account connection. Repeating the revoke returns the revoked invitation. Accepted invitations return 409; disable the member instead. Retains invitation history.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn revoke(
        &self,
        practice_id: &str,
        invitation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RevokePracticeTeamInvitationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/practices/{}/team/invitations/{}",
                    practice_id, invitation_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Requires team:write. Resends a pending or expired invitation with the same ID, recipient, roles, and locations. The previous link stops working and the new link expires in seven days. Accepted and revoked invitations return 409. A 502 means the invitation was saved but email delivery could not be confirmed; retry this operation.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn resend(
        &self,
        practice_id: &str,
        invitation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ResendPracticeTeamInvitationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/practices/{}/team/invitations/{}/resend",
                    practice_id, invitation_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
