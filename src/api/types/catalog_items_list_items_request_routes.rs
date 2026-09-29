pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ListItemsRequestRoutes {
    ListItemsRequestRoutesZero(ListItemsRequestRoutesZero),

    ListItemsRequestRoutesOneItemList(Vec<ListItemsRequestRoutesOneItem>),
}

impl ListItemsRequestRoutes {
    pub fn is_list_items_request_routes_zero(&self) -> bool {
        matches!(self, Self::ListItemsRequestRoutesZero(_))
    }

    pub fn is_list_items_request_routes_one_item_list(&self) -> bool {
        matches!(self, Self::ListItemsRequestRoutesOneItemList(_))
    }

    pub fn as_list_items_request_routes_zero(&self) -> Option<&ListItemsRequestRoutesZero> {
        match self {
            Self::ListItemsRequestRoutesZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_items_request_routes_zero(self) -> Option<ListItemsRequestRoutesZero> {
        match self {
            Self::ListItemsRequestRoutesZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_items_request_routes_one_item_list(
        &self,
    ) -> Option<&Vec<ListItemsRequestRoutesOneItem>> {
        match self {
            Self::ListItemsRequestRoutesOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_items_request_routes_one_item_list(
        self,
    ) -> Option<Vec<ListItemsRequestRoutesOneItem>> {
        match self {
            Self::ListItemsRequestRoutesOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
