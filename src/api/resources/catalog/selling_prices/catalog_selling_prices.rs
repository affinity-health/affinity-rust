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

    /// Requires selling_prices:read. Omit practiceId for the platform default, or supply a managed practice. A null amount inherits the next applicable price. Amounts use the catalog pricing basis, in USD cents. purchaseAmountCents is the platform's Affinity purchase price for that same basis. requiresReview indicates changed product pricing terms, not a below-purchase-price discount.
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

    /// Requires selling_prices:write. Sets a platform default or managed practice override in the current Test/Live mode. Send baseVersion from Read selling price. Null removes the override. Prices use the catalog pricing basis. Intentional discounts below purchaseAmountCents are allowed; compare these amounts to warn about selling below your Affinity purchase price. This does not change the platform's Affinity purchase price or collect practice payments.
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
        catalog_item_id: &str,
        request: &PlatformPublicApiSellingPricesUpdateSellingPriceRequest,
        options: Option<RequestOptions>,
    ) -> Result<PlatformPublicApiSellingPricesUpdateSellingPriceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("v1/catalog/items/{}/selling-price", catalog_item_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
