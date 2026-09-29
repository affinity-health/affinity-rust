use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub mod invitations;
pub use invitations::InvitationsClient;
pub mod members;
pub use members::MembersClient;
pub mod prescribers;
pub use prescribers::PrescribersClient;
pub struct TeamClient {
    pub http_client: HttpClient,
    pub invitations: InvitationsClient,
    pub members: MembersClient,
    pub prescribers: PrescribersClient,
}

impl TeamClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            invitations: InvitationsClient::new(config.clone())?,
            members: MembersClient::new(config.clone())?,
            prescribers: PrescribersClient::new(config.clone())?,
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
    pub async fn register(
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

    /// Requires team:read. Returns counts of members, invitations, and prescribers. Use the paginated members, prescribers, and invitations collections for individual records. Team access and clinician credentials are shared between Test and Live.
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
}
