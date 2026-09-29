use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CatalogClient {
    pub http_client: HttpClient,
}

impl CatalogClient {
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
    pub async fn list_catalog_items(
        &self,
        request: &ListCatalogItemsQueryRequest,
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

    /// Lists pharmacies available to the authenticated account, including approved invite-only relationships.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_pharmacies(
        &self,
        request: &ListPharmaciesQueryRequest,
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

    /// Returns an array of at most 50 reviewed shipping services eligible for a catalog item, destination, and API mode. destinationState must be a USPS state or territory code. Each option has one temperature; pharmacy catalog summaries list all supported temperatures.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_shipping_options(
        &self,
        catalog_item_id: &str,
        request: &ListShippingOptionsQueryRequest,
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

    /// Requires catalog:read. Returns reviewed SIG presets, guided patterns, quantity constraints and product requirements for a practice and mode. Revisions identify changed defaults. No patient-specific rationale or diagnosis is inferred.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn retrieve_prescribing_options(
        &self,
        catalog_item_id: &str,
        request: &RetrievePrescribingOptionsQueryRequest,
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
