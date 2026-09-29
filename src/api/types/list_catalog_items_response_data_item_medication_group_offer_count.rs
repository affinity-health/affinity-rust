pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListCatalogItemsResponseDataItemMedicationGroupOfferCount {
    Double(f64),

    ListCatalogItemsResponseDataItemMedicationGroupOfferCountOne(
        ListCatalogItemsResponseDataItemMedicationGroupOfferCountOne,
    ),
}

impl ListCatalogItemsResponseDataItemMedicationGroupOfferCount {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_catalog_items_response_data_item_medication_group_offer_count_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ListCatalogItemsResponseDataItemMedicationGroupOfferCountOne(_)
        )
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_catalog_items_response_data_item_medication_group_offer_count_one(
        &self,
    ) -> Option<&ListCatalogItemsResponseDataItemMedicationGroupOfferCountOne> {
        match self {
            Self::ListCatalogItemsResponseDataItemMedicationGroupOfferCountOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_list_catalog_items_response_data_item_medication_group_offer_count_one(
        self,
    ) -> Option<ListCatalogItemsResponseDataItemMedicationGroupOfferCountOne> {
        match self {
            Self::ListCatalogItemsResponseDataItemMedicationGroupOfferCountOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for ListCatalogItemsResponseDataItemMedicationGroupOfferCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListCatalogItemsResponseDataItemMedicationGroupOfferCountOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
