use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AccountClient {
    pub http_client: HttpClient,
}

impl AccountClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the platform organization, request livemode, and effective access. API keys report scopes and the service_key role; dashboard sessions report membership permissions. operatingMode describes organization Live access, not the credential's Test/Live mode.
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
        request: &AccountGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetAccountResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/account",
                None,
                QueryBuilder::new()
                    .serialize("orgId", request.org_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
