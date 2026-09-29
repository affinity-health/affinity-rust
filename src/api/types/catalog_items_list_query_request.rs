pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CatalogItemsListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<ListItemsRequestView>,
    #[serde(rename = "relatedToCatalogItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_to_catalog_item_id: Option<String>,
    #[serde(rename = "catalogKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_kind: Option<ListItemsRequestCatalogKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<ListItemsRequestSort>,
    #[serde(rename = "catalogItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<ListItemsRequestAvailability>,
    #[serde(rename = "pharmacyIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_ids: Option<ListItemsRequestPharmacyIds>,
    #[serde(rename = "dosageForms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dosage_forms: Option<ListItemsRequestDosageForms>,
    #[serde(rename = "endingBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    #[serde(rename = "hideControlledSubstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_controlled_substances: Option<bool>,
    #[serde(rename = "hideUnpriced")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_unpriced: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(rename = "practiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub practice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement: Option<ListItemsRequestRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<ListItemsRequestRoutes>,
    #[serde(rename = "startingAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

impl CatalogItemsListQueryRequest {
    pub fn builder() -> CatalogItemsListQueryRequestBuilder {
        <CatalogItemsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CatalogItemsListQueryRequestBuilder {
    view: Option<ListItemsRequestView>,
    related_to_catalog_item_id: Option<String>,
    catalog_kind: Option<ListItemsRequestCatalogKind>,
    sort: Option<ListItemsRequestSort>,
    catalog_item_id: Option<String>,
    availability: Option<ListItemsRequestAvailability>,
    pharmacy_ids: Option<ListItemsRequestPharmacyIds>,
    dosage_forms: Option<ListItemsRequestDosageForms>,
    ending_before: Option<String>,
    hide_controlled_substances: Option<bool>,
    hide_unpriced: Option<bool>,
    limit: Option<i64>,
    org_id: Option<String>,
    practice_id: Option<String>,
    query: Option<String>,
    requirement: Option<ListItemsRequestRequirement>,
    routes: Option<ListItemsRequestRoutes>,
    starting_after: Option<String>,
}

impl CatalogItemsListQueryRequestBuilder {
    pub fn view(mut self, value: ListItemsRequestView) -> Self {
        self.view = Some(value);
        self
    }

    pub fn related_to_catalog_item_id(mut self, value: impl Into<String>) -> Self {
        self.related_to_catalog_item_id = Some(value.into());
        self
    }

    pub fn catalog_kind(mut self, value: ListItemsRequestCatalogKind) -> Self {
        self.catalog_kind = Some(value);
        self
    }

    pub fn sort(mut self, value: ListItemsRequestSort) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn catalog_item_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_item_id = Some(value.into());
        self
    }

    pub fn availability(mut self, value: ListItemsRequestAvailability) -> Self {
        self.availability = Some(value);
        self
    }

    pub fn pharmacy_ids(mut self, value: ListItemsRequestPharmacyIds) -> Self {
        self.pharmacy_ids = Some(value);
        self
    }

    pub fn dosage_forms(mut self, value: ListItemsRequestDosageForms) -> Self {
        self.dosage_forms = Some(value);
        self
    }

    pub fn ending_before(mut self, value: impl Into<String>) -> Self {
        self.ending_before = Some(value.into());
        self
    }

    pub fn hide_controlled_substances(mut self, value: bool) -> Self {
        self.hide_controlled_substances = Some(value);
        self
    }

    pub fn hide_unpriced(mut self, value: bool) -> Self {
        self.hide_unpriced = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn requirement(mut self, value: ListItemsRequestRequirement) -> Self {
        self.requirement = Some(value);
        self
    }

    pub fn routes(mut self, value: ListItemsRequestRoutes) -> Self {
        self.routes = Some(value);
        self
    }

    pub fn starting_after(mut self, value: impl Into<String>) -> Self {
        self.starting_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CatalogItemsListQueryRequest`].
    pub fn build(self) -> Result<CatalogItemsListQueryRequest, BuildError> {
        Ok(CatalogItemsListQueryRequest {
            view: self.view,
            related_to_catalog_item_id: self.related_to_catalog_item_id,
            catalog_kind: self.catalog_kind,
            sort: self.sort,
            catalog_item_id: self.catalog_item_id,
            availability: self.availability,
            pharmacy_ids: self.pharmacy_ids,
            dosage_forms: self.dosage_forms,
            ending_before: self.ending_before,
            hide_controlled_substances: self.hide_controlled_substances,
            hide_unpriced: self.hide_unpriced,
            limit: self.limit,
            org_id: self.org_id,
            practice_id: self.practice_id,
            query: self.query,
            requirement: self.requirement,
            routes: self.routes,
            starting_after: self.starting_after,
        })
    }
}
