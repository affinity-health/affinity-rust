use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SellingPricesClient {
    pub http_client: HttpClient,
}

impl SellingPricesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Requires selling_prices:read. Reads the Affinity-managed purchase-price override inherited by this platform's practices unless Affinity sets a practice override. Use the practice-scoped catalog for effective practice prices and presentation-price when an Affinity default may be absent. Platforms cannot edit purchase prices.
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
        request: &CatalogSellingPricesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlatformPublicApiSellingPricesReadSellingPriceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/catalog/items/{}/selling-price", catalog_item_id),
                None,
                QueryBuilder::new()
                    .serialize("practiceId", request.practice_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
