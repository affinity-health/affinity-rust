use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TeamClient {
    pub http_client: HttpClient,
}

impl TeamClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires team:write and Idempotency-Key. Registers a practice member without an invitation. Test requires synthetic .test emails and Affinity Test NPIs. Live requires approved integration and practice access. Identity attestation records the integration's assertion; it does not verify login email or clinical credentials. Existing memberships and verified provider records are preserved. Use the returned user ID for orders and signing.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn register_user(
        &self,
        practice_id: &str,
        request: &RegisterUserRequest,
        options: Option<RequestOptions>,
    ) -> Result<RegisterUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/practices/{}/users", practice_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
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
    pub async fn list_practice_team_invitations(
        &self,
        practice_id: &str,
        request: &ListPracticeTeamInvitationsQueryRequest,
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
    pub async fn invite_practice_team_person(
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

    /// Requires team:read. Returns counts of members, invitations, and prescribers. Use the paginated members, prescribers, and invitations collections for individual records. Team access and clinician credentials are shared between Test and Live.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_practice_team(
        &self,
        practice_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPracticeTeamResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/practices/{}/team", practice_id),
                None,
                None,
                options,
            )
            .await
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
    pub async fn list_practice_team_members(
        &self,
        practice_id: &str,
        request: &ListPracticeTeamMembersQueryRequest,
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
    pub async fn list_practice_team_prescribers(
        &self,
        practice_id: &str,
        request: &ListPracticeTeamPrescribersQueryRequest,
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

    /// Requires team:read. Returns current account membership, roles, location access, and prescriber connection. The member ID identifies practice access; it is not the integration user ID used by orders.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_practice_team_member(
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
    pub async fn update_practice_team_member(
        &self,
        practice_id: &str,
        member_id: &str,
        request: &UpdatePracticeTeamMemberRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePracticeTeamMemberResponse, ApiError> {
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

    /// Requires team:read. Returns the clinical profile and submitted licenses, including license IDs. This is setup information, not a signing authorization.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_practice_team_prescriber(
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
    pub async fn update_practice_team_prescriber(
        &self,
        practice_id: &str,
        prescriber_id: &str,
        request: &UpdatePracticeTeamPrescriberRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePracticeTeamPrescriberResponse, ApiError> {
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

    /// Requires team:write and an active accepted prescriber account connection in this practice. Adds a license. Expiration is optional, but must be in the future when supplied. An exact repeat returns the existing license; update an existing license with PATCH and its license ID. Licenses are shared across practices and Test/Live. Other licenses stay unchanged.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_practice_team_license(
        &self,
        practice_id: &str,
        prescriber_id: &str,
        request: &CreatePracticeTeamLicenseRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePracticeTeamLicenseResponse, ApiError> {
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
    pub async fn update_practice_team_license(
        &self,
        practice_id: &str,
        prescriber_id: &str,
        license_id: &str,
        request: &UpdatePracticeTeamLicenseRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePracticeTeamLicenseResponse, ApiError> {
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

    /// Requires team:read. Returns invitation status and current onboarding state for your integration. An accepted invitation can still have disabled membership or pending clinical review. Invitation tokens are never returned.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_practice_team_invitation(
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
    pub async fn revoke_practice_team_invitation(
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
    pub async fn resend_practice_team_invitation(
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
