use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ShippingOptionsClient {
    pub http_client: HttpClient,
}

impl ShippingOptionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns an array of at most 50 reviewed shipping services eligible for a catalog item, destination, and API mode. destinationState must be a USPS state or territory code. Each option has one temperature; pharmacy catalog summaries list all supported temperatures.
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
        catalog_item_id: &str,
        request: &CatalogShippingOptionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListShippingOptionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/catalog/items/{}/shipping-options", catalog_item_id),
                None,
                QueryBuilder::new()
                    .string("destinationState", request.destination_state.clone())
                    .serialize("destinationType", request.destination_type.clone())
                    .build(),
                options,
            )
            .await
    }
}
