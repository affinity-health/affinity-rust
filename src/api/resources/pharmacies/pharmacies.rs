use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PharmaciesClient {
    pub http_client: HttpClient,
}

impl PharmaciesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists pharmacies available to the authenticated account, including approved invite-only relationships.
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
        request: &PharmaciesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPharmaciesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/pharmacies",
                None,
                QueryBuilder::new()
                    .serialize("endingBefore", request.ending_before.clone())
                    .int("limit", request.limit.clone())
                    .serialize("orgId", request.org_id.clone())
                    .serialize("pharmacyId", request.pharmacy_id.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("shipsToState", request.ships_to_state.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .build(),
                options,
            )
            .await
    }
}
