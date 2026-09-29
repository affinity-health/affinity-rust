use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PrescribingOptionsClient {
    pub http_client: HttpClient,
}

impl PrescribingOptionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires catalog:read. Returns reviewed SIG presets, guided patterns, quantity constraints and product requirements for a practice and mode. Revisions identify changed defaults. No patient-specific rationale or diagnosis is inferred.
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
        catalog_item_id: &str,
        request: &CatalogPrescribingOptionsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RetrievePrescribingOptionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/catalog/items/{}/prescribing-options", catalog_item_id),
                None,
                QueryBuilder::new()
                    .string("practiceId", request.practice_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
