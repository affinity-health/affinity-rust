pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ListItemsRequestDosageForms {
    ListItemsRequestDosageFormsZero(ListItemsRequestDosageFormsZero),

    ListItemsRequestDosageFormsOneItemList(Vec<ListItemsRequestDosageFormsOneItem>),
}

impl ListItemsRequestDosageForms {
    pub fn is_list_items_request_dosage_forms_zero(&self) -> bool {
        matches!(self, Self::ListItemsRequestDosageFormsZero(_))
    }

    pub fn is_list_items_request_dosage_forms_one_item_list(&self) -> bool {
        matches!(self, Self::ListItemsRequestDosageFormsOneItemList(_))
    }

    pub fn as_list_items_request_dosage_forms_zero(
        &self,
    ) -> Option<&ListItemsRequestDosageFormsZero> {
        match self {
            Self::ListItemsRequestDosageFormsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_items_request_dosage_forms_zero(
        self,
    ) -> Option<ListItemsRequestDosageFormsZero> {
        match self {
            Self::ListItemsRequestDosageFormsZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_items_request_dosage_forms_one_item_list(
        &self,
    ) -> Option<&Vec<ListItemsRequestDosageFormsOneItem>> {
        match self {
            Self::ListItemsRequestDosageFormsOneItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_items_request_dosage_forms_one_item_list(
        self,
    ) -> Option<Vec<ListItemsRequestDosageFormsOneItem>> {
        match self {
            Self::ListItemsRequestDosageFormsOneItemList(value) => Some(value),
            _ => None,
        }
    }
}
