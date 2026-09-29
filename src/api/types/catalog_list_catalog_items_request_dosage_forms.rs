pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ListCatalogItemsRequestDosageForms {
    ListCatalogItemsRequestDosageFormsZero(ListCatalogItemsRequestDosageFormsZero),

    ListCatalogItemsRequestDosageFormsOneItemList(Vec<ListCatalogItemsRequestDosageFormsOneItem>),
}

impl ListCatalogItemsRequestDosageForms {
    pub fn is_list_catalog_items_request_dosage_forms_zero(&self) -> bool {
        matches!(self, Self::ListCatalogItemsRequestDosageFormsZero(_))
    }

    pub fn is_list_catalog_items_request_dosage_forms_one_item_list(&self) -> bool {
        matches!(self, Self::ListCatalogItemsRequestDosageFormsOneItemList(_))
    }

    pub fn as_list_catalog_items_request_dosage_forms_zero(
        &self,
    ) -> Option<&ListCatalogItemsRequestDosageFormsZero> {
        match self {
            Self::ListCatalogItemsRequestDosageFormsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_catalog_items_request_dosage_forms_zero(
        self,
    ) -> Option<ListCatalogItemsRequestDosageFormsZero> {
        match self {
            Self::ListCatalogItemsRequestDosageFormsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_catalog_items_request_dosage_forms_one_item_list(
        &self,
    ) -> Option<&Vec<ListCatalogItemsRequestDosageFormsOneItem>> {
        match self {
            Self::ListCatalogItemsRequestDosageFormsOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_catalog_items_request_dosage_forms_one_item_list(
        self,
    ) -> Option<Vec<ListCatalogItemsRequestDosageFormsOneItem>> {
        match self {
            Self::ListCatalogItemsRequestDosageFormsOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
