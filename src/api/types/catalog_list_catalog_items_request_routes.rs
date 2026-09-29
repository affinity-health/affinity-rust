pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ListCatalogItemsRequestRoutes {
    ListCatalogItemsRequestRoutesZero(ListCatalogItemsRequestRoutesZero),

    ListCatalogItemsRequestRoutesOneItemList(Vec<ListCatalogItemsRequestRoutesOneItem>),
}

impl ListCatalogItemsRequestRoutes {
    pub fn is_list_catalog_items_request_routes_zero(&self) -> bool {
        matches!(self, Self::ListCatalogItemsRequestRoutesZero(_))
    }

    pub fn is_list_catalog_items_request_routes_one_item_list(&self) -> bool {
        matches!(self, Self::ListCatalogItemsRequestRoutesOneItemList(_))
    }

    pub fn as_list_catalog_items_request_routes_zero(
        &self,
    ) -> Option<&ListCatalogItemsRequestRoutesZero> {
        match self {
            Self::ListCatalogItemsRequestRoutesZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_catalog_items_request_routes_zero(
        self,
    ) -> Option<ListCatalogItemsRequestRoutesZero> {
        match self {
            Self::ListCatalogItemsRequestRoutesZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_catalog_items_request_routes_one_item_list(
        &self,
    ) -> Option<&Vec<ListCatalogItemsRequestRoutesOneItem>> {
        match self {
            Self::ListCatalogItemsRequestRoutesOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_catalog_items_request_routes_one_item_list(
        self,
    ) -> Option<Vec<ListCatalogItemsRequestRoutesOneItem>> {
        match self {
            Self::ListCatalogItemsRequestRoutesOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
