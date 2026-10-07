use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PresentationPricesClient {
    pub http_client: HttpClient,
}

impl PresentationPricesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires selling_prices:read. Reads the Affinity default and the Affinity-managed purchase-price override for this platform, shared by every pharmacy. Practices inherit it unless Affinity sets a practice override; use the practice-scoped catalog for effective practice prices. Missing defaults are null. Platforms cannot edit purchase prices.
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
        request: &CatalogPresentationPricesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlatformPublicApiSellingPricesReadPresentationPriceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/catalog/items/{}/presentation-price", catalog_item_id),
                None,
                QueryBuilder::new()
                    .serialize("practiceId", request.practice_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
