use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ItemsClient {
    pub http_client: HttpClient,
}

impl ItemsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists catalog items for the authenticated account and mode. Use view=medications for priced prescription groups with offer counts, pharmacy counts, and strengths; the default view=offers returns individual offers. Use relatedToCatalogItemId to find offers for the same medication and route. When practiceId is supplied, a practice price overrides the platform price and missing overrides inherit the platform price.
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
        request: &CatalogItemsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCatalogItemsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/catalog/items",
                None,
                QueryBuilder::new()
                    .serialize("view", request.view.clone())
                    .serialize(
                        "relatedToCatalogItemId",
                        request.related_to_catalog_item_id.clone(),
                    )
                    .serialize("catalogKind", request.catalog_kind.clone())
                    .serialize("sort", request.sort.clone())
                    .serialize("catalogItemId", request.catalog_item_id.clone())
                    .serialize("availability", request.availability.clone())
                    .serialize("pharmacyIds", request.pharmacy_ids.clone())
                    .serialize("dosageForms", request.dosage_forms.clone())
                    .serialize("endingBefore", request.ending_before.clone())
                    .bool(
                        "hideControlledSubstances",
                        request.hide_controlled_substances.clone(),
                    )
                    .bool("hideUnpriced", request.hide_unpriced.clone())
                    .int("limit", request.limit.clone())
                    .serialize("orgId", request.org_id.clone())
                    .serialize("practiceId", request.practice_id.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("requirement", request.requirement.clone())
                    .serialize("routes", request.routes.clone())
                    .serialize("startingAfter", request.starting_after.clone())
                    .build(),
                options,
            )
            .await
    }
}
